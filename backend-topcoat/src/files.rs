use std::collections::BTreeMap;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

use crate::auth::Authenticated;
use crate::{AppState, api};

const MAX_FILE_BYTES: usize = 2 * 1024 * 1024;

pub(crate) async fn handle(
    state: &AppState,
    user: &Authenticated,
    method: &str,
    operation: &str,
    query: &BTreeMap<String, Vec<String>>,
    body: Value,
) -> Result<Value, String> {
    match (method, operation) {
        ("GET", "page") => page(state, query).await,
        ("POST", "upload") => upload(state, user, body).await,
        ("GET", "content") => content(state, query).await,
        ("GET", "presigned-url") => presigned_url(state, user, query).await,
        ("POST", "create") => create_metadata(state, user, body).await,
        ("DELETE", "delete") => delete(state, query).await,
        ("DELETE", "delete-list") => delete_list(state, query).await,
        ("POST", "upload-folder") => upload_folder(state, user, body).await,
        _ => Err(format!("未实现的文件操作：{method} {operation}")),
    }
}

async fn page(state: &AppState, query: &BTreeMap<String, Vec<String>>) -> Result<Value, String> {
    let page_no = query
        .get("pageNo")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let page_size = query
        .get("pageSize")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(10)
        .clamp(1, 500);
    let name = query
        .get("name")
        .and_then(|values| values.first())
        .filter(|value| !value.is_empty());
    let total: i64 = if let Some(name) = name {
        state
            .database
            .query_one(
                "SELECT count(*)::bigint FROM infra_file WHERE name ILIKE $1",
                &[&format!("%{name}%")],
            )
            .await
            .map_err(db_error)?
            .get(0)
    } else {
        state
            .database
            .query_one("SELECT count(*)::bigint FROM infra_file", &[])
            .await
            .map_err(db_error)?
            .get(0)
    };
    let rows = if let Some(name) = name {
        state
            .database
            .query(
                "SELECT id,name,config_id,path,url,type,size,create_time FROM infra_file WHERE name ILIKE $1 ORDER BY id DESC LIMIT $2 OFFSET $3",
                &[&format!("%{name}%"), &page_size, &((page_no - 1) * page_size)],
            )
            .await
            .map_err(db_error)?
    } else {
        state
            .database
            .query(
                "SELECT id,name,config_id,path,url,type,size,create_time FROM infra_file ORDER BY id DESC LIMIT $1 OFFSET $2",
                &[&page_size, &((page_no - 1) * page_size)],
            )
            .await
            .map_err(db_error)?
    };
    Ok(api::page(
        Value::Array(rows.iter().map(file_row).collect()),
        total,
    ))
}

async fn upload(state: &AppState, user: &Authenticated, body: Value) -> Result<Value, String> {
    let (name, directory, bytes) = decode_upload(body)?;
    let id: i64 = state
        .database
        .query_one("SELECT COALESCE(MAX(id),0)+1 FROM infra_file", &[])
        .await
        .map_err(db_error)?
        .get(0);
    let safe_name = sanitize_name(&name);
    let path = if directory.trim().is_empty() {
        format!("{id}_{safe_name}")
    } else {
        format!("{}/{id}_{safe_name}", directory.trim_matches('/'))
    };
    let url = format!("/infra/file/content?id={id}");
    let media_type = media_type(&name);
    let creator = user.user_id.to_string();
    state
        .database
        .execute(
            "WITH inserted AS (
                INSERT INTO infra_file (create_time,id,creator,name,config_id,path,url,type,size)
                VALUES (CURRENT_TIMESTAMP,$1,$2,$3,1,$4,$5,$6,$7) RETURNING id
            )
            INSERT INTO infra_file_content (id,config_id,path,content,creator,create_time,updater,update_time,deleted)
            SELECT inserted.id,1,$4,$8,$9,CURRENT_TIMESTAMP,$9,CURRENT_TIMESTAMP,0 FROM inserted",
            &[
                &id,
                &user.user_id,
                &name,
                &path,
                &url,
                &media_type,
                &(bytes.len() as i32),
                &bytes,
                &creator,
            ],
        )
        .await
        .map_err(db_error)?;
    Ok(Value::String(url))
}

async fn upload_folder(
    state: &AppState,
    user: &Authenticated,
    body: Value,
) -> Result<Value, String> {
    let files = body
        .get("files")
        .and_then(Value::as_array)
        .ok_or_else(|| "缺少 files".to_owned())?;
    let directory = body
        .get("directory")
        .and_then(Value::as_str)
        .unwrap_or("folder");
    let mut urls = Vec::new();
    for file in files {
        let mut request = file.clone();
        if request.get("directory").is_none() {
            request["directory"] = Value::String(directory.into());
        }
        urls.push(upload(state, user, request).await?);
    }
    Ok(Value::Array(urls))
}

async fn content(state: &AppState, query: &BTreeMap<String, Vec<String>>) -> Result<Value, String> {
    let id = query
        .get("id")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| "缺少文件 id".to_owned())?;
    let row = state
        .database
        .query_opt(
            "SELECT f.name,f.type,c.content FROM infra_file f JOIN infra_file_content c ON c.id=f.id WHERE f.id=$1 AND c.deleted=0",
            &[&id],
        )
        .await
        .map_err(db_error)?
        .ok_or_else(|| "文件不存在".to_owned())?;
    let bytes: Vec<u8> = row.get("content");
    Ok(json!({
        "name": row.get::<_, Option<String>>("name").unwrap_or_default(),
        "contentType": row.get::<_, Option<String>>("type").unwrap_or_else(|| "application/octet-stream".into()),
        "base64": STANDARD.encode(bytes)
    }))
}

