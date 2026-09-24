use std::{
    env,
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

mod api;
mod auth;
mod boxun_actions;
mod crud;
mod export;
mod files;
mod seed;
mod system;
mod weather;

use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::{Mutex, oneshot};
use topcoat::{
    Result,
    context::Cx,
    router::{
        Body, Method, Methods, RouteFn, Router, RouterBuilderDiscoverExt,
        content::Json,
        request::{headers, uri},
        response::{IntoResponse, Response},
        route, to_bytes,
    },
};

#[cfg(unix)]
use tokio::net::UnixListener;

type RouteFuture<'a> = Pin<Box<dyn Future<Output = Result<Response>> + Send + 'a>>;

#[derive(Clone)]
struct AppState {
    database: Arc<tokio_postgres::Client>,
    ingress_token: String,
    jwt_secret: String,
    broker_socket: Option<String>,
    channel: Arc<Mutex<Vec<ChannelWaiter>>>,
}

#[derive(Deserialize)]
struct HostConfiguration {
    abi_version: u32,
    tenant_id: String,
    database_url: Option<String>,
    ingress_token: String,
    #[serde(default)]
    broker_socket: Option<String>,
    #[serde(default)]
    jwt_secret: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(port) = env::var("AIO_PLUGIN_PORT") {
        // Topcoat 使用 PORT；AIO 的隔离进程只注入 AIO_PLUGIN_PORT。
        unsafe { env::set_var("PORT", port) };
    }
    let state = connect_state().await?;
    eprintln!("资料员Agent 数据库就绪，开始初始化");
    seed::initialize(&state).await.map_err(|error| {
        eprintln!("资料员Agent 初始化失败: {error}");
        topcoat::Error::from(std::io::Error::other(error))
    })?;
    eprintln!("资料员Agent 初始化完成，开始监听");
    serve(router(state)).await.map_err(topcoat::Error::from)
}

async fn serve(router: Router) -> std::io::Result<()> {
    if let Ok(path) = env::var("AIO_PLUGIN_SOCKET") {
        #[cfg(unix)]
        {
            let path = std::path::PathBuf::from(path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            match std::fs::remove_file(&path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
            let listener = UnixListener::bind(path)?;
            return topcoat::serve(listener, router).await;
        }
        #[cfg(not(unix))]
        {
            let _ = path;
        }
    }
    topcoat::start(router).await
}

fn router(state: AppState) -> Router {
    Router::builder()
        .app_context(state)
        .route(RouteFn::new(
            Methods::Any,
            "/admin-api/{*path}",
            admin_api_route,
        ))
        .discover()
        .build()
}

#[route(GET "/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}

#[route(GET "/aio/describe")]
async fn describe() -> Result<Json<Value>> {
    Ok(Json(json!({
        "label": "资料员Agent",
        "pages": [{
            "id": "documentation-agent",
            "label": "资料员Agent",
            "entry": "index.html",
            "scene": ["workspace", "工作空间"],
            "menu_path": ["资料员Agent"],
            "permission": null,
            "surface": "workspace"
        }]
    })))
}

async fn admin_api(cx: &Cx, body: Body) -> Result<Response> {
    let state = topcoat::context::app_context::<AppState>(cx);
    let method = topcoat::router::request::method(cx).clone();
    let uri = uri(cx);
    let request_headers = headers(cx);
    if request_headers
        .get("x-aio-token")
        .and_then(|value| value.to_str().ok())
        != Some(state.ingress_token.as_str())
    {
        return (
            topcoat::router::StatusCode::UNAUTHORIZED,
            Json(json!({"code":401,"msg":"插件入口票据无效","data":null})),
        )
            .into_response(cx);
    }
    let body = to_bytes(body, usize::MAX).await.unwrap_or_default();
    let payload = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body).unwrap_or(Value::Null)
    };
    let path = uri.path().trim_start_matches("/admin-api");
    if method == Method::POST && path.trim_start_matches('/') == "system/auth/login" {
        return match auth::login(state, payload).await {
            Ok(data) => Ok(Json(json!({"code":0,"msg":"","data":data})).into_response(cx)?),
            Err(message) => {
                Ok(Json(json!({"code":401,"msg":message,"data":null})).into_response(cx)?)
            }
        };
    }
    if method == Method::POST && path.trim_start_matches('/') == "system/auth/refresh-token" {
        let refresh_token = uri
            .query()
            .and_then(|query| {
                url::form_urlencoded::parse(query.as_bytes()).find(|(key, _)| key == "refreshToken")
            })
            .map(|(_, value)| value.into_owned());
        return match auth::refresh(state, refresh_token.as_deref()).await {
            Ok(data) => Ok(Json(json!({"code":0,"msg":"","data":data})).into_response(cx)?),
            Err(message) => {
                Ok(Json(json!({"code":401,"msg":message,"data":null})).into_response(cx)?)
            }
        };
    }
    if method == Method::POST && path.trim_start_matches('/') == "system/auth/host-login" {
        let user_id = request_headers
            .get("x-aio-user-id")
            .and_then(|value| value.to_str().ok());
        let tenant_id = request_headers
            .get("x-aio-tenant-id")
            .and_then(|value| value.to_str().ok());
        let Some(user_id) = user_id else {
            return Json(json!({"code":401,"msg":"缺少宿主用户","data":null})).into_response(cx);
        };
        return match auth::host_login(state, user_id, tenant_id.unwrap_or("default")).await {
            Ok(data) => Json(json!({"code":0,"msg":"","data":data})).into_response(cx),
            Err(message) => Json(json!({"code":401,"msg":message,"data":null})).into_response(cx),
        };
    }
    let authorization = request_headers
        .get("authorization")
        .and_then(|value| value.to_str().ok());
    let access_query = uri.query().and_then(|query| {
        url::form_urlencoded::parse(query.as_bytes())
            .find(|(key, _)| key == "access_token")
            .map(|(_, value)| value.into_owned())
    });
    let authenticated =
        match auth::authenticate(state, authorization, access_query.as_deref()).await {
            Ok(authenticated) => authenticated,
            Err(message) => {
                return Json(json!({"code":401,"msg":message,"data":null})).into_response(cx);
            }
        };
    if method == Method::GET && path.trim_start_matches('/') == "system/auth/get-permission-info" {
        return match auth::permission_info(state, &authenticated).await {
            Ok(data) => Ok(Json(json!({"code":0,"msg":"","data":data})).into_response(cx)?),
            Err(message) => {
                Ok(Json(json!({"code":500,"msg":message,"data":null})).into_response(cx)?)
            }
        };
    }
    if method == Method::POST && path.trim_start_matches('/') == "system/auth/logout" {
        return Json(json!({"code":0,"msg":"","data":true})).into_response(cx);
    }
    dispatch(state, &authenticated, &method, path, uri.query(), payload).await
}

fn admin_api_route(cx: &Cx, body: Body) -> RouteFuture<'_> {
    Box::pin(admin_api(cx, body))
}

