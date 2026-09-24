use std::collections::BTreeMap;

use serde_json::{Map, Value, json};
use tokio_postgres::types::ToSql;

use crate::AppState;

#[derive(Clone)]
pub(crate) enum Param {
    Text(String),
    I16(i16),
    I32(i32),
    I64(i64),
    Float(f64),
    Bool(bool),
    Timestamp(chrono::NaiveDateTime),
    #[expect(dead_code, reason = "保留日期参数类型供后续日期字段使用")]
    Date(chrono::NaiveDate),
    Null,
}

impl Param {
    pub(crate) fn as_sql(&self) -> &(dyn ToSql + Sync) {
        match self {
            Self::Text(value) => value,
            Self::I16(value) => value,
            Self::I32(value) => value,
            Self::I64(value) => value,
            Self::Float(value) => value,
            Self::Bool(value) => value,
            Self::Timestamp(value) => value,
            Self::Date(value) => value,
            Self::Null => &Option::<String>::None,
        }
    }
}

pub(crate) fn sql_refs(params: &[Param]) -> Vec<&(dyn ToSql + Sync)> {
    params.iter().map(Param::as_sql).collect()
}

#[derive(Clone, Copy)]
pub(crate) struct Entity {
    pub route: &'static str,
    pub table: &'static str,
    pub columns: &'static [&'static str],
    pub filters: &'static [&'static str],
}

