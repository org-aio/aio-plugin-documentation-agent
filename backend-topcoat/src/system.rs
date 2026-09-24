use std::collections::BTreeMap;

use calamine::Reader;
use serde_json::{Value, json};
use tokio_postgres::types::ToSql;

use crate::auth::Authenticated;
use crate::crud::{self, Param};
use crate::{AppState, api};

#[derive(Clone, Copy)]
struct SystemEntity {
    route: &'static str,
    table: &'static str,
    columns: &'static [&'static str],
    filters: &'static [&'static str],
    order: &'static str,
}

pub(crate) fn entity_for_export(route: &str) -> Option<(&'static str, &'static [&'static str])> {
    SYSTEM_ENTITIES
        .iter()
        .find(|entity| entity.route == route)
        .map(|entity| (entity.table, entity.columns))
}

const SYSTEM_ENTITIES: &[SystemEntity] = &[
    SystemEntity {
        route: "system/user",
        table: "system_users",
        columns: &[
            "username", "password", "nickname", "dept_id", "email", "mobile", "sex", "avatar",
            "status", "remark",
        ],
        filters: &["username", "mobile", "status", "dept_id"],
        order: "id DESC",
    },
    SystemEntity {
        route: "system/role",
        table: "system_role",
        columns: &[
            "code",
            "sort",
            "name",
            "remark",
            "status",
            "type",
            "data_scope",
            "data_scope_dept_ids",
        ],
        filters: &["name", "code", "status"],
        order: "sort,id",
    },
    SystemEntity {
        route: "system/menu",
        table: "system_menu",
        columns: &[
            "name",
            "permission",
            "type",
            "sort",
            "parent_id",
            "path",
            "icon",
            "component",
            "component_name",
            "status",
            "visible",
            "keep_alive",
            "always_show",
        ],
        filters: &["name", "status", "parent_id"],
        order: "sort,id",
    },
    SystemEntity {
        route: "system/dept",
        table: "system_dept",
        columns: &[
            "name",
            "parent_id",
            "sort",
            "status",
            "leader_user_id",
            "phone",
            "email",
        ],
        filters: &["name", "status", "parent_id"],
        order: "sort,id",
    },
    SystemEntity {
        route: "system/post",
        table: "system_post",
        columns: &["name", "code", "sort", "status", "remark"],
        filters: &["name", "code", "status"],
        order: "sort,id",
    },
    SystemEntity {
        route: "system/dict-type",
        table: "system_dict_type",
        columns: &["name", "type", "status", "remark"],
        filters: &["name", "type", "status"],
        order: "id DESC",
    },
    SystemEntity {
        route: "system/dict-data",
        table: "system_dict_data",
        columns: &[
            "sort",
            "label",
            "value",
            "dict_type",
            "status",
            "color_type",
            "css_class",
            "remark",
        ],
        filters: &["label", "dict_type", "status"],
        order: "sort,id",
    },
    SystemEntity {
        route: "system/mail-account",
        table: "system_mail_account",
        columns: &[
            "mail",
            "username",
            "password",
            "host",
            "port",
            "ssl_enable",
            "starttls_enable",
        ],
        filters: &["mail", "host"],
        order: "id DESC",
    },
    SystemEntity {
        route: "infra/config",
        table: "infra_config",
        columns: &[
            "category",
            "type",
            "name",
            "config_key",
            "value",
            "visible",
            "remark",
        ],
        filters: &["category", "type", "name", "config_key", "visible"],
        order: "id DESC",
    },
    SystemEntity {
        route: "infra/file-config",
        table: "infra_file_config",
        columns: &["name", "storage", "master", "visible", "config", "remark"],
        filters: &["name", "storage", "master", "visible"],
        order: "id DESC",
    },
    SystemEntity {
        route: "system/login-log",
        table: "system_login_log",
        columns: &[
            "log_type",
            "trace_id",
            "user_id",
            "user_type",
            "username",
            "result",
            "status",
            "user_ip",
            "user_agent",
        ],
        filters: &["username", "result", "status", "user_id"],
        order: "id DESC",
    },
    SystemEntity {
        route: "system/operate-log",
        table: "system_operate_log",
        columns: &[
            "trace_id",
            "user_type",
            "user_id",
            "user_name",
            "type",
            "sub_type",
            "biz_id",
            "action",
            "extra",
            "request_method",
            "request_url",
            "user_ip",
            "user_agent",
            "creator",
            "creator_name",
        ],
        filters: &["user_name", "type", "action", "user_id"],
        order: "id DESC",
    },
    SystemEntity {
        route: "infra/api-access-log",
        table: "infra_api_access_log",
        columns: &[
            "trace_id",
            "user_id",
            "user_type",
            "application_name",
            "request_method",
            "request_params",
            "response_body",
            "request_url",
            "user_ip",
            "user_agent",
            "operate_module",
            "operate_name",
            "operate_type",
            "begin_time",
            "end_time",
            "duration",
            "result_code",
            "result_msg",
        ],
        filters: &["request_url", "result_code", "user_id", "duration"],
        order: "id DESC",
    },
    SystemEntity {
        route: "infra/api-error-log",
        table: "infra_api_error_log",
        columns: &[
            "trace_id",
            "user_id",
            "user_type",
            "application_name",
            "request_method",
            "request_params",
            "request_url",
            "user_ip",
            "user_agent",
            "exception_time",
            "exception_name",
            "exception_message",
            "exception_root_cause_message",
            "exception_stack_trace",
            "exception_class_name",
            "exception_file_name",
            "exception_method_name",
            "exception_line_number",
            "process_user_id",
            "process_status",
            "process_time",
            "result_code",
        ],
        filters: &["request_url", "process_status", "user_id", "result_code"],
        order: "id DESC",
    },
];

pub(crate) async fn handle(
    state: &AppState,
    user: &Authenticated,
    method: &str,
    route: &str,
    operation: &str,
    query: &BTreeMap<String, Vec<String>>,
    body: Value,
) -> Result<Value, String> {
    if route == "system/user/profile" {
        return profile(state, user, method, operation, body).await;
    }
    if route == "system/permission" {
        return permission(state, user, method, operation, query, body).await;
    }
    if route == "system/captcha" {
        return captcha(method, operation);
    }
    if route == "infra/file" || route.starts_with("infra/file/") {
        return crate::files::handle(state, user, method, operation, query, body).await;
    }
    if route == "system/user" && operation == "get-import-template" {
        return import_user_template();
    }
    if route == "system/user" && operation == "import" && method == "POST" {
        return import_users(state, body).await;
    }
    let entity = SYSTEM_ENTITIES
        .iter()
        .find(|entity| entity.route == route)
        .ok_or_else(|| "未知的系统模块".to_owned())?;
    match (method, operation) {
        ("GET", "page") | ("GET", "list") => {
            page(state, entity, query, route == "system/dept/list").await
        }
        ("GET", "get") => get(state, entity, query).await,
        ("GET", "simple-list") => simple_list(state, entity, query).await,
        ("POST", "create") | ("POST", "upsert") => create(state, entity, body).await,
        ("PUT", "update") => update(state, entity, body).await,
        ("DELETE", "delete") => delete(state, entity, query).await,
        ("DELETE", "delete-list") => delete_list(state, entity, query).await,
        ("GET", "type") if entity.route == "system/dict-data" => dict_by_type(state, query).await,
        ("GET", "get-value-by-key") if entity.route == "infra/config" => {
            config_value(state, query).await
        }
        ("PUT", "update-status") if entity.route == "system/role" => {
            update_status(state, entity, body).await
        }
        ("PUT", "update-status") if entity.route == "system/user" => {
            update_status(state, entity, body).await
        }
        ("PUT", "update-password") if entity.route == "system/user" => {
            update_password(state, body).await
        }
        ("PUT", "update-status") if entity.route == "infra/api-error-log" => {
            update_error_log_status(state, query).await
        }
        ("PUT", "update-master") if entity.route == "infra/file-config" => {
            update_file_master(state, query).await
        }
        ("GET", "test") if entity.route == "infra/file-config" => {
            test_file_config(state, query).await
        }
        _ => Err(format!("未实现的系统操作：{method} {route}/{operation}")),
    }
}

async fn update_error_log_status(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = required_id(query)?;
    let status = int_query(query, "processStatus")?.unwrap_or(0);
    state
        .database
        .execute(
            "UPDATE infra_api_error_log SET process_status=$1,process_time=CURRENT_TIMESTAMP WHERE id=$2",
            &[&(status as i32), &id],
        )
        .await
        .map_err(db_error)?;
    Ok(json!(true))
}

fn import_user_template() -> Result<Value, String> {
    let csv = "\u{feff}用户账号,用户昵称,部门编号,手机号码,邮箱,性别,岗位编号,角色编号,备注\n";
    Ok(json!({
        "base64": base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            csv.as_bytes()
        ),
        "contentType": "application/vnd.ms-excel",
        "fileName": "用户导入模版.csv"
    }))
}