async fn dispatch(
    state: &AppState,
    authenticated: &auth::Authenticated,
    method: &Method,
    path: &str,
    query: Option<&str>,
    body: Value,
) -> std::result::Result<Response, topcoat::Error> {
    let (route, operation) = path
        .trim_start_matches('/')
        .rsplit_once('/')
        .ok_or_else(|| topcoat::Error::from(std::io::Error::other("业务路由格式无效")))?;
    let route = route.strip_prefix("boxun/").unwrap_or(route);
    let query = query
        .map(|value| {
            url::form_urlencoded::parse(value.as_bytes())
                .into_owned()
                .fold(
                    std::collections::BTreeMap::<String, Vec<String>>::new(),
                    |mut values, (key, value)| {
                        values.entry(key).or_default().push(value);
                        values
                    },
                )
        })
        .unwrap_or_default();
    let result = if operation == "export-excel"
        && (crud::entity(route).is_some() || route.starts_with("system/"))
    {
        if crud::entity(route).is_some() {
            export::export(state, route, &query).await
        } else {
            export::export_system(state, route, &query).await
        }
    } else if matches!(
        route,
        "homepage"
            | "project-info"
            | "project-company"
            | "commission-order"
            | "block-retention-ledger"
            | "raw-material-ledger"
            | "commercial-concrete-ledger"
            | "template-management"
            | "historical-weather"
            | "account-payment"
            | "channel"
    ) && is_action(route, operation)
    {
        boxun_actions::handle(
            state,
            authenticated,
            method.as_str(),
            route,
            operation,
            &query,
            body,
        )
        .await
    } else if route.starts_with("system/")
        || route == "infra/config"
        || route.starts_with("infra/config/")
        || route == "infra/file"
        || route.starts_with("infra/file/")
        || route == "infra/file-config"
        || route.starts_with("infra/file-config/")
        || route.starts_with("infra/api-")
    {
        system::handle(
            state,
            authenticated,
            method.as_str(),
            route,
            operation,
            &query,
            body,
        )
        .await
    } else {
        crud::handle(state, method.as_str(), route, operation, &query, body).await
    };
    let data = match result {
        Ok(data) => data,
        Err(message) => {
            eprintln!("Boxun CRUD 失败: {message}");
            return Json(json!({"code":500,"msg":message,"data":null}))
                .into_response(&Cx::default());
        }
    };
    Json(json!({"code":0,"msg":"","data":data})).into_response(&Cx::default())
}