pub(crate) const ENTITIES: &[Entity] = &[
    Entity {
        route: "account-payment",
        table: "biz_account_payment_status_table",
        columns: &[
            "username",
            "payment_amount",
            "payment_time",
            "expiration_time",
            "order_number",
            "account_type",
            "business_description",
            "order_payment_status",
        ],
        filters: &[
            "username",
            "order_number",
            "account_type",
            "order_payment_status",
        ],
    },
    Entity {
        route: "block-retention-ledger",
        table: "boxun_wtsj_block_retention_ledger",
        columns: &[
            "fk_sh_id",
            "production_date",
            "building_no",
            "concat_position",
            "strength_grade",
            "impermeability_level",
            "sum_volume",
            "number_of_standard_curing_specimen_groups",
            "number_of_specimens_in_the_same_culture",
            "number_of_demolding_specimen_groups",
            "number_of_sets_of_impermeable_test_pieces",
            "age",
            "testing_conclusion",
            "print_flag",
            "sample_count",
            "sample_type",
            "entrustment_number",
            "sy_bs",
            "attachment_url",
            "sample_status",
            "qr_code",
            "report_number",
            "test_date",
            "report_date",
            "project_id",
            "project_name",
            "engineering_location",
            "entrusting_unit",
            "manufacturer",
            "model",
            "representative_quantity",
            "representative_batch",
            "test_method",
            "detection_category",
            "description",
            "quality_level",
            "principal",
            "sample_sender",
            "witness",
            "witness_unit",
        ],
        filters: &[
            "project_id",
            "project_name",
            "building_no",
            "concat_position",
            "strength_grade",
            "sy_bs",
        ],
    },
    Entity {
        route: "bystander-record",
        table: "boxun_bystander_record",
        columns: &[
            "fk_sh_id",
            "fk_wt_id",
            "key_parts_of_bystanders",
            "construction_unit",
            "start_time_of_side_station",
            "end_time_of_side_station",
            "pouring_location",
            "number_of_carpenters",
            "number_of_steel_workers",
            "number_of_concrete_workers",
            "number_of_technical_personnel",
            "number_of_automobile_pumps",
            "number_of_vibrating_rods",
            "name_of_commercial_mixing_station",
            "strength_grade",
            "sum_volume",
            "number_of_standard_curing_specimen_groups",
            "number_of_specimens_in_the_same_culture",
            "number_of_demolding_specimen_groups",
            "number_of_sets_of_impermeable_test_pieces",
            "date_of_bystander",
            "print_flag",
            "attachment_url",
        ],
        filters: &[
            "fk_wt_id",
            "pouring_location",
            "strength_grade",
            "date_of_bystander",
        ],
    },
    Entity {
        route: "commercial-concrete-ledger",
        table: "boxun_wtsj_commercial_concrete_ledger",
        columns: &[
            "project_id",
            "material_name",
            "strength_grade",
            "manufacturer",
            "production_date",
            "building_no",
            "concat_position",
            "production_quantity",
            "sum_volume",
            "unit_price",
            "order_number",
            "sample_id",
            "testing_conclusion",
            "print_flag",
            "sample_count",
            "sample_type",
            "entrustment_number",
            "sy_bs",
            "attachment_url",
            "sample_status",
            "qr_code",
            "report_number",
        ],
        filters: &[
            "project_id",
            "material_name",
            "strength_grade",
            "building_no",
            "concat_position",
            "sy_bs",
        ],
    },
    Entity {
        route: "commission-order",
        table: "boxun_wtsj_commission_order",
        columns: &[
            "project_id",
            "project_name",
            "project_name_with_lou",
            "material_name",
            "inspection_department",
            "commission_date",
            "inspection_basis",
            "construction_organization",
            "sample_type",
            "entrustment_number",
            "sy_bs",
            "fk_sk_id",
            "attachment_url",
            "print_flag",
            "report_number",
            "test_date",
            "report_date",
            "sample_status",
            "sample_count",
            "sample_id",
            "sample_name",
            "testing_conclusion",
            "manufacturer",
            "model",
            "representative_quantity",
            "representative_batch",
            "test_method",
        ],
        filters: &[
            "project_id",
            "project_name",
            "entrustment_number",
            "construction_organization",
            "commission_date",
            "sy_bs",
        ],
    },
    Entity {
        route: "commission-order-sample",
        table: "boxun_wtsj_commission_order_sample",
        columns: &[
            "order_id",
            "sample_name",
            "sample_type",
            "sample_count",
            "sample_status",
            "sample_id",
            "testing_conclusion",
            "attachment_url",
            "print_flag",
            "report_number",
        ],
        filters: &["order_id", "sample_name", "sample_type", "sample_status"],
    },
    Entity {
        route: "concrete-construction-record",
        table: "boxun_wtsj_concrete_construction_record",
        columns: &[
            "fk_sk_id",
            "production_date",
            "concat_position",
            "pouring_position",
            "construction_unit",
            "name_of_commercial_mixing_station",
            "strength_grade",
            "sum_volume",
            "print_flag",
            "building_no",
            "weather",
            "temperature",
            "pouring_time",
            "slump",
            "sample_count",
            "sample_type",
            "entrustment_number",
            "sy_bs",
            "attachment_url",
            "sample_status",
            "qr_code",
            "report_number",
        ],
        filters: &[
            "fk_sk_id",
            "project_id",
            "building_no",
            "pouring_position",
            "strength_grade",
        ],
    },
    Entity {
        route: "delegation-order-context",
        table: "boxun_delegation_order_context",
        columns: &[
            "pid",
            "header",
            "material_name",
            "inspection_basis",
            "normative_encoding",
            "sort_no",
            "note1",
            "note2",
            "note3",
            "note4",
        ],
        filters: &["pid", "material_name", "inspection_basis"],
    },
    Entity {
        route: "experimental-report",
        table: "biz_experimental_report",
        columns: &[
            "wtd_sample_id",
            "project_id",
            "sample_id",
            "commission_number",
            "report_number",
            "project_name",
            "project_number",
            "engineering_location",
            "sample_number",
            "sample_name",
            "sample_name4bg",
            "entrusting_unit",
            "sg_unit",
            "js_unit",
            "witness_unit",
            "witness",
            "principal",
            "sample_sender",
            "entrustment_date",
            "test_date",
            "report_date",
            "production_date",
            "make_date",
            "representative_quantity",
            "representative_batch",
            "model",
            "type",
            "strength_grade",
            "test_method",
            "detection_category",
            "description",
            "manufacturer",
            "quality_level",
            "sample_quantity",
            "attachment_url",
        ],
        filters: &[
            "project_id",
            "report_number",
            "sample_name",
            "project_name",
            "strength_grade",
        ],
    },
    Entity {
        route: "historical-weather",
        table: "boxun_historical_weather",
        columns: &[
            "weather_date",
            "week",
            "maximum_temperature",
            "lowest_temperature",
            "morning",
            "afternoon",
            "wind_direction",
            "air_quality_index",
            "accumulated_temperature",
            "area_id",
            "area_type",
        ],
        filters: &["weather_date", "area_id", "area_type"],
    },
    Entity {
        route: "project-company",
        table: "boxun_project_conpany",
        columns: &[
            "unit_name",
            "unit_type",
            "social_credit_code",
            "address",
            "contact_person",
            "phone",
            "id_card",
            "legal_person_name",
            "legal_person_phone",
            "legal_person_card",
            "company_logo",
            "enable_or_not",
            "project_id",
        ],
        filters: &["project_id", "unit_name", "unit_type"],
    },
    Entity {
        route: "project-info",
        table: "boxun_project_info",
        columns: &[
            "project_name",
            "project_type",
            "start_date",
            "completion_date",
            "engineering_progress",
            "area_code",
            "area_name",
            "project_address",
            "name_of_inspection_company",
            "address_of_the_inspection_unit",
            "email_address_of_the_inspection_unit",
        ],
        filters: &[
            "project_name",
            "project_type",
            "area_code",
            "engineering_progress",
        ],
    },
    Entity {
        route: "property-info",
        table: "boxun_property_info",
        columns: &["fk_project_id", "pid", "name", "type", "axis", "lp_index"],
        filters: &["fk_project_id", "pid", "name", "type"],
    },
    Entity {
        route: "raw-material-ledger",
        table: "boxun_mobilization_of_raw_materials",
        columns: &[
            "project_id",
            "sample_id",
            "entrustment_date",
            "entry_date",
            "strength_grade",
            "manufacturer",
            "furnace_batch_number",
            "building_no",
            "concat_position",
            "representative_batch",
            "number_of_pieces",
            "batch_number",
            "remarks",
            "test_number",
            "testing_conclusion",
            "sy_bs",
        ],
        filters: &[
            "project_id",
            "manufacturer",
            "building_no",
            "concat_position",
            "batch_number",
            "sy_bs",
        ],
    },
    Entity {
        route: "template-management",
        table: "boxun_my_template_management",
        columns: &[
            "project_id",
            "enable_or_not",
            "template_url",
            "template_description",
            "template_type",
            "bytecode_ref",
        ],
        filters: &["project_id", "template_type", "enable_or_not"],
    },
    Entity {
        route: "witness-record",
        table: "boxun_witness_record",
        columns: &[
            "fk_wt_id",
            "project_name",
            "sample_name",
            "building_no",
            "sampling_date",
            "witness_sampling_instructions",
            "print_flag",
            "witness",
            "witness_unit",
            "construction_unit",
            "sampling_location",
            "attachment_url",
        ],
        filters: &[
            "fk_wt_id",
            "project_name",
            "sample_name",
            "building_no",
            "sampling_date",
        ],
    },
];

