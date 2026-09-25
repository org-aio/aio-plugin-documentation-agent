use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, OnceLock};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio_postgres::types::ToSql;

use crate::AppState;

const ACCESS_SECONDS: i64 = 86_400;
const REFRESH_SECONDS: i64 = 2_592_000;
const HOST_SESSION_REUSE_SECONDS: i64 = 300;

static HOST_SESSION_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
static HOST_SESSIONS: OnceLock<Mutex<HashMap<String, CachedHostSession>>> = OnceLock::new();

struct CachedHostSession {
    expires_at: i64,
    payload: Arc<Value>,
}

fn validation() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_aud = false;
    validation
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    typ: String,
    sid: String,
    iat: i64,
    exp: i64,
    user_id: i64,
    tenant_id: i64,
    roles: Vec<String>,
    permissions: Vec<String>,
}

pub(crate) struct Authenticated {
    pub user_id: i64,
    #[expect(dead_code, reason = "保留 JWT 租户上下文供后续租户过滤使用")]
    pub tenant_id: i64,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

pub(crate) async fn host_login(
    state: &AppState,
    external_user_id: &str,
    external_tenant_id: &str,
) -> Result<Value, String> {
    let _guard = HOST_SESSION_LOCK
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await;
    let cache_key = format!("{external_tenant_id}\0{external_user_id}");
    let now = chrono::Utc::now().timestamp();
    let sessions = HOST_SESSIONS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut sessions) = sessions.lock() {
        sessions.retain(|_, session| session.expires_at > now);
        if let Some(session) = sessions.get(&cache_key) {
            return Ok((*session.payload).clone());
        }
    }
    let existing = state
        .database
        .query_opt(
            "SELECT id,tenant_id,status FROM system_users WHERE external_tenant_id=$1 AND external_user_id=$2 ORDER BY id LIMIT 1",
            &[&external_tenant_id, &external_user_id],
        )
        .await
        .map_err(db_error)?;
    let (user_id, tenant_id, status): (i64, i64, i32) = if let Some(row) = existing {
        let user_id = row.get("id");
        state
            .database
            .execute(
                "UPDATE system_users SET tenant_id=1 WHERE id=$1",
                &[&user_id],
            )
            .await
            .map_err(db_error)?;
        (user_id, 1, row.get("status"))
    } else {
        let user_id = state
            .database
            .query_one(
                "WITH next_user AS (SELECT COALESCE(MAX(id),0)+1 AS id FROM system_users) INSERT INTO system_users (create_time,id,tenant_id,status,username,password,nickname,realname,external_user_id,external_tenant_id,dept_id,email,mobile,sex) SELECT CURRENT_TIMESTAMP,next_user.id,1,0,$1,NULL,$2,$2,$1,$3,100,'','','1' FROM next_user RETURNING id",
                &[&external_user_id, &external_user_id, &external_tenant_id],
            )
            .await
            .map_err(db_error)?
            .get(0);
        (user_id, 1, 0)
    };
    if status != 0 {
        return Err("宿主用户已禁用".into());
    }
    state
        .database
        .execute(
            "INSERT INTO system_user_role (user_id,role_id) VALUES ($1,1) ON CONFLICT DO NOTHING",
            &[&user_id],
        )
        .await
        .map_err(db_error)?;
    let payload = token_payload(state, user_id, tenant_id).await?;
    if let Ok(mut sessions) = sessions.lock() {
        sessions.insert(
            cache_key,
            CachedHostSession {
                expires_at: now + HOST_SESSION_REUSE_SECONDS,
                payload: Arc::new(payload.clone()),
            },
        );
    }
    Ok(payload)
}

pub(crate) async fn login(state: &AppState, body: Value) -> Result<Value, String> {
    let username = body
        .get("username")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "登录账号不能为空".to_owned())?;
    let password = body
        .get("password")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "密码不能为空".to_owned())?;
    let row = state
        .database
        .query_opt(
            "SELECT id,tenant_id,username,password,status FROM system_users WHERE username=$1 ORDER BY id LIMIT 1",
            &[&username],
        )
        .await
        .map_err(db_error)?
        .ok_or_else(|| "登录失败，账号密码不正确".to_owned())?;
    let hash: Option<String> = row.get("password");
    let status: i32 = row.get("status");
    if status != 0 {
        return Err("登录失败，账号被禁用".into());
    }
    if !hash
        .as_deref()
        .is_some_and(|hash| bcrypt::verify(password, hash).unwrap_or(false))
    {
        return Err("登录失败，账号密码不正确".into());
    }
    let user_id: i64 = row.get("id");
    let tenant_id: i64 = row.get("tenant_id");
    token_payload(state, user_id, tenant_id).await
}