async fn import_users(state: &AppState, body: Value) -> Result<Value, String> {
    let name = body
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("users.xlsx");
    let encoded = body
        .get("base64")
        .and_then(Value::as_str)
        .ok_or_else(|| "导入文件缺少 base64".to_owned())?;
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)
        .map_err(|_| "导入文件不是有效 base64".to_owned())?;
    let mut workbook = calamine::open_workbook_auto_from_rs(std::io::Cursor::new(bytes))
        .map_err(|error| format!("无法读取导入文件 {name}：{error}"))?;
    let sheet = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| "导入文件没有工作表".to_owned())?;
    let range = workbook
        .worksheet_range(&sheet)
        .map_err(|error| format!("无法读取工作表 {sheet}：{error}"))?;
    let mut rows = range.rows();
    let Some(headers) = rows.next() else {
        return Err("导入文件没有表头".into());
    };
    let header = headers.iter().map(cell_text).collect::<Vec<_>>();
    let column = |names: &[&str]| {
        header
            .iter()
            .position(|value| names.iter().any(|name| value.contains(name)))
    };
    let username_column =
        column(&["用户账号", "账号", "username"]).ok_or_else(|| "缺少用户账号列".to_owned())?;
    let nickname_column = column(&["用户昵称", "昵称", "nickname"]).unwrap_or(username_column);
    let mobile_column = column(&["手机号码", "手机号", "mobile"]);
    let email_column = column(&["邮箱", "email"]);
    let status_column = column(&["状态", "status"]);
    let mut create_usernames = Vec::new();
    let mut update_usernames = Vec::new();
    let mut failure_usernames = serde_json::Map::new();
    for (offset, row) in rows.enumerate() {
        let username = row.get(username_column).map(cell_text).unwrap_or_default();
        if username.trim().is_empty() {
            continue;
        }
        let nickname = row
            .get(nickname_column)
            .map(cell_text)
            .unwrap_or_else(|| username.clone());
        let mobile = mobile_column
            .and_then(|index| row.get(index))
            .map(cell_text)
            .unwrap_or_default();
        let email = email_column
            .and_then(|index| row.get(index))
            .map(cell_text)
            .unwrap_or_default();
        let status = status_column
            .and_then(|index| row.get(index))
            .map(cell_text)
            .and_then(|value| match value.trim() {
                "开启" | "正常" | "启用" | "1" => Some(0_i32),
                "关闭" | "禁用" | "0" => Some(1_i32),
                _ => None,
            })
            .unwrap_or(0);
        let existing = state
            .database
            .query_opt(
                "SELECT id FROM system_users WHERE username=$1 LIMIT 1",
                &[&username],
            )
            .await
            .map_err(db_error)?;
        let result = if let Some(existing) = existing {
            let id: i64 = existing.get(0);
            state
                .database
                .execute(
                    "UPDATE system_users SET nickname=$1,mobile=$2,email=$3,status=$4,update_time=CURRENT_TIMESTAMP WHERE id=$5",
                    &[&nickname, &mobile, &email, &status, &id],
                )
                .await
        } else {
            let id = next_id(state, "system_users").await?;
            let password = bcrypt::hash("admin123", 4)
                .map_err(|error| format!("默认密码加密失败：{error}"))?;
            state
                .database
                .execute(
                    "INSERT INTO system_users (create_time,id,tenant_id,status,username,password,nickname,realname,dept_id,email,mobile,sex) VALUES (CURRENT_TIMESTAMP,$1,1,$2,$3,$4,$5,$5,100,$6,$7,'1')",
                    &[&id, &status, &username, &password, &nickname, &email, &mobile],
                )
                .await
        };
        match result {
            Ok(_) if existing_is_present(state, &username).await? => {
                update_usernames.push(username)
            }
            Ok(_) => create_usernames.push(username),
            Err(error) => {
                failure_usernames.insert(
                    username,
                    Value::String(format!("第 {} 行：{error}", offset + 2)),
                );
            }
        }
    }
    Ok(json!({
        "createUsernames": create_usernames,
        "updateUsernames": update_usernames,
        "failureUsernames": failure_usernames
    }))
}