fn is_action(route: &str, operation: &str) -> bool {
    matches!(
        (route, operation),
        ("homepage", _)
            | ("project-info", "get-all-project")
            | ("project-info", "get-all-sj-project")
            | ("project-info", "query-project-company-by-main-id")
            | ("project-info", "current-project-companies")
            | ("project-company", "simple-list")
            | ("commission-order", "issue-an-order")
            | (
                "commission-order",
                "query-commission-order-sample-by-main-id"
            )
            | ("commission-order", "download-attachment")
            | ("commission-order", "batch-download-generated")
            | ("delegation-order-context", "load-tree-data")
            | ("witness-record", "generate-from-commission-order")
            | ("bystander-record", "generate-from-commission-order")
            | (
                "concrete-construction-record",
                "generate-from-commission-order"
            )
            | ("block-retention-ledger", "entrust-the-test-block")
            | ("block-retention-ledger", "evaluate-strength")
            | ("block-retention-ledger", "generate-multi-kit")
            | ("block-retention-ledger", "gen-and-send-email")
            | ("raw-material-ledger", "to-entrust-raw-materials")
            | ("raw-material-ledger", "add-batch")
            | ("raw-material-ledger", "raw-material-pull-down")
            | ("raw-material-ledger", "data-generation")
            | (
                "raw-material-ledger",
                "generate-various-raw-material-records"
            )
            | ("commercial-concrete-ledger", "add-batch")
            | ("commercial-concrete-ledger", "specimen-group-numbers")
            | ("commercial-concrete-ledger", "generate-various-records")
            | ("commercial-concrete-ledger", "delete-various-records")
            | (
                "commercial-concrete-ledger",
                "manually-generate-various-records"
            )
            | ("commercial-concrete-ledger", "mock-sh")
            | (
                "commercial-concrete-ledger",
                "generate-test-block-retention-account"
            )
            | (
                "commercial-concrete-ledger",
                "batch-generation-of-concrete-construction-records"
            )
            | (
                "commercial-concrete-ledger",
                "generate-side-station-records"
            )
            | ("commercial-concrete-ledger", "delete-by-project-id")
            | ("template-management", "inherit-templates")
            | ("template-management", "field-mapping")
            | ("historical-weather", "areas")
            | ("historical-weather", "area-name")
            | ("historical-weather", "page-view")
            | ("historical-weather", "view-specified-weather")
            | ("historical-weather", "sync-weather-year")
            | ("historical-weather", "sync-weather-month")
            | ("historical-weather", "sync-recent-years")
            | ("historical-weather", "test-cloud-message")
            | ("block-retention-ledger", "gen-ttj-wtd")
            | ("block-retention-ledger", "gen-cm-wtd")
            | ("block-retention-ledger", "download-various-packages")
            | ("account-payment", "account-price-list")
            | ("channel", "push")
            | ("channel", "poll")
    )
}

async fn poll_channel(waiters: &Mutex<Vec<ChannelWaiter>>) -> Result<Value, String> {
    let (sender, mut receiver) = oneshot::channel();
    static NEXT_WAITER_ID: AtomicUsize = AtomicUsize::new(1);
    let id = NEXT_WAITER_ID.fetch_add(1, Ordering::Relaxed);
    waiters.lock().await.push(ChannelWaiter { id, sender });
    let result = match tokio::time::timeout(Duration::from_secs(30), &mut receiver).await {
        Ok(Ok(message)) => Ok(Value::String(message)),
        Ok(Err(_)) => Err("长轮询通道已关闭".into()),
        Err(_) => Ok(Value::Null),
    };
    if matches!(&result, Ok(Value::Null) | Err(_)) {
        waiters.lock().await.retain(|waiter| waiter.id != id);
    }
    result
}

async fn push_channel(waiters: &Mutex<Vec<ChannelWaiter>>, message: String) -> usize {
    let targets = std::mem::take(&mut *waiters.lock().await);
    let delivered = targets.len();
    for target in targets {
        let _ = target.sender.send(message.clone());
    }
    delivered
}

struct ChannelWaiter {
    id: usize,
    sender: oneshot::Sender<String>,
}

async fn connect_state() -> Result<AppState> {
    let host = read_host_configuration()?;
    let database_url = host
        .database_url
        .filter(|value| !value.trim().is_empty())
        .or_else(|| env::var("BOXUN_DATABASE_URL").ok())
        .ok_or_else(|| topcoat::Error::from(std::io::Error::other("宿主未授权数据库")))?;
    let database_url = normalize_database_url(&database_url)?;
    eprintln!("资料员Agent 开始连接宿主数据库");
    let (database, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .map_err(|error| {
            eprintln!("资料员Agent 数据库连接失败: {error:?}");
            topcoat::Error::from(std::io::Error::other(error.to_string()))
        })?;
    eprintln!("资料员Agent 数据库连接成功");
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Boxun Topcoat 数据库连接中断: {error}");
        }
    });
    Ok(AppState {
        database: Arc::new(database),
        ingress_token: host.ingress_token,
        jwt_secret: host
            .jwt_secret
            .filter(|value| value.len() >= 32)
            .or_else(|| env::var("BOXUN_JWT_SECRET").ok())
            .unwrap_or_else(|| "boxun-topcoat-local-jwt-secret-change-me".into()),
        broker_socket: host.broker_socket,
        channel: Arc::new(Mutex::new(Vec::new())),
    })
}