async fn token_payload(state: &AppState, user_id: i64, tenant_id: i64) -> Result<Value, String> {
    let (roles, permissions) = access(state, user_id, tenant_id).await?;
    let now = chrono::Utc::now().timestamp();
    let session_id = uuid::Uuid::new_v4().to_string();
    let access_token = encode_token(
        state,
        &session_id,
        now,
        now + ACCESS_SECONDS,
        user_id,
        tenant_id,
        &roles,
        &permissions,
        "access",
    )?;
    let refresh_token = encode_token(
        state,
        &session_id,
        now,
        now + REFRESH_SECONDS,
        user_id,
        tenant_id,
        &roles,
        &permissions,
        "refresh",
    )?;
    Ok(json!({
        "userId": user_id,
        "accessToken": access_token,
        "refreshToken": refresh_token,
        "expiresTime": chrono::DateTime::from_timestamp(now + ACCESS_SECONDS, 0)
            .map(|value| value.naive_utc().to_string())
    }))
}

pub(crate) async fn authenticate(
    state: &AppState,
    authorization: Option<&str>,
    access_token: Option<&str>,
) -> Result<Authenticated, String> {
    let token = authorization
        .and_then(|value| value.strip_prefix("Bearer "))
        .or_else(|| access_token.filter(|value| !value.is_empty()))
        .ok_or_else(|| "未登录".to_owned())?;
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &validation(),
    )
    .map_err(|_| "访问令牌无效".to_owned())?
    .claims;
    if claims.typ != "access" {
        return Err("访问令牌无效".into());
    }
    Ok(Authenticated {
        user_id: claims.user_id,
        tenant_id: claims.tenant_id,
        roles: claims.roles,
        permissions: claims.permissions,
    })
}

pub(crate) async fn refresh(
    state: &AppState,
    refresh_token: Option<&str>,
) -> Result<Value, String> {
    let token = refresh_token
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "刷新令牌不能为空".to_owned())?;
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &validation(),
    )
    .map_err(|_| "刷新令牌无效".to_owned())?
    .claims;
    if claims.typ != "refresh" {
        return Err("刷新令牌无效".into());
    }
    let now = chrono::Utc::now().timestamp();
    let access_token = encode_token(
        state,
        &claims.sid,
        now,
        now + ACCESS_SECONDS,
        claims.user_id,
        claims.tenant_id,
        &claims.roles,
        &claims.permissions,
        "access",
    )?;
    let refresh_token = encode_token(
        state,
        &claims.sid,
        now,
        now + REFRESH_SECONDS,
        claims.user_id,
        claims.tenant_id,
        &claims.roles,
        &claims.permissions,
        "refresh",
    )?;
    Ok(json!({
        "accessToken": access_token,
        "refreshToken": refresh_token,
        "expiresTime": chrono::DateTime::from_timestamp(now + ACCESS_SECONDS, 0)
            .map(|value| value.naive_utc().to_string())
    }))
}

pub(crate) async fn permission_info(
    state: &AppState,
    user: &Authenticated,
) -> Result<Value, String> {
    let row = state
        .database
        .query_one(
            "SELECT id,username,nickname,tenant_id,email,avatar FROM system_users WHERE id=$1",
            &[&user.user_id],
        )
        .await
        .map_err(db_error)?;
    let menus = menu_tree(state, user).await?;
    Ok(json!({
        "user": {
            "id": row.get::<_, i64>("id"),
            "username": row.get::<_, String>("username"),
            "nickname": row.get::<_, Option<String>>("nickname").unwrap_or_default(),
            "tenantId": row.get::<_, Option<i64>>("tenant_id"),
            "email": row.get::<_, Option<String>>("email"),
            "avatar": row.get::<_, Option<String>>("avatar").unwrap_or_default(),
            "depts": [],
            "deptIds": []
        },
        "roles": user.roles,
        "permissions": user.permissions,
        "menus": menus
    }))
}