async fn existing_is_present(state: &AppState, username: &str) -> Result<bool, String> {
    state
        .database
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM system_users WHERE username=$1)",
            &[&username],
        )
        .await
        .map_err(db_error)
        .map(|row| row.get(0))
}

fn cell_text(value: &calamine::Data) -> String {
    match value {
        calamine::Data::Empty => String::new(),
        calamine::Data::String(value) => value.clone(),
        calamine::Data::Float(value) => value.to_string(),
        calamine::Data::Int(value) => value.to_string(),
        calamine::Data::Bool(value) => value.to_string(),
        calamine::Data::DateTime(value) => value.to_string(),
        calamine::Data::DateTimeIso(value) => value.clone(),
        calamine::Data::DurationIso(value) => value.clone(),
        calamine::Data::Error(value) => format!("{value:?}"),
    }
}

async fn update_file_master(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = required_id(query)?;
    state
        .database
        .execute("UPDATE infra_file_config SET master=FALSE", &[])
        .await
        .map_err(db_error)?;
    state
        .database
        .execute(
            "UPDATE infra_file_config SET master=TRUE,update_time=CURRENT_TIMESTAMP WHERE id=$1",
            &[&id],
        )
        .await
        .map_err(db_error)?;
    Ok(json!(true))
}

async fn test_file_config(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = required_id(query)?;
    let exists = state
        .database
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM infra_file_config WHERE id=$1)",
            &[&id],
        )
        .await
        .map_err(db_error)?
        .get::<_, bool>(0);
    if !exists {
        return Err("文件配置不存在".into());
    }
    Ok(Value::String("本地 PostgreSQL 文件存储可用".into()))
}