async fn presigned_url(
    state: &AppState,
    _user: &Authenticated,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let name = query
        .get("name")
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_else(|| "file".into());
    let directory = query
        .get("directory")
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
    let id: i64 = state
        .database
        .query_one("SELECT COALESCE(MAX(id),0)+1 FROM infra_file", &[])
        .await
        .map_err(db_error)?
        .get(0);
    let path = format!(
        "{}/{id}_{}",
        directory.trim_matches('/'),
        sanitize_name(&name)
    );
    Ok(json!({
        "configId": 1,
        "uploadUrl": "/infra/file/upload",
        "url": format!("/infra/file/content?id={id}"),
        "path": path
    }))
}

async fn create_metadata(
    state: &AppState,
    user: &Authenticated,
    body: Value,
) -> Result<Value, String> {
    let name = body
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| "缺少 name".to_owned())?;
    let path = body.get("path").and_then(Value::as_str).unwrap_or(name);
    let url = body
        .get("url")
        .and_then(Value::as_str)
        .unwrap_or("/infra/file/content");
    let size = body
        .get("size")
        .and_then(Value::as_i64)
        .unwrap_or(0)
        .clamp(0, i32::MAX as i64) as i32;
    let id: i64 = state
        .database
        .query_one("SELECT COALESCE(MAX(id),0)+1 FROM infra_file", &[])
        .await
        .map_err(db_error)?
        .get(0);
    state
        .database
        .execute(
            "INSERT INTO infra_file (create_time,id,creator,name,config_id,path,url,type,size) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,1,$4,$5,$6,$7)",
            &[&id, &user.user_id, &name, &path, &url, &media_type(name), &size],
        )
        .await
        .map_err(db_error)?;
    Ok(json!(id))
}

async fn delete(state: &AppState, query: &BTreeMap<String, Vec<String>>) -> Result<Value, String> {
    let id = query
        .get("id")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| "缺少文件 id".to_owned())?;
    remove_ids(state, &[id]).await
}

async fn delete_list(
    state: &AppState,
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
    remove_ids(state, &ids).await
}

async fn remove_ids(state: &AppState, ids: &[i64]) -> Result<Value, String> {
    if ids.is_empty() {
        return Ok(json!(0));
    }
    let placeholders = (1..=ids.len())
        .map(|index| format!("${index}"))
        .collect::<Vec<_>>()
        .join(",");
    let affected = state
        .database
        .execute(
            &format!(
                "WITH removed_content AS (DELETE FROM infra_file_content WHERE id IN ({placeholders}) RETURNING id) DELETE FROM infra_file WHERE id IN ({placeholders})"
            ),
            &ids.iter().map(|id| id as &(dyn tokio_postgres::types::ToSql + Sync)).collect::<Vec<_>>(),
        )
        .await
        .map_err(db_error)?;
    Ok(json!(affected))
}

fn decode_upload(body: Value) -> Result<(String, String, Vec<u8>), String> {
    let name = body
        .get("name")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "上传缺少文件名".to_owned())?
        .replace(['/', '\\', '\0'], "_");
    let directory = body
        .get("directory")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim_matches('/')
        .replace(['\\', '\0'], "_");
    let encoded = body
        .get("base64")
        .and_then(Value::as_str)
        .ok_or_else(|| "上传缺少 base64".to_owned())?;
    let bytes = STANDARD
        .decode(encoded)
        .map_err(|_| "上传内容不是有效 base64".to_owned())?;
    if bytes.is_empty() || bytes.len() > MAX_FILE_BYTES {
        return Err(format!("文件大小必须在 1 到 {MAX_FILE_BYTES} 字节之间"));
    }
    Ok((name, directory, bytes))
}

fn sanitize_name(name: &str) -> String {
    let value = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || "._-".contains(character) {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if value.is_empty() {
        "file".into()
    } else {
        value
    }
}

fn media_type(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".pdf") {
        "application/pdf"
    } else if lower.ends_with(".xlsx") {
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    } else if lower.ends_with(".xls") {
        "application/vnd.ms-excel"
    } else if lower.ends_with(".docx") {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    } else if lower.ends_with(".doc") {
        "application/msword"
    } else if lower.ends_with(".zip") {
        "application/zip"
    } else if lower.ends_with(".json") {
        "application/json"
    } else if lower.ends_with(".txt") {
        "text/plain;charset=utf-8"
    } else {
        "application/octet-stream"
    }
    .into()
}

fn file_row(row: &tokio_postgres::Row) -> Value {
    json!({
        "id": row.get::<_, i64>("id"),
        "name": row.get::<_, Option<String>>("name").unwrap_or_default(),
        "configId": row.get::<_, Option<i64>>("config_id"),
        "path": row.get::<_, String>("path"),
        "url": row.get::<_, String>("url"),
        "type": row.get::<_, Option<String>>("type"),
        "size": row.get::<_, i32>("size"),
        "createTime": row.get::<_, chrono::NaiveDateTime>("create_time").to_string(),
    })
}

fn db_error(error: tokio_postgres::Error) -> String {
    format!("文件数据库操作失败：{error}")
}