async fn access(
    state: &AppState,
    user_id: i64,
    tenant_id: i64,
) -> Result<(Vec<String>, Vec<String>), String> {
    let rows = state
        .database
        .query(
            "SELECT r.code FROM system_user_role ur JOIN system_role r ON r.id=ur.role_id WHERE ur.user_id=$1 AND (r.tenant_id IS NULL OR r.tenant_id=$2)",
            &[&user_id, &tenant_id],
        )
        .await
        .map_err(db_error)?;
    let roles = rows
        .into_iter()
        .filter_map(|row| row.get::<_, Option<String>>("code"))
        .collect::<Vec<_>>();
    let permissions = if roles.iter().any(|role| role == "super_admin") {
        vec!["*:*:*".to_owned()]
    } else {
        let rows = state
            .database
            .query(
                "SELECT DISTINCT m.permission FROM system_user_role ur JOIN role_menu_mapping rm ON rm.role_id=ur.role_id JOIN system_menu m ON m.id=rm.menu_id WHERE ur.user_id=$1 AND m.status=0 AND m.permission IS NOT NULL AND m.permission <> ''",
                &[&user_id],
            )
            .await
            .map_err(db_error)?;
        rows.into_iter()
            .filter_map(|row| row.get::<_, Option<String>>("permission"))
            .flat_map(|value| {
                value
                    .lines()
                    .map(str::trim)
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .filter(|value| !value.is_empty())
            .collect()
    };
    Ok((roles, permissions))
}

async fn menu_tree(state: &AppState, user: &Authenticated) -> Result<Value, String> {
    let sql = if user.roles.iter().any(|role| role == "super_admin") {
        "SELECT id,name,permission,type,sort,parent_id,path,icon,component,component_name,visible,keep_alive,always_show FROM system_menu WHERE status=0 AND type<>3 ORDER BY sort,id"
    } else {
        "SELECT DISTINCT m.id,m.name,m.permission,m.type,m.sort,m.parent_id,m.path,m.icon,m.component,m.component_name,m.visible,m.keep_alive,m.always_show FROM system_user_role ur JOIN role_menu_mapping rm ON rm.role_id=ur.role_id JOIN system_menu m ON m.id=rm.menu_id WHERE ur.user_id=$1 AND m.status=0 AND m.type<>3 ORDER BY m.sort,m.id"
    };
    let params: Vec<&(dyn ToSql + Sync)> = if user.roles.iter().any(|role| role == "super_admin") {
        vec![]
    } else {
        vec![&user.user_id]
    };
    let rows = state.database.query(sql, &params).await.map_err(db_error)?;
    let mut nodes = BTreeMap::<i64, Value>::new();
    let mut parents = BTreeMap::<i64, i64>::new();
    for row in &rows {
        let id: i64 = row.get("id");
        let parent_id: Option<i64> = row.get("parent_id");
        parents.insert(id, parent_id.unwrap_or(0));
        nodes.insert(id, json!({
            "id": id.to_string(),
            "name": row.get::<_, Option<String>>("name").unwrap_or_default(),
            "path": normalize_menu_path(row.get::<_, Option<String>>("path"), parent_id.is_none_or(|value| value == 0)),
            "component": row.get::<_, Option<String>>("component"),
            "componentName": row.get::<_, Option<String>>("component_name"),
            "icon": row.get::<_, Option<String>>("icon").unwrap_or_default(),
            "visible": row.get::<_, Option<bool>>("visible").unwrap_or(true),
            "keepAlive": row.get::<_, Option<bool>>("keep_alive").unwrap_or(true),
            "alwaysShow": row.get::<_, Option<bool>>("always_show").unwrap_or(false),
            "parentId": parent_id.unwrap_or(0).to_string(),
            "children": []
        }));
    }
    let ids = nodes.keys().copied().collect::<Vec<_>>();
    for id in ids.into_iter().rev() {
        let parent_id = parents.get(&id).copied().unwrap_or(0);
        if let Some(node) = nodes.remove(&id) {
            if let Some(parent) = nodes.get_mut(&parent_id) {
                parent["children"]
                    .as_array_mut()
                    .expect("children 必须是数组")
                    .insert(0, node);
            } else {
                nodes.entry(id).or_insert(node);
            }
        }
    }
    Ok(Value::Array(nodes.into_values().collect()))
}

fn normalize_menu_path(path: Option<String>, root: bool) -> String {
    let value = path.unwrap_or_default().trim().to_owned();
    if !root || value.is_empty() || value.starts_with('/') || value.contains("://") {
        value
    } else {
        format!("/{value}")
    }
}

#[allow(clippy::too_many_arguments)]
fn encode_token(
    state: &AppState,
    session_id: &str,
    issued_at: i64,
    expires_at: i64,
    user_id: i64,
    tenant_id: i64,
    roles: &[String],
    permissions: &[String],
    typ: &str,
) -> Result<String, String> {
    encode(
        &Header::new(Algorithm::HS256),
        &Claims {
            sub: user_id.to_string(),
            typ: typ.into(),
            sid: session_id.into(),
            iat: issued_at,
            exp: expires_at,
            user_id,
            tenant_id,
            roles: roles.to_vec(),
            permissions: permissions.to_vec(),
        },
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| "签发登录令牌失败".to_owned())
}

fn db_error(error: tokio_postgres::Error) -> String {
    format!("数据库操作失败：{error}")
}