async fn page(
    state: &AppState,
    entity: &SystemEntity,
    query: &BTreeMap<String, Vec<String>>,
    tree: bool,
) -> Result<Value, String> {
    if tree {
        return simple_list(state, entity, query).await;
    }
    let page_no = int_query(query, "pageNo")?.unwrap_or(1).max(1);
    let page_size = int_query(query, "pageSize")?.unwrap_or(10).clamp(1, 500);
    let (where_sql, params) = filters(entity, query)?;
    let count_sql = format!(
        "SELECT count(*)::bigint FROM {} {where_sql}",
        quote(entity.table)
    );
    let total: i64 = state
        .database
        .query_one(&count_sql, &sql_refs(&params))
        .await
        .map_err(db_error)?
        .get(0);
    let mut params = params;
    params.push(Param::I64(page_size));
    params.push(Param::I64((page_no - 1) * page_size));
    let sql = format!(
        "SELECT * FROM {} {where_sql} ORDER BY {} LIMIT ${} OFFSET ${}",
        quote(entity.table),
        entity.order,
        params.len() - 1,
        params.len()
    );
    let rows = state
        .database
        .query(&sql, &sql_refs(&params))
        .await
        .map_err(db_error)?;
    Ok(api::page(
        Value::Array(rows.iter().map(row_value).collect()),
        total,
    ))
}