pub(crate) fn entity(route: &str) -> Option<&'static Entity> {
    ENTITIES.iter().find(|entity| entity.route == route)
}

pub(crate) async fn handle(
    state: &AppState,
    method: &str,
    route: &str,
    operation: &str,
    query: &BTreeMap<String, Vec<String>>,
    body: Value,
) -> Result<Value, String> {
    let entity = entity(route).ok_or_else(|| "未知的业务模块".to_owned())?;
    match (method, operation) {
        ("GET", "page") => page(state, entity, query).await,
        ("GET", "get") => get(state, entity, query).await,
        ("GET", "simple-list") | ("GET", "list") | ("GET", "list-all") | ("GET", "list-simple") => {
            simple_list(state, entity, query).await
        }
        ("POST", "list-by-condition") => list_by_condition(state, entity, body).await,
        ("POST", "create") | ("POST", "upsert") => create(state, entity, body).await,
        ("PUT", "update") => update(state, entity, body).await,
        ("DELETE", "delete") => delete(state, entity, query).await,
        ("DELETE", "delete-list") => delete_list(state, entity, query).await,
        _ => Err(format!("未实现的业务操作：{method} {route}/{operation}")),
    }
}

async fn page(
    state: &AppState,
    entity: &Entity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let page_no = int_query(query, "pageNo")?.unwrap_or(1).max(1);
    let page_size = int_query(query, "pageSize")?.unwrap_or(10).clamp(1, 500);
    let (where_sql, params) = filters(entity, query)?;
    let count_sql = format!(
        "SELECT count(*)::bigint FROM {} {where_sql}",
        quote(entity.table)
    );
    let count_refs = sql_refs(&params);
    let total: i64 = state
        .database
        .query_one(&count_sql, &count_refs)
        .await
        .map_err(db_error)?
        .get(0);
    let mut list_params = params;
    list_params.push(Param::I64(page_size));
    list_params.push(Param::I64((page_no - 1) * page_size));
    let sql = format!(
        "SELECT * FROM {} {where_sql} ORDER BY id DESC LIMIT ${} OFFSET ${}",
        quote(entity.table),
        list_params.len() - 1,
        list_params.len()
    );
    let refs = sql_refs(&list_params);
    let rows = state.database.query(&sql, &refs).await.map_err(db_error)?;
    Ok(json!({"list": rows.iter().map(row_value).collect::<Vec<_>>(), "total": total}))
}

