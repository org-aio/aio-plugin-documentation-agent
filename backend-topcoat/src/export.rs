use std::collections::BTreeMap;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use rust_xlsxwriter::Workbook;
use serde_json::{Value, json};
use tokio_postgres::types::ToSql;

use crate::AppState;
use crate::crud::{Entity, Param, entity};

pub(crate) async fn export(
    state: &AppState,
    route: &str,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let entity = entity(route).ok_or_else(|| "未知的导出模块".to_owned())?;
    let (where_sql, params) = filters(entity, query)?;
    let sql = format!(
        "SELECT * FROM {} {where_sql} ORDER BY id DESC LIMIT 10000",
        quote(entity.table)
    );
    let rows = state
        .database
        .query(&sql, &sql_refs(&params))
        .await
        .map_err(db_error)?;
    let columns = rows
        .first()
        .map(|row| {
            row.columns()
                .iter()
                .map(|column| column.name().to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            entity
                .columns
                .iter()
                .map(|column| (*column).to_owned())
                .collect()
        });
    let bytes = workbook_bytes("数据", &columns, &rows)?;
    Ok(json!({
        "base64": STANDARD.encode(bytes),
        "contentType": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "fileName": format!("{}.xlsx", route.replace('/', "_"))
    }))
}

pub(crate) async fn export_system(
    state: &AppState,
    route: &str,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let (table, columns) =
        crate::system::entity_for_export(route).ok_or_else(|| "未知的系统导出模块".to_owned())?;
    let rows = state
        .database
        .query(
            &format!(
                "SELECT * FROM {} ORDER BY id DESC LIMIT 10000",
                quote(table)
            ),
            &[],
        )
        .await
        .map_err(db_error)?;
    let columns = rows
        .first()
        .map(|row| {
            row.columns()
                .iter()
                .map(|column| column.name().to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| columns.iter().map(|column| (*column).to_owned()).collect());
    let bytes = workbook_bytes("数据", &columns, &rows)?;
    let _ = query;
    Ok(json!({
        "base64": STANDARD.encode(bytes),
        "contentType": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "fileName": format!("{}.xlsx", route.replace('/', "_"))
    }))
}

fn workbook_bytes(
    sheet_name: &str,
    columns: &[String],
    rows: &[tokio_postgres::Row],
) -> Result<Vec<u8>, String> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet
        .set_name(sheet_name)
        .map_err(|error| format!("创建 Excel 工作表失败：{error}"))?;
    for (column, name) in columns.iter().enumerate() {
        sheet
            .write_string(0, column as u16, name)
            .map_err(|error| format!("写入 Excel 表头失败：{error}"))?;
    }
    for (row_index, row) in rows.iter().enumerate() {
        for (column, name) in columns.iter().enumerate() {
            let value = crate::crud::column_value(row, name);
            let text = match value {
                Value::Null => String::new(),
                Value::String(value) => value,
                other => other.to_string(),
            };
            sheet
                .write_string((row_index + 1) as u32, column as u16, &text)
                .map_err(|error| format!("写入 Excel 单元格失败：{error}"))?;
        }
    }
    workbook
        .save_to_buffer()
        .map_err(|error| format!("生成 Excel 失败：{error}"))
}

fn filters(
    entity: &Entity,
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

fn sql_refs(params: &[Param]) -> Vec<&(dyn ToSql + Sync)> {
    params.iter().map(Param::as_sql).collect()
}

fn quote(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

fn db_error(error: tokio_postgres::Error) -> String {
    format!("导出数据库操作失败：{error}")
}