async fn get(
    state: &AppState,
    entity: &SystemEntity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = required_id(query)?;
    let sql = format!("SELECT * FROM {} WHERE id=$1", quote(entity.table));
    let row = state
        .database
        .query_opt(&sql, &[&id])
        .await
        .map_err(db_error)?;
    Ok(row.as_ref().map(row_value).unwrap_or(Value::Null))
}

async fn simple_list(
    state: &AppState,
    entity: &SystemEntity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let (where_sql, params) = filters(entity, query)?;
    let sql = format!(
        "SELECT * FROM {} {where_sql} ORDER BY {} LIMIT 1000",
        quote(entity.table),
        entity.order
    );
    let rows = state
        .database
        .query(&sql, &sql_refs(&params))
        .await
        .map_err(db_error)?;
    Ok(Value::Array(rows.iter().map(row_value).collect()))
}

async fn create(state: &AppState, entity: &SystemEntity, body: Value) -> Result<Value, String> {
    let map = body
        .as_object()
        .ok_or_else(|| "请求体必须是 JSON 对象".to_owned())?;
    let mut params = vec![Param::I64(next_id(state, entity.table).await?)];
    let mut columns = vec!["id".to_owned(), "create_time".to_owned()];
    params.push(Param::Timestamp(chrono::Utc::now().naive_utc()));
    for column in entity.columns {
        if let Some(value) = map.get(*column) {
            if *column == "password" {
                let password = value.as_str().unwrap_or_default();
                params.push(Param::Text(
                    bcrypt::hash(password, 4).map_err(|error| format!("密码加密失败：{error}"))?,
                ));
            } else {
                params.push(json_param(value));
            }
            columns.push((*column).to_owned());
        }
    }
    let placeholders = (1..=params.len())
        .map(|index| format!("${index}"))
        .collect::<Vec<_>>()
        .join(",");
    let names = columns
        .iter()
        .map(|column| quote(column))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "INSERT INTO {} ({names}) VALUES ({placeholders}) RETURNING *",
        quote(entity.table)
    );
    let row = state
        .database
        .query_one(&sql, &sql_refs(&params))
        .await
        .map_err(db_error)?;
    Ok(row_value(&row))
}

async fn update(state: &AppState, entity: &SystemEntity, body: Value) -> Result<Value, String> {
    let map = body
        .as_object()
        .ok_or_else(|| "请求体必须是 JSON 对象".to_owned())?;
    let id = map
        .get("id")
        .and_then(value_as_i64)
        .ok_or_else(|| "缺少 id".to_owned())?;
    let mut params = vec![Param::Timestamp(chrono::Utc::now().naive_utc())];
    let mut assignments = vec!["update_time=$1".to_owned()];
    for column in entity.columns {
        if *column == "password" || !map.contains_key(*column) {
            continue;
        }
        params.push(json_param(map.get(*column).unwrap()));
        assignments.push(format!("{}=${}", quote(column), params.len()));
    }
    params.push(Param::I64(id));
    let sql = format!(
        "UPDATE {} SET {} WHERE id=${} RETURNING *",
        quote(entity.table),
        assignments.join(","),
        params.len()
    );
    let row = state
        .database
        .query_opt(&sql, &sql_refs(&params))
        .await
        .map_err(db_error)?;
    Ok(row.as_ref().map(row_value).unwrap_or(Value::Null))
}

async fn delete(
    state: &AppState,
    entity: &SystemEntity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = required_id(query)?;
    let sql = format!("DELETE FROM {} WHERE id=$1", quote(entity.table));
    let affected = state
        .database
        .execute(&sql, &[&id])
        .await
        .map_err(db_error)?;
    Ok(json!(affected))
}