fn normalize_database_url(value: &str) -> Result<String> {
    let mut url = url::Url::parse(value)
        .map_err(|error| topcoat::Error::from(std::io::Error::other(error.to_string())))?;
    let socket_host = url
        .query_pairs()
        .find(|(key, _)| key == "host")
        .map(|(_, value)| value.into_owned());
    let socket_only = socket_host
        .as_deref()
        .is_some_and(|value| value.starts_with('/'));
    let supported = [
        "sslmode",
        "application_name",
        "connect_timeout",
        "options",
        "target_session_attrs",
        "host",
        "port",
        "user",
        "password",
        "dbname",
    ];
    let mut retained = url
        .query_pairs()
        .filter(|(key, _)| supported.contains(&key.as_ref()))
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect::<Vec<_>>();

    retained.retain(|(key, _)| key != "sslmode");
    retained.push(("sslmode".to_owned(), "disable".to_owned()));

    url.set_query(None);
    if !retained.is_empty() {
        let mut query = url.query_pairs_mut();
        for (key, value) in &retained {
            query.append_pair(key, value);
        }
    }

    if socket_only {
        let user = url.username();
        let password = url.password().unwrap_or_default();
        let credentials = if user.is_empty() {
            String::new()
        } else if password.is_empty() {
            user.to_owned()
        } else {
            format!("{user}:{password}@")
        };
        let query = url.query().unwrap_or_default();
        return Ok(format!(
            "{}://{}{}?{}",
            url.scheme(),
            credentials,
            url.path(),
            query
        ));
    }
    Ok(url.to_string())
}

fn read_host_configuration() -> Result<HostConfiguration> {
    if let Ok(path) = env::var("AIO_PLUGIN_CONFIG") {
        let bytes = std::fs::read(path)
            .map_err(|error| topcoat::Error::from(std::io::Error::other(error.to_string())))?;
        let host: HostConfiguration = serde_json::from_slice(&bytes)?;
        if host.abi_version != 2
            || host.tenant_id.trim().is_empty()
            || host.ingress_token.len() < 32
        {
            return Err(topcoat::Error::from(std::io::Error::other("宿主绑定无效")));
        }
        return Ok(host);
    }
    Ok(HostConfiguration {
        abi_version: 2,
        tenant_id: env::var("BOXUN_TENANT").unwrap_or_else(|_| "local-development".into()),
        database_url: env::var("BOXUN_DATABASE_URL").ok(),
        ingress_token: env::var("BOXUN_TOKEN")
            .unwrap_or_else(|_| "local-development-token-000000000000".into()),
        broker_socket: env::var("BOXUN_BROKER_SOCKET").ok(),
        jwt_secret: env::var("BOXUN_JWT_SECRET").ok(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn channel_broadcasts_to_waiting_pollers() {
        let waiters = Arc::new(Mutex::new(Vec::<ChannelWaiter>::new()));
        let waiter = tokio::spawn({
            let waiters = waiters.clone();
            async move { poll_channel(&waiters).await }
        });
        for _ in 0..100 {
            if !waiters.lock().await.is_empty() {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(waiters.lock().await.len(), 1, "poll 必须先登记等待者");
        assert_eq!(push_channel(&waiters, "hello".into()).await, 1);
        assert_eq!(
            waiter
                .await
                .expect("poll 任务必须完成")
                .expect("poll 必须成功"),
            Value::String("hello".into())
        );
    }

    #[tokio::test]
    async fn channel_reports_zero_without_waiters() {
        let waiters = Mutex::new(Vec::<ChannelWaiter>::new());
        assert_eq!(push_channel(&waiters, "hello".into()).await, 0);
    }

    #[test]
    fn preserves_process_database_socket_and_ignores_sqlx_only_options() {
        let normalized = normalize_database_url(
            "postgres://user:pass@localhost:5432/aio_plugin_components?sslmode=prefer&statement-cache-capacity=100&host=%2Fdatabase",
        )
        .expect("process 数据库 URL");
        assert!(
            normalized.contains("host=%2Fdatabase")
                && !normalized.contains("statement-cache-capacity")
        );
        let config: tokio_postgres::Config = normalized.parse().expect("tokio-postgres 配置");
        assert_eq!(
            config.get_hosts(),
            &[tokio_postgres::config::Host::Unix("/database".into())]
        );
    }
}