async fn get(
    state: &AppState,
    entity: &Entity,
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
    entity: &Entity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let (where_sql, params) = filters(entity, query)?;
    let sql = format!(
        "SELECT * FROM {} {where_sql} ORDER BY id DESC LIMIT 500",
        quote(entity.table)
    );
    let refs = sql_refs(&params);
    let rows = state.database.query(&sql, &refs).await.map_err(db_error)?;
    Ok(Value::Array(rows.iter().map(row_value).collect()))
}

async fn list_by_condition(
    state: &AppState,
    entity: &Entity,
    body: Value,
) -> Result<Value, String> {
    let map = body
        .as_object()
        .ok_or_else(|| "查询条件必须是 JSON 对象".to_owned())?;
    let query = map
        .iter()
        .map(|(key, value)| (key.clone(), values(value)))
        .collect::<BTreeMap<_, _>>();
    simple_list(state, entity, &query).await
}

async fn create(state: &AppState, entity: &Entity, body: Value) -> Result<Value, String> {
    let map = body
        .as_object()
        .ok_or_else(|| "请求体必须是 JSON 对象".to_owned())?;
    let mut params = Vec::<Param>::new();
    let mut columns = vec!["id".to_owned(), "create_time".to_owned()];
    let mut next_id: i64 = state
        .database
        .query_one(
            &format!("SELECT COALESCE(MAX(id),0)+1 FROM {}", quote(entity.table)),
            &[],
        )
        .await
        .map_err(db_error)?
        .get(0);
    next_id = next_id.max(1);
    params.push(Param::I64(next_id));
    params.push(Param::Timestamp(chrono::Utc::now().naive_utc()));
    let column_types = column_types(state, entity).await?;
    for column in entity.columns {
        if let Some(value) = map.get(*column) {
            columns.push((*column).to_owned());
            params.push(json_param(
                value,
                column_types.get(*column).map(String::as_str),
            )?);
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
    let refs = sql_refs(&params);
    let row = state
        .database
        .query_one(&sql, &refs)
        .await
        .map_err(db_error)?;
    Ok(row_value(&row))
}

async fn update(state: &AppState, entity: &Entity, body: Value) -> Result<Value, String> {
    let map = body
        .as_object()
        .ok_or_else(|| "请求体必须是 JSON 对象".to_owned())?;
    let id = map
        .get("id")
        .and_then(value_as_i64)
        .ok_or_else(|| "缺少 id".to_owned())?;
    let mut params = Vec::<Param>::new();
    let mut assignments = vec!["update_time=$1".to_owned()];
    params.push(Param::Timestamp(chrono::Utc::now().naive_utc()));
    let column_types = column_types(state, entity).await?;
    for column in entity.columns {
        if let Some(value) = map.get(*column) {
            params.push(json_param(
                value,
                column_types.get(*column).map(String::as_str),
            )?);
            assignments.push(format!("{}=${}", quote(column), params.len()));
        }
    }
    params.push(Param::I64(id));
    let sql = format!(
        "UPDATE {} SET {} WHERE id=${} RETURNING *",
        quote(entity.table),
        assignments.join(","),
        params.len()
    );
    let refs = sql_refs(&params);
    let row = state
        .database
        .query_opt(&sql, &refs)
        .await
        .map_err(db_error)?;
    Ok(row.as_ref().map(row_value).unwrap_or(Value::Null))
}

async fn delete(
    state: &AppState,
    entity: &Entity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = required_id(query)?;
    let sql = format!("DELETE FROM {} WHERE id=$1", quote(entity.table));
    Ok(json!(
        state
            .database
            .execute(&sql, &[&id])
            .await
            .map_err(db_error)?
    ))
}

async fn delete_list(
    state: &AppState,
    entity: &Entity,
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

fn filters(
    entity: &Entity,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<(String, Vec<Param>), String> {
    let mut clauses = Vec::new();
    let mut params = Vec::<Param>::new();
    for key in entity.filters {
        let Some(value) = query
            .get(*key)
            .and_then(|values| values.first())
            .filter(|value| !value.is_empty())
        else {
            continue;
        };
        params.push(Param::Text(value.clone()));
        clauses.push(format!("{} = ${}", quote(key), params.len()));
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

pub(crate) fn row_value(row: &tokio_postgres::Row) -> Value {
    let mut object = Map::new();
    for (index, column) in row.columns().iter().enumerate() {
        let name = column.name().to_owned();
        let value = column_value_at(row, index, column.type_().name());
        object.insert(name, value);
    }
    Value::Object(object)
}

pub(crate) fn column_value(row: &tokio_postgres::Row, name: &str) -> Value {
    let Some(index) = row
        .columns()
        .iter()
        .position(|column| column.name() == name)
    else {
        return Value::Null;
    };
    column_value_at(row, index, row.columns()[index].type_().name())
}

fn column_value_at(row: &tokio_postgres::Row, index: usize, data_type: &str) -> Value {
    let value = match data_type {
        "int2" => row
            .try_get::<_, Option<i16>>(index)
            .ok()
            .flatten()
            .map(Value::from),
        "int4" => row
            .try_get::<_, Option<i32>>(index)
            .ok()
            .flatten()
            .map(Value::from),
        "int8" => row
            .try_get::<_, Option<i64>>(index)
            .ok()
            .flatten()
            .map(Value::from),
        "float4" => row
            .try_get::<_, Option<f32>>(index)
            .ok()
            .flatten()
            .map(|v| json!(v)),
        "float8" => row
            .try_get::<_, Option<f64>>(index)
            .ok()
            .flatten()
            .map(Value::from),
        "bool" => row
            .try_get::<_, Option<bool>>(index)
            .ok()
            .flatten()
            .map(Value::from),
        "date" => row
            .try_get::<_, Option<chrono::NaiveDate>>(index)
            .ok()
            .flatten()
            .map(|value| Value::String(value.to_string())),
        "timestamp" => row
            .try_get::<_, Option<chrono::NaiveDateTime>>(index)
            .ok()
            .flatten()
            .map(|value| Value::String(value.to_string())),
        "timestamptz" => row
            .try_get::<_, Option<chrono::DateTime<chrono::Utc>>>(index)
            .ok()
            .flatten()
            .map(|value| Value::String(value.to_rfc3339())),
        _ => row
            .try_get::<_, Option<String>>(index)
            .ok()
            .flatten()
            .map(Value::from),
    };
    value.unwrap_or(Value::Null)
}

fn quote(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

fn values(value: &Value) -> Vec<String> {
    match value {
        Value::Array(items) => items.iter().filter_map(value_as_string).collect(),
        _ => value_as_string(value).into_iter().collect(),
    }
}

fn value_as_string(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}

fn value_as_i64(value: &Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_str()?.parse().ok())
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

async fn column_types(
    state: &AppState,
    entity: &Entity,
) -> Result<BTreeMap<String, String>, String> {
    let rows = state
        .database
        .query(
            "SELECT column_name, data_type FROM information_schema.columns WHERE table_schema='public' AND table_name=$1",
            &[&entity.table],
        )
        .await
        .map_err(db_error)?;
    Ok(rows
        .into_iter()
        .map(|row| (row.get::<_, String>(0), row.get::<_, String>(1)))
        .collect())
}

pub(crate) fn json_param(value: &Value, data_type: Option<&str>) -> Result<Param, String> {
    match value {
        Value::Null => Ok(Param::Null),
        Value::String(value) => Ok(Param::Text(value.clone())),
        Value::Bool(value) => Ok(Param::Bool(*value)),
        Value::Number(value) => match data_type {
            Some("smallint") => Ok(Param::I16(
                value
                    .as_i64()
                    .and_then(|value| i16::try_from(value).ok())
                    .ok_or_else(|| "smallint 数值越界".to_owned())?,
            )),
            Some("integer") => Ok(Param::I32(
                value
                    .as_i64()
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| "integer 数值越界".to_owned())?,
            )),
            Some("bigint") => Ok(Param::I64(
                value.as_i64().ok_or_else(|| "bigint 数值无效".to_owned())?,
            )),
            _ if value.is_i64() => Ok(Param::I64(value.as_i64().unwrap())),
            _ => Ok(Param::Float(
                value.as_f64().ok_or_else(|| "数值无效".to_owned())?,
            )),
        },
        other => Ok(Param::Text(other.to_string())),
    }
}

fn db_error(error: tokio_postgres::Error) -> String {
    format!("数据库操作失败：{error}")
}