async fn delete_list(
    state: &AppState,
    entity: &SystemEntity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let ids = query
        .get("ids")
        .and_then(|values| values.first())
        .map(|value| {
            value
                .split(',')
                .filter_map(|item| item.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if ids.is_empty() {
        return Ok(json!(0));
    }
    let placeholders = (1..=ids.len())
        .map(|index| format!("${index}"))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "DELETE FROM {} WHERE id IN ({placeholders})",
        quote(entity.table)
    );
    let refs = ids
        .iter()
        .map(|value| value as &(dyn ToSql + Sync))
        .collect::<Vec<_>>();
    Ok(json!(
        state
            .database
            .execute(&sql, &refs)
            .await
            .map_err(db_error)?
    ))
}

async fn profile(
    state: &AppState,
    user: &Authenticated,
    method: &str,
    operation: &str,
    body: Value,
) -> Result<Value, String> {
    match (method, operation) {
        ("GET", "get") => {
            let row = state
                .database
                .query_one(
                    "SELECT id,username,nickname,email,mobile,sex,avatar,create_time FROM system_users WHERE id=$1",
                    &[&user.user_id],
                )
                .await
                .map_err(db_error)?;
            let roles = state
                .database
                .query(
                    "SELECT r.id,r.name,r.code FROM system_user_role ur JOIN system_role r ON r.id=ur.role_id WHERE ur.user_id=$1",
                    &[&user.user_id],
                )
                .await
                .map_err(db_error)?;
            Ok(json!({
                "id": row.get::<_, i64>("id"),
                "username": row.get::<_, String>("username"),
                "nickname": row.get::<_, Option<String>>("nickname").unwrap_or_default(),
                "email": row.get::<_, Option<String>>("email").unwrap_or_default(),
                "mobile": row.get::<_, Option<String>>("mobile").unwrap_or_default(),
                "sex": row.get::<_, Option<i32>>("sex"),
                "avatar": row.get::<_, Option<String>>("avatar").unwrap_or_default(),
                "createTime": row.get::<_, chrono::NaiveDateTime>("create_time").to_string(),
                "roles": roles.iter().map(|role| json!({
                    "id": role.get::<_, i64>("id"),
                    "name": role.get::<_, Option<String>>("name"),
                    "code": role.get::<_, Option<String>>("code"),
                })).collect::<Vec<_>>(),
                "posts": []
            }))
        }
        ("PUT", "update") => {
            let map = body
                .as_object()
                .ok_or_else(|| "请求体必须是 JSON 对象".to_owned())?;
            let mut params = Vec::<Param>::new();
            let mut assignments = vec!["update_time=$1".to_owned()];
            params.push(Param::Timestamp(chrono::Utc::now().naive_utc()));
            for column in ["nickname", "email", "mobile", "sex", "avatar"] {
                if let Some(value) = map.get(column) {
                    params.push(json_param(value));
                    assignments.push(format!("{}=${}", quote(column), params.len()));
                }
            }
            params.push(Param::I64(user.user_id));
            state
                .database
                .execute(
                    &format!(
                        "UPDATE system_users SET {} WHERE id=${}",
                        assignments.join(","),
                        params.len()
                    ),
                    &sql_refs(&params),
                )
                .await
                .map_err(db_error)?;
            Ok(json!(true))
        }
        ("PUT", "update-password") => {
            let old_password = body
                .get("oldPassword")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let new_password = body
                .get("newPassword")
                .and_then(Value::as_str)
                .filter(|value| value.len() >= 4 && value.len() <= 64)
                .ok_or_else(|| "新密码长度必须为 4-64 位".to_owned())?;
            let row = state
                .database
                .query_one(
                    "SELECT password FROM system_users WHERE id=$1",
                    &[&user.user_id],
                )
                .await
                .map_err(db_error)?;
            let hash: Option<String> = row.get("password");
            if !hash
                .as_deref()
                .is_some_and(|hash| bcrypt::verify(old_password, hash).unwrap_or(false))
            {
                return Err("原密码不正确".into());
            }
            let password =
                bcrypt::hash(new_password, 4).map_err(|error| format!("密码加密失败：{error}"))?;
            state
                .database
                .execute(
                    "UPDATE system_users SET password=$1,update_time=CURRENT_TIMESTAMP WHERE id=$2",
                    &[&password, &user.user_id],
                )
                .await
                .map_err(db_error)?;
            Ok(json!(true))
        }
        _ => Err(format!("未实现的个人资料操作：{method} {operation}")),
    }
}

async fn permission(
    state: &AppState,
    _user: &Authenticated,
    method: &str,
    operation: &str,
    query: &BTreeMap<String, Vec<String>>,
    body: Value,
) -> Result<Value, String> {
    match (method, operation) {
        ("GET", "list-role-menus") => {
            let role_id = int_query(query, "roleId")?.ok_or_else(|| "缺少 roleId".to_owned())?;
            let rows = state
                .database
                .query(
                    "SELECT menu_id FROM role_menu_mapping WHERE role_id=$1",
                    &[&role_id],
                )
                .await
                .map_err(db_error)?;
            Ok(Value::Array(
                rows.iter()
                    .map(|row| json!(row.get::<_, i64>("menu_id")))
                    .collect(),
            ))
        }
        ("POST", "assign-role-menu") => {
            let role_id = body
                .get("roleId")
                .and_then(value_as_i64)
                .ok_or_else(|| "缺少 roleId".to_owned())?;
            let menu_ids = body
                .get("menuIds")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
                .unwrap_or_default();
            state
                .database
                .execute(
                    "DELETE FROM role_menu_mapping WHERE role_id=$1",
                    &[&role_id],
                )
                .await
                .map_err(db_error)?;
            for menu_id in menu_ids {
                state
                    .database
                    .execute(
                        "INSERT INTO role_menu_mapping (role_id,menu_id) VALUES ($1,$2) ON CONFLICT DO NOTHING",
                        &[&role_id, &menu_id],
                    )
                    .await
                    .map_err(db_error)?;
            }
            Ok(json!(true))
        }
        ("GET", "list-user-roles") => {
            let user_id = int_query(query, "userId")?.ok_or_else(|| "缺少 userId".to_owned())?;
            let rows = state
                .database
                .query(
                    "SELECT role_id FROM system_user_role WHERE user_id=$1",
                    &[&user_id],
                )
                .await
                .map_err(db_error)?;
            Ok(Value::Array(
                rows.iter()
                    .map(|row| json!(row.get::<_, i64>("role_id")))
                    .collect(),
            ))
        }
        ("POST", "assign-user-role") => {
            let user_id = body
                .get("userId")
                .and_then(value_as_i64)
                .ok_or_else(|| "缺少 userId".to_owned())?;
            let role_ids = body
                .get("roleIds")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
                .unwrap_or_default();
            state
                .database
                .execute("DELETE FROM system_user_role WHERE user_id=$1", &[&user_id])
                .await
                .map_err(db_error)?;
            for role_id in role_ids {
                state
                    .database
                    .execute(
                        "INSERT INTO system_user_role (user_id,role_id) VALUES ($1,$2) ON CONFLICT DO NOTHING",
                        &[&user_id, &role_id],
                    )
                    .await
                    .map_err(db_error)?;
            }
            Ok(json!(true))
        }
        ("POST", "assign-role-data-scope") => {
            let role_id = body
                .get("roleId")
                .and_then(value_as_i64)
                .ok_or_else(|| "缺少 roleId".to_owned())?;
            let data_scope = body
                .get("dataScope")
                .and_then(value_as_i64)
                .ok_or_else(|| "缺少 dataScope".to_owned())?;
            let dept_ids = body.get("dataScopeDeptIds").cloned().unwrap_or(Value::Null);
            state
                .database
                .execute(
                    "UPDATE system_role SET data_scope=$1,data_scope_dept_ids=$2,update_time=CURRENT_TIMESTAMP WHERE id=$3",
                    &[&(data_scope as i32), &serde_json::to_string(&dept_ids).unwrap_or_else(|_| "[]".into()), &role_id],
                )
                .await
                .map_err(db_error)?;
            Ok(json!(true))
        }
        _ => Err(format!("未实现的权限操作：{method} {operation}")),
    }
}

fn captcha(method: &str, operation: &str) -> Result<Value, String> {
    match (method, operation) {
        ("GET", "config") => Ok(json!({"enabled": false, "type": "blockPuzzle"})),
        _ => Err(format!("未实现的验证码操作：{method} {operation}")),
    }
}

async fn dict_by_type(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let dict_type = query
        .get("type")
        .and_then(|values| values.first())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "缺少字典类型 type".to_owned())?;
    let rows = state
        .database
        .query(
            "SELECT * FROM system_dict_data WHERE dict_type=$1 AND status=0 AND deleted=0 ORDER BY sort,id",
            &[&dict_type],
        )
        .await
        .map_err(db_error)?;
    Ok(Value::Array(rows.iter().map(row_value).collect()))
}

async fn config_value(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let key = query
        .get("key")
        .and_then(|values| values.first())
        .ok_or_else(|| "缺少参数键 key".to_owned())?;
    let row = state
        .database
        .query_opt(
            "SELECT value FROM infra_config WHERE config_key=$1 AND visible=TRUE ORDER BY id LIMIT 1",
            &[&key],
        )
        .await
        .map_err(db_error)?;
    Ok(row
        .map(|row| Value::String(row.get::<_, Option<String>>("value").unwrap_or_default()))
        .unwrap_or(Value::Null))
}

async fn update_status(
    state: &AppState,
    entity: &SystemEntity,
    body: Value,
) -> Result<Value, String> {
    let id = body
        .get("id")
        .and_then(value_as_i64)
        .ok_or_else(|| "缺少 id".to_owned())?;
    let status = body
        .get("status")
        .and_then(value_as_i64)
        .ok_or_else(|| "缺少 status".to_owned())? as i32;
    let sql = format!(
        "UPDATE {} SET status=$1,update_time=CURRENT_TIMESTAMP WHERE id=$2",
        quote(entity.table)
    );
    state
        .database
        .execute(&sql, &[&status, &id])
        .await
        .map_err(db_error)?;
    Ok(json!(true))
}

async fn update_password(state: &AppState, body: Value) -> Result<Value, String> {
    let id = body
        .get("id")
        .and_then(value_as_i64)
        .ok_or_else(|| "缺少 id".to_owned())?;
    let password = body
        .get("password")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "密码不能为空".to_owned())?;
    let hash = bcrypt::hash(password, 4).map_err(|error| format!("密码加密失败：{error}"))?;
    state
        .database
        .execute(
            "UPDATE system_users SET password=$1,update_time=CURRENT_TIMESTAMP WHERE id=$2",
            &[&hash, &id],
        )
        .await
        .map_err(db_error)?;
    Ok(json!(true))
}

fn filters(
    entity: &SystemEntity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<(String, Vec<Param>), String> {
    let mut clauses = Vec::new();
    let mut params = Vec::new();
    for key in entity.filters {
        let Some(value) = query
            .get(*key)
            .and_then(|values| values.first())
            .filter(|value| !value.is_empty())
        else {
            continue;
        };
        params.push(Param::Text(value.clone()));
        clauses.push(format!("{}=${}", quote(key), params.len()));
    }
    Ok((
        if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        },
        params,
    ))
}

async fn next_id(state: &AppState, table: &str) -> Result<i64, String> {
    let value: i64 = state
        .database
        .query_one(
            &format!("SELECT COALESCE(MAX(id),0)+1 FROM {}", quote(table)),
            &[],
        )
        .await
        .map_err(db_error)?
        .get(0);
    Ok(value.max(1))
}

fn row_value(row: &tokio_postgres::Row) -> Value {
    crud::row_value(row)
}

fn json_param(value: &Value) -> Param {
    crud::json_param(value, None).expect("系统字段参数无效")
}

fn sql_refs(params: &[Param]) -> Vec<&(dyn ToSql + Sync)> {
    crud::sql_refs(params)
}

fn int_query(query: &BTreeMap<String, Vec<String>>, key: &str) -> Result<Option<i64>, String> {
    query
        .get(key)
        .and_then(|values| values.first())
        .map(|value| value.parse().map_err(|_| format!("{key} 不是有效整数")))
        .transpose()
}

fn required_id(query: &BTreeMap<String, Vec<String>>) -> Result<i64, String> {
    int_query(query, "id")?.ok_or_else(|| "缺少 id".to_owned())
}

fn value_as_i64(value: &Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_str()?.parse().ok())
}

fn quote(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

fn db_error(error: tokio_postgres::Error) -> String {
    format!("数据库操作失败：{error}")
}
