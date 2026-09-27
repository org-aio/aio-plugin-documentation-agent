use std::collections::BTreeMap;

use crate::auth::Authenticated;
use crate::{AppState, api};
use chrono::{Datelike, NaiveDate};
use pinyin::ToPinyin;
use serde_json::{Value, json};

const RAW_MATERIAL_MOCK_ROWS: usize = 15;

struct RawMaterialEntrustRequest {
    ids: Vec<i64>,
    project_id: Option<String>,
}

struct RawMaterialRecord {
    id: i64,
    sample_id: Option<String>,
    entry_date: Option<NaiveDate>,
    strength_grade: Option<String>,
    concat_position: Option<String>,
    furnace_batch_number: Option<String>,
    manufacturer: Option<String>,
    representative_batch: Option<String>,
}

struct RawMaterialMockRow {
    sample_id: String,
    entry_date: NaiveDate,
    strength_grade: String,
    manufacturer: &'static str,
    furnace_batch_number: String,
    building_no: &'static str,
    concat_position: &'static str,
    representative_batch: &'static str,
    batch_number: String,
}

pub(crate) async fn handle(
    state: &AppState,
    user: &Authenticated,
    method: &str,
    route: &str,
    operation: &str,
    query: &BTreeMap<String, Vec<String>>,
    body: Value,
) -> Result<Value, String> {
    match (route, method, operation) {
        ("homepage", "POST", "inspection-today") => inspection_today(state, body).await,
        ("homepage", "POST", "sample-names-by-type") => sample_names(state, body, false).await,
        ("homepage", "POST", "sample-names-by-progress-type") => {
            sample_names(state, body, true).await
        }
        ("homepage", "POST", "various-progress") => various_progress(state, body).await,
        ("project-info", "GET", "get-all-project") => simple_projects(state).await,
        ("project-info", "POST", "get-all-sj-project") => all_sj_projects(state).await,
        ("project-info", "GET", "query-project-company-by-main-id") => {
            project_companies(state, query).await
        }
        ("project-info", "GET", "current-project-companies") => {
            current_project_companies(state, query).await
        }
        ("project-company", "GET", "simple-list") => project_companies(state, query).await,
        ("commission-order", "POST", "issue-an-order") => issue_orders(state, body).await,
        ("commission-order", "GET", "query-commission-order-sample-by-main-id") => {
            commission_samples(state, query).await
        }
        ("witness-record", "POST", "generate-from-commission-order")
        | ("bystander-record", "POST", "generate-from-commission-order")
        | ("concrete-construction-record", "POST", "generate-from-commission-order") => {
            generate_record_from_order(state, route, body).await
        }
        ("commission-order", "GET", "download-attachment") => {
            commission_attachment(state, query).await
        }
        ("commission-order", "POST", "batch-download-generated") => {
            commission_batch_zip(state, body).await
        }
        ("delegation-order-context", "GET", "load-tree-data") => delegation_tree(state).await,
        ("block-retention-ledger", "POST", "entrust-the-test-block") => {
            entrust_blocks(state, query, body).await
        }
        ("block-retention-ledger", "POST", "evaluate-strength") => evaluate_strength(body),
        ("block-retention-ledger", "POST", "generate-multi-kit") => {
            generate_kits(state, query).await
        }
        ("block-retention-ledger", "POST", "gen-and-send-email") => {
            send_commission_email(state, body).await
        }
        ("raw-material-ledger", "POST", "to-entrust-raw-materials") => {
            entrust_raw_materials(state, body).await
        }
        ("raw-material-ledger", "POST", "add-batch") => add_raw_materials(state, user, body).await,
        ("raw-material-ledger", "GET", "raw-material-pull-down") => {
            raw_material_pull_down(state).await
        }
        ("raw-material-ledger", "POST" | "GET", "data-generation") => {
            raw_material_data(state, query, &body).await
        }
        ("raw-material-ledger", "POST", "generate-various-raw-material-records") => {
            generate_raw_material_witness_records(state, user, body).await
        }
        ("commercial-concrete-ledger", "POST", "add-batch") => {
            add_commercial_ledgers(state, body).await
        }
        (
            "commercial-concrete-ledger" | "block-retention-ledger",
            "GET",
            "specimen-group-numbers",
        ) => specimen_group_numbers(query),
        ("commercial-concrete-ledger", "POST", "generate-various-records") => {
            generate_commercial_records(state, body, CommercialRecordKind::All).await
        }
        (
            "commercial-concrete-ledger",
            "POST",
            "batch-generation-of-concrete-construction-records",
        ) => generate_all_commercial(state, CommercialRecordKind::Concrete).await,
        ("commercial-concrete-ledger", "POST", "delete-various-records") => {
            delete_commercial_related(state, body).await
        }
        ("commercial-concrete-ledger", "POST", "manually-generate-various-records") => {
            generate_commercial_records(state, body, CommercialRecordKind::All).await
        }
        ("commercial-concrete-ledger", "POST", "mock-sh") => mock_commercial(state, query).await,
        ("commercial-concrete-ledger", "POST", "generate-test-block-retention-account") => {
            generate_all_commercial(state, CommercialRecordKind::Block).await
        }
        ("commercial-concrete-ledger", "POST", "generate-side-station-records") => {
            generate_all_commercial(state, CommercialRecordKind::Bystander).await
        }
        ("commercial-concrete-ledger", "POST", "delete-by-project-id") => {
            delete_commercial_by_project(state, query).await
        }
        ("template-management", "POST", "inherit-templates")
        | ("template-management", "GET", "inherit-templates") => {
            inherit_templates(state, query).await
        }
        ("template-management", "GET", "field-mapping") => field_mapping(query),
        ("historical-weather", "GET", "areas") => weather_areas().await,
        ("historical-weather", "GET", "area-name") => weather_area_name(query).await,
        ("historical-weather", "GET", "page-view") => weather_page_view(state, query).await,
        ("historical-weather", "GET", "view-specified-weather") => {
            weather_by_date(state, query).await
        }
        ("historical-weather", "POST", "sync-weather-year") => sync_weather_year(state, body).await,
        ("historical-weather", "POST", "sync-weather-month") => {
            sync_weather_month(state, body).await
        }
        ("historical-weather", "POST", "sync-recent-years") => {
            sync_weather_recent(state, query).await
        }
        ("historical-weather", "POST", "test-cloud-message") => {
            test_weather_source(state, query).await
        }
        ("block-retention-ledger", "GET", "gen-ttj-wtd") => {
            block_sheet(state, query, "同养委托单", "block-retention-ledger").await
        }
        ("block-retention-ledger", "GET", "gen-cm-wtd") => {
            block_sheet(state, query, "拆模委托单", "block-retention-ledger").await
        }
        ("block-retention-ledger", "GET", "download-various-packages") => {
            block_sheet(state, query, "试块套件", "block-retention-ledger").await
        }
        ("account-payment", "GET", "account-price-list") => Ok(account_price_list()),
        ("channel", "POST", "push") => {
            let message = body
                .as_str()
                .map(str::to_owned)
                .or_else(|| {
                    body.get("message")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                })
                .ok_or_else(|| "频道消息不能为空".to_owned())?;
            // 原实现允许没有等待者时正常返回 0。
            let delivered = crate::push_channel(&state.channel, message).await;
            Ok(json!({ "delivered": delivered }))
        }
        ("channel", "GET", "poll") => crate::poll_channel(&state.channel).await,
        _ => Err(format!("未实现的业务动作：{method} {route}/{operation}")),
    }
}

async fn send_commission_email(state: &AppState, body: Value) -> Result<Value, String> {
    let ids = body
        .as_array()
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .unwrap_or_default();
    if ids.is_empty() {
        return Err("请选择委托单".into());
    }
    let account = state
        .database
        .query_opt(
            "SELECT mail,username,password,host,port,ssl_enable,starttls_enable FROM system_mail_account ORDER BY id LIMIT 1",
            &[],
        )
        .await
        .map_err(db_error)?
        .ok_or_else(|| "请先在邮件账号中配置 SMTP 服务器".to_owned())?;
    let recipient = state
        .database
        .query_opt(
            "SELECT p.email_address_of_the_inspection_unit FROM boxun_wtsj_commission_order o JOIN boxun_project_info p ON p.id::text=o.project_id WHERE o.id=ANY($1) AND p.email_address_of_the_inspection_unit IS NOT NULL AND p.email_address_of_the_inspection_unit <> '' LIMIT 1",
            &[&ids],
        )
        .await
        .map_err(db_error)?
        .and_then(|row| row.get::<_, Option<String>>(0))
        .ok_or_else(|| "请配置送检单位邮箱".to_owned())?;
    let mail_from = account
        .get::<_, Option<String>>("mail")
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "邮件账号缺少发件邮箱".to_owned())?;
    let host: String = account.get("host");
    let port = account.get::<_, i32>("port").max(1) as u16;
    let username = account.get::<_, Option<String>>("username");
    let password = account.get::<_, Option<String>>("password");
    let starttls = account
        .get::<_, Option<bool>>("starttls_enable")
        .unwrap_or(false);
    let ssl = account
        .get::<_, Option<bool>>("ssl_enable")
        .unwrap_or(false);
    let mut builder = if starttls {
        topcoat::mail::SmtpTransport::starttls(&host)
    } else if ssl {
        topcoat::mail::SmtpTransport::relay(&host)
    } else {
        Ok(topcoat::mail::SmtpTransport::unencrypted(&host))
    }
    .map_err(|error| format!("SMTP 配置无效：{error}"))?
    .port(port);
    if let (Some(username), Some(password)) = (username.as_deref(), password.as_deref()) {
        builder = builder.credentials(username, password);
    }
    let transport = builder.build();
    let subject = "工程委托单已生成";
    let content = format!("已生成 {} 份委托单，请查收。", ids.len());
    let mail = topcoat::mail::Mail::builder()
        .from(
            topcoat::mail::Mailbox::new(&mail_from)
                .map_err(|error| format!("发件邮箱无效：{error}"))?,
        )
        .to(vec![
            topcoat::mail::Mailbox::new(&recipient)
                .map_err(|error| format!("收件邮箱无效：{error}"))?,
        ])
        .subject(subject)
        .text(content)
        .build();
    use topcoat::mail::Transport;
    transport
        .send(&topcoat::context::Cx::default(), mail)
        .await
        .map_err(|error| format!("邮件发送失败：{error}"))?;
    update_status_by_ids(
        state,
        "boxun_wtsj_commission_order",
        "sy_bs",
        "已送检",
        &ids,
    )
    .await?;
    Ok(Value::String(mail_from))
}

async fn generate_record_from_order(
    state: &AppState,
    route: &str,
    body: Value,
) -> Result<Value, String> {
    let order_id = body
        .get("commissionOrderId")
        .and_then(value_as_i64)
        .ok_or_else(|| "缺少 commissionOrderId".to_owned())?;
    let order = state
        .database
        .query_opt(
            "SELECT * FROM boxun_wtsj_commission_order WHERE id=$1",
            &[&order_id],
        )
        .await
        .map_err(db_error)?
        .ok_or_else(|| "委托单不存在".to_owned())?;
    let record_id = match route {
        "witness-record" => {
            let id = next_id(state, "boxun_witness_record").await?;
            state.database.execute(
                "INSERT INTO boxun_witness_record (create_time,id,fk_wt_id,project_name,sample_name,building_no,sampling_date,witness_sampling_instructions,print_flag) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,'0')",
                &[&id, &order_id.to_string(), &order.get::<_, Option<String>>("project_name"), &order.get::<_, Option<String>>("material_name"), &order.get::<_, Option<String>>("project_name_with_lou"), &order.get::<_, Option<chrono::NaiveDate>>("commission_date"), &order.get::<_, Option<String>>("inspection_basis")],
            ).await.map_err(db_error)?;
            id
        }
        "bystander-record" => {
            let id = next_id(state, "boxun_bystander_record").await?;
            state.database.execute(
                "INSERT INTO boxun_bystander_record (create_time,id,fk_wt_id,key_parts_of_bystanders,pouring_location,construction_unit,strength_grade,date_of_bystander,print_flag) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,'0')",
                &[&id, &order_id.to_string(), &order.get::<_, Option<String>>("material_name"), &order.get::<_, Option<String>>("project_name_with_lou"), &order.get::<_, Option<String>>("construction_organization"), &order.get::<_, Option<String>>("inspection_basis"), &order.get::<_, Option<chrono::NaiveDate>>("commission_date")],
            ).await.map_err(db_error)?;
            id
        }
        _ => {
            let id = next_id(state, "boxun_wtsj_concrete_construction_record").await?;
            state.database.execute(
                "INSERT INTO boxun_wtsj_concrete_construction_record (create_time,id,fk_sk_id,project_id,production_date,concat_position,pouring_position,construction_unit,name_of_commercial_mixing_station,print_flag) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,'0')",
                &[&id, &order.get::<_, Option<String>>("fk_sk_id"), &order.get::<_, Option<String>>("project_id"), &order.get::<_, Option<chrono::NaiveDate>>("commission_date"), &order.get::<_, Option<String>>("project_name_with_lou"), &order.get::<_, Option<String>>("project_name_with_lou"), &order.get::<_, Option<String>>("construction_organization"), &order.get::<_, Option<String>>("inspection_department")],
            ).await.map_err(db_error)?;
            id
        }
    };
    Ok(Value::Array(vec![json!(record_id.to_string())]))
}

async fn mock_commercial(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let project_id = query
        .get("projectId")
        .and_then(|values| values.first())
        .cloned()
        .ok_or_else(|| "请选择项目".to_owned())?;
    let mut created = 0;
    for index in 1..=20 {
        let id = next_id(state, "boxun_wtsj_commercial_concrete_ledger").await?;
        let position = if index % 2 == 0 {
            "一层梁板"
        } else {
            "基础底板"
        };
        state.database.execute(
            "INSERT INTO boxun_wtsj_commercial_concrete_ledger (create_time,id,project_id,production_date,building_no,concat_position,strength_grade,manufacturer,production_quantity,sum_volume) VALUES (CURRENT_TIMESTAMP,$1,$2,CURRENT_DATE,$3,$4,'C30','示例搅拌站',30,30)",
            &[&id, &project_id, &format!("{}号楼", index % 3 + 1), &position],
        ).await.map_err(db_error)?;
        created += 1;
    }
    Ok(json!(created))
}

#[derive(Clone, Copy, PartialEq)]
enum CommercialRecordKind {
    All,
    Block,
    Concrete,
    Bystander,
}

async fn generate_all_commercial(
    state: &AppState,
    kind: CommercialRecordKind,
) -> Result<Value, String> {
    let ids = state
        .database
        .query(
            "SELECT id FROM boxun_wtsj_commercial_concrete_ledger ORDER BY id LIMIT 200",
            &[],
        )
        .await
        .map_err(db_error)?
        .into_iter()
        .map(|row| row.get::<_, i64>(0))
        .collect::<Vec<_>>();
    if ids.is_empty() {
        return Ok(json!(0));
    }
    let generated = generate_commercial_records(state, json!({"ids": ids}), kind).await?;
    Ok(generated
        .get(match kind {
            CommercialRecordKind::Block => "blockRetentionLedgerCount",
            CommercialRecordKind::Concrete => "concreteRecordCount",
            CommercialRecordKind::Bystander | CommercialRecordKind::All => "bystanderRecordCount",
        })
        .cloned()
        .unwrap_or_else(|| json!(0)))
}

fn evaluate_strength(body: Value) -> Result<Value, String> {
    let grade = body
        .get("strengthGrade")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_uppercase();
    let standard_value = grade
        .split_once('C')
        .and_then(|(_, value)| {
            let digits = value
                .chars()
                .take_while(|character| character.is_ascii_digit() || *character == '.')
                .collect::<String>();
            digits.parse::<f64>().ok()
        })
        .filter(|value| *value > 0.0)
        .ok_or_else(|| format!("无法从强度等级中解析出标准值：{grade}"))?;
    let values = body
        .get("representativeValues")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_f64)
                .filter(|value| value.is_finite() && *value > 0.0)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if values.is_empty() {
        return Err("至少需要一组有效的抗压强度代表值".into());
    }
    let sample_size = values.len();
    let mean = values.iter().sum::<f64>() / sample_size as f64;
    let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
    let statistical = sample_size >= 10;
    let standard_deviation = statistical.then(|| {
        let variance = values
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / (sample_size - 1) as f64;
        variance.sqrt()
    });
    let (lambda1, lambda2, mean_requirement, minimum_requirement, method, method_label) =
        if statistical {
            let deviation = standard_deviation.unwrap_or(0.0);
            let (lambda1, lambda2) = match sample_size {
                10..=14 => (1.15, 0.90),
                15..=24 => (1.05, 0.85),
                _ => (0.95, 0.85),
            };
            (
                Some(lambda1),
                Some(lambda2),
                mean - lambda1 * deviation,
                lambda2 * standard_value,
                "STATISTICAL_UNKNOWN_SIGMA",
                "统计方法（标准差未知）",
            )
        } else {
            let lambda3 = if standard_value >= 60.0 { 1.10 } else { 1.15 };
            (
                None,
                None,
                lambda3 * standard_value,
                0.95 * standard_value,
                "NON_STATISTICAL",
                "非统计方法",
            )
        };
    let mean_qualified = if statistical {
        mean_requirement >= 0.90 * standard_value
    } else {
        mean >= mean_requirement
    };
    let minimum_qualified = minimum >= minimum_requirement;
    let qualified = mean_qualified && minimum_qualified;
    let lambda3 = (!statistical).then_some(if standard_value >= 60.0 { 1.10 } else { 1.15 });
    let lambda4 = (!statistical).then_some(0.95);
    Ok(json!({
        "strengthGrade": grade,
        "standardValue": standard_value,
        "sampleSize": sample_size,
        "method": method,
        "methodLabel": method_label,
        "representativeValues": values,
        "mean": mean,
        "minimum": minimum,
        "standardDeviation": standard_deviation,
        "lambda1": lambda1,
        "lambda2": lambda2,
        "lambda3": lambda3,
        "lambda4": lambda4,
        "meanRequirement": mean_requirement,
        "minimumRequirement": minimum_requirement,
        "meanQualified": mean_qualified,
        "minimumQualified": minimum_qualified,
        "qualified": qualified,
        "conclusion": if qualified { "合格" } else { "不合格" }
    }))
}

async fn generate_kits(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = query
        .get("id")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| "缺少台账 id".to_owned())?;
    let kit_type = query
        .get("type")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(1);
    let row = state
        .database
        .query_opt(
            "SELECT number_of_standard_curing_specimen_groups,number_of_sets_of_impermeable_test_pieces,number_of_specimens_in_the_same_culture,number_of_demolding_specimen_groups FROM boxun_wtsj_block_retention_ledger WHERE id=$1",
            &[&id],
        )
        .await
        .map_err(db_error)?
        .ok_or_else(|| "试块留置台账不存在".to_owned())?;
    let count = match kit_type {
        1 => row.get::<_, Option<i32>>(0).unwrap_or(0),
        2 => row.get::<_, Option<i32>>(1).unwrap_or(0),
        3 => row.get::<_, Option<i32>>(2).unwrap_or(0),
        _ => row.get::<_, Option<i32>>(3).unwrap_or(0),
    };
    Ok(Value::Array(
        (1..=count)
            .map(|index| json!(format!("{id}-{kit_type}-{index}")))
            .collect(),
    ))
}

async fn simple_projects(state: &AppState) -> Result<Value, String> {
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_project_info ORDER BY id DESC LIMIT 500",
            &[],
        )
        .await
        .map_err(db_error)?;
    Ok(Value::Array(
        rows.iter().map(crate::crud::row_value).collect(),
    ))
}

async fn all_sj_projects(state: &AppState) -> Result<Value, String> {
    let projects = simple_projects(state).await?;
    let projects = projects.as_array().cloned().unwrap_or_default();
    let mut result = Vec::new();
    for project in projects {
        let project_id = project
            .get("id")
            .and_then(value_as_string)
            .unwrap_or_default();
        let inspection = inspection_today(state, json!({"projectId": project_id})).await?;
        result.push(json!({"project": project, "inspectionToday": inspection}));
    }
    Ok(Value::Array(result))
}

async fn project_companies(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let project_id = query
        .get("id")
        .or_else(|| query.get("projectId"))
        .or_else(|| query.get("mainId"))
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
    let rows = if project_id.is_empty() {
        state
            .database
            .query("SELECT * FROM boxun_project_conpany ORDER BY id DESC LIMIT 500", &[])
            .await
    } else {
        state
            .database
            .query(
                "SELECT * FROM boxun_project_conpany WHERE project_id=$1 ORDER BY id DESC LIMIT 500",
                &[&project_id],
            )
            .await
    }
    .map_err(db_error)?;
    Ok(Value::Array(
        rows.iter().map(crate::crud::row_value).collect(),
    ))
}

async fn current_project_companies(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    project_companies(state, query).await
}

async fn inspection_today(state: &AppState, body: Value) -> Result<Value, String> {
    let project_id = body
        .get("projectId")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if project_id.is_empty() {
        return Ok(empty_inspection());
    }
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_wtsj_block_retention_ledger WHERE project_id=$1 AND production_date BETWEEN CURRENT_DATE-3 AND CURRENT_DATE+3 ORDER BY production_date,id DESC LIMIT 500",
            &[&project_id],
        )
        .await
        .map_err(db_error)?;
    let mut by = Vec::new();
    let mut ks = Vec::new();
    let mut ttj = Vec::new();
    let mut cm = Vec::new();
    for row in rows {
        let value = crate::crud::row_value(&row);
        if row
            .get::<_, Option<i32>>("number_of_standard_curing_specimen_groups")
            .unwrap_or(0)
            > 0
        {
            by.push(value.clone());
        }
        if row
            .get::<_, Option<i32>>("number_of_sets_of_impermeable_test_pieces")
            .unwrap_or(0)
            > 0
        {
            ks.push(value.clone());
        }
        if row
            .get::<_, Option<i32>>("number_of_specimens_in_the_same_culture")
            .unwrap_or(0)
            > 0
        {
            ttj.push(value.clone());
        }
        if row
            .get::<_, Option<i32>>("number_of_demolding_specimen_groups")
            .unwrap_or(0)
            > 0
        {
            cm.push(value);
        }
    }
    Ok(json!({
        "byList": by,
        "ksList": ks,
        "ttjList": ttj,
        "cmList": cm,
        "yclList": [],
        "otherSampleList": []
    }))
}

fn empty_inspection() -> Value {
    json!({"byList": [], "ksList": [], "ttjList": [], "cmList": [], "yclList": [], "otherSampleList": []})
}

async fn sample_names(state: &AppState, body: Value, by_progress: bool) -> Result<Value, String> {
    let sample_type = body.get("sampleType").and_then(value_as_i64);
    let progress_type = body.get("progressType").and_then(value_as_i64);
    let sql = if by_progress {
        match progress_type {
            Some(1) => "SELECT * FROM boxun_delegation_order_context ORDER BY sort_no,id LIMIT 500",
            Some(2) => "SELECT * FROM boxun_delegation_order_context ORDER BY sort_no,id LIMIT 500",
            Some(3) => "SELECT * FROM boxun_delegation_order_context ORDER BY sort_no,id LIMIT 500",
            _ => return Ok(Value::Array(Vec::new())),
        }
    } else if sample_type.is_some() {
        "SELECT * FROM boxun_delegation_order_context WHERE id=$1 ORDER BY sort_no,id LIMIT 500"
    } else {
        "SELECT * FROM boxun_delegation_order_context ORDER BY sort_no,id LIMIT 500"
    };
    let rows = match (by_progress, sample_type) {
        (false, Some(sample_type)) => state.database.query(sql, &[&sample_type]).await,
        _ => state.database.query(sql, &[]).await,
    }
    .map_err(db_error)?;
    Ok(Value::Array(
        rows.iter().map(crate::crud::row_value).collect(),
    ))
}

async fn various_progress(state: &AppState, body: Value) -> Result<Value, String> {
    let project_id = body
        .get("projectId")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let progress_type = body.get("progressType").and_then(value_as_i64).unwrap_or(0);
    if project_id.is_empty() {
        return Ok(Value::Array(Vec::new()));
    }
    match progress_type {
        1 => {
            let rows = state
                .database
                .query(
                    "SELECT building_no,pouring_position,production_date FROM boxun_wtsj_block_retention_ledger WHERE project_id=$1 ORDER BY production_date DESC LIMIT 500",
                    &[&project_id],
                )
                .await
                .map_err(db_error)?;
            let mut groups = BTreeMap::<String, Vec<Value>>::new();
            for row in rows {
                let building = row
                    .get::<_, Option<String>>("building_no")
                    .unwrap_or_default();
                let date = row.get::<_, Option<chrono::NaiveDate>>("production_date");
                groups.entry(building).or_default().push(json!({
                    "buildingNo": row.get::<_, Option<String>>("building_no"),
                    "pouringPosition": row.get::<_, Option<String>>("pouring_position"),
                    "productionDate": date.map(|value| value.to_string()),
                    "productionDateNum": date.map(|value| value.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis()),
                }));
            }
            Ok(Value::Array(
                groups
                    .into_iter()
                    .map(|(x, y)| json!({"x": x, "y": y}))
                    .collect(),
            ))
        }
        2 => progress_items(state, project_id, false).await,
        3 => progress_items(state, project_id, true).await,
        _ => Ok(Value::Array(Vec::new())),
    }
}

async fn progress_items(state: &AppState, project_id: &str, report: bool) -> Result<Value, String> {
    let date_column = if report {
        "report_date"
    } else {
        "commission_date"
    };
    let table = if report {
        "biz_experimental_report"
    } else {
        "boxun_wtsj_commission_order"
    };
    let rows = state
        .database
        .query(
            &format!(
                "SELECT * FROM {table} WHERE project_id=$1 ORDER BY {date_column} DESC LIMIT 500"
            ),
            &[&project_id],
        )
        .await
        .map_err(db_error)?;
    Ok(Value::Array(
        rows.iter()
            .map(|row| {
                json!({
                    "buildingNo": crate::crud::column_value(row, "project_name_with_lou"),
                    "pouringPosition": crate::crud::column_value(row, "material_name"),
                    "commissionDate": crate::crud::column_value(row, "commission_date"),
                    "reportDate": crate::crud::column_value(row, "report_date"),
                    "sampleType": crate::crud::column_value(row, "sample_type"),
                })
            })
            .collect(),
    ))
}

async fn issue_orders(state: &AppState, body: Value) -> Result<Value, String> {
    let ids = body
        .as_array()
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .or_else(|| {
            body.get("ids")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(value_as_i64).collect())
        })
        .unwrap_or_default();
    Ok(json!(
        update_status_by_ids(
            state,
            "boxun_wtsj_commission_order",
            "sy_bs",
            "已送检",
            &ids
        )
        .await?
            > 0
    ))
}

async fn commission_samples(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = query
        .get("id")
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_wtsj_commission_order_sample WHERE fk_wt_id=$1 ORDER BY id",
            &[&id],
        )
        .await
        .map_err(db_error)?;
    Ok(Value::Array(
        rows.iter().map(crate::crud::row_value).collect(),
    ))
}

async fn commission_attachment(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let id = query
        .get("id")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| "缺少委托单 id".to_owned())?;
    let order = state
        .database
        .query_opt(
            "SELECT * FROM boxun_wtsj_commission_order WHERE id=$1",
            &[&id],
        )
        .await
        .map_err(db_error)?
        .ok_or_else(|| "委托单不存在".to_owned())?;
    let samples = state
        .database
        .query(
            "SELECT * FROM boxun_wtsj_commission_order_sample WHERE fk_wt_id=$1 ORDER BY id",
            &[&id.to_string()],
        )
        .await
        .map_err(db_error)?;
    let mut rows = vec![
        vec!["字段".to_owned(), "内容".to_owned()],
        vec!["工程名称".into(), text_column(&order, "project_name")],
        vec!["委托日期".into(), text_column(&order, "commission_date")],
        vec![
            "送检单位".into(),
            text_column(&order, "inspection_department"),
        ],
        vec!["见证单位".into(), text_column(&order, "witness_unit")],
        vec![
            "施工单位".into(),
            text_column(&order, "construction_organization"),
        ],
        vec!["检验依据".into(), text_column(&order, "inspection_basis")],
        vec!["".into(), "".into()],
        vec![
            "试样名称".into(),
            "强度等级".into(),
            "工程部位".into(),
            "成型日期".into(),
            "数量".into(),
        ],
    ];
    for sample in samples {
        rows.push(vec![
            text_column(&sample, "sample_name"),
            text_column(&sample, "strength_grade"),
            text_column(&sample, "engineering_location"),
            text_column(&sample, "forming_date"),
            text_column(&sample, "sample_quantity"),
        ]);
    }
    workbook_envelope("委托单", rows, "commission-order")
}

fn text_column(row: &tokio_postgres::Row, column: &str) -> String {
    match crate::crud::column_value(row, column) {
        Value::Null => String::new(),
        Value::String(value) => value,
        other => other.to_string(),
    }
}

fn workbook_envelope(
    sheet_name: &str,
    rows: Vec<Vec<String>>,
    file_prefix: &str,
) -> Result<Value, String> {
    let mut workbook = rust_xlsxwriter::Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet
        .set_name(sheet_name)
        .map_err(|error| format!("创建生成物工作表失败：{error}"))?;
    for (row_index, row) in rows.iter().enumerate() {
        for (column, value) in row.iter().enumerate() {
            sheet
                .write_string(row_index as u32, column as u16, value)
                .map_err(|error| format!("写入生成物失败：{error}"))?;
        }
    }
    let bytes = workbook
        .save_to_buffer()
        .map_err(|error| format!("生成 Excel 失败：{error}"))?;
    Ok(json!({
        "base64": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes),
        "contentType": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "fileName": format!("{file_prefix}-{sheet_name}.xlsx")
    }))
}

async fn commission_batch_zip(state: &AppState, body: Value) -> Result<Value, String> {
    let ids = body
        .as_array()
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .unwrap_or_default();
    if ids.is_empty() {
        return Err("请选择委托单".into());
    }
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for id in ids {
        let query = [("id".to_owned(), vec![id.to_string()])]
            .into_iter()
            .collect();
        let value = commission_attachment(state, &query).await?;
        let encoded = value
            .get("base64")
            .and_then(Value::as_str)
            .ok_or_else(|| "委托单生成物缺少内容".to_owned())?;
        let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)
            .map_err(|_| "委托单生成物不是有效 base64".to_owned())?;
        zip.start_file(format!("委托单-{id}.xlsx"), options)
            .map_err(|error| format!("写入 ZIP 失败：{error}"))?;
        std::io::Write::write_all(&mut zip, &bytes)
            .map_err(|error| format!("写入 ZIP 内容失败：{error}"))?;
    }
    let bytes = zip
        .finish()
        .map_err(|error| format!("完成 ZIP 失败：{error}"))?
        .into_inner();
    Ok(json!({
        "base64": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes),
        "contentType": "application/zip",
        "fileName": "委托单批量下载.zip"
    }))
}

async fn delegation_tree(state: &AppState) -> Result<Value, String> {
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_delegation_order_context ORDER BY sort_no,id",
            &[],
        )
        .await
        .map_err(db_error)?;
    let mut by_parent = BTreeMap::<String, Vec<Value>>::new();
    for row in &rows {
        let parent = text_column(row, "pid");
        by_parent
            .entry(parent)
            .or_default()
            .push(crate::crud::row_value(row));
    }
    fn build(parent: &str, by_parent: &BTreeMap<String, Vec<Value>>) -> Vec<Value> {
        by_parent
            .get(parent)
            .map(|children| {
                children
                    .iter()
                    .cloned()
                    .map(|mut child| {
                        let id = child
                            .get("id")
                            .and_then(value_as_string)
                            .unwrap_or_default();
                        child["children"] = Value::Array(build(&id, by_parent));
                        child
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    Ok(Value::Array(build("", &by_parent)))
}

async fn block_sheet(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
    label: &str,
    file_prefix: &str,
) -> Result<Value, String> {
    let id = query
        .get("id")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| "缺少试块台账 id".to_owned())?;
    let row = state
        .database
        .query_opt(
            "SELECT * FROM boxun_wtsj_block_retention_ledger WHERE id=$1",
            &[&id],
        )
        .await
        .map_err(db_error)?
        .ok_or_else(|| "试块留置台账不存在".to_owned())?;
    let rows = vec![
        vec!["字段".to_owned(), "内容".to_owned()],
        vec!["楼号".into(), text_column(&row, "building_no")],
        vec!["浇筑部位".into(), text_column(&row, "concat_position")],
        vec!["成型日期".into(), text_column(&row, "production_date")],
        vec!["强度等级".into(), text_column(&row, "strength_grade")],
        vec!["抗渗等级".into(), text_column(&row, "impermeability_level")],
        vec!["方量".into(), text_column(&row, "sum_volume")],
        vec![
            "标养组数".into(),
            text_column(&row, "number_of_standard_curing_specimen_groups"),
        ],
        vec![
            "抗渗组数".into(),
            text_column(&row, "number_of_sets_of_impermeable_test_pieces"),
        ],
        vec![
            "同养组数".into(),
            text_column(&row, "number_of_specimens_in_the_same_culture"),
        ],
        vec![
            "拆模组数".into(),
            text_column(&row, "number_of_demolding_specimen_groups"),
        ],
    ];
    workbook_envelope(label, rows, file_prefix)
}

async fn entrust_blocks(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
    body: Value,
) -> Result<Value, String> {
    let ids = body
        .as_array()
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .unwrap_or_default();
    let block_type = query
        .get("type")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i32>().ok());
    if ids.is_empty() {
        return Err("请选择台账".into());
    }
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_wtsj_block_retention_ledger WHERE id = ANY($1) ORDER BY building_no,production_date,id",
            &[&ids],
        )
        .await
        .map_err(db_error)?;
    let mut created = 0;
    let mut group_orders = BTreeMap::new();
    for row in rows {
        let building = row
            .get::<_, Option<String>>("building_no")
            .unwrap_or_default();
        let date = row.get::<_, Option<chrono::NaiveDate>>("production_date");
        // 同组只创建一张委托单，但每条台账的样品都必须保留；项目之间不能合并。
        let key = (
            row.get::<_, Option<String>>("project_id"),
            building.clone(),
            date,
        );
        let order_id = if let Some(order_id) = group_orders.get(&key) {
            *order_id
        } else {
            created += 1;
            let order_id = next_id(state, "boxun_wtsj_commission_order").await?;
            state.database.execute(
            "INSERT INTO boxun_wtsj_commission_order (create_time,id,project_id,fk_sk_id,project_name_with_lou,commission_date,material_name,inspection_basis,sy_bs) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,'待送检')",
            &[&order_id, &row.get::<_, Option<String>>("project_id"), &row.get::<_, i64>("id").to_string(), &building, &date, &row.get::<_, Option<String>>("strength_grade"), &row.get::<_, Option<String>>("impermeability_level")],
        ).await.map_err(db_error)?;
            group_orders.insert(key, order_id);
            order_id
        };
        let counts = match block_type {
            Some(1) => vec![(
                1,
                row.get::<_, Option<i32>>("number_of_standard_curing_specimen_groups")
                    .unwrap_or(0),
            )],
            Some(2) => vec![(
                2,
                row.get::<_, Option<i32>>("number_of_sets_of_impermeable_test_pieces")
                    .unwrap_or(0),
            )],
            Some(3) => vec![(
                3,
                row.get::<_, Option<i32>>("number_of_specimens_in_the_same_culture")
                    .unwrap_or(0),
            )],
            Some(4) => vec![(
                4,
                row.get::<_, Option<i32>>("number_of_demolding_specimen_groups")
                    .unwrap_or(0),
            )],
            _ => vec![
                (
                    1,
                    row.get::<_, Option<i32>>("number_of_standard_curing_specimen_groups")
                        .unwrap_or(0),
                ),
                (
                    2,
                    row.get::<_, Option<i32>>("number_of_sets_of_impermeable_test_pieces")
                        .unwrap_or(0),
                ),
                (
                    3,
                    row.get::<_, Option<i32>>("number_of_specimens_in_the_same_culture")
                        .unwrap_or(0),
                ),
                (
                    4,
                    row.get::<_, Option<i32>>("number_of_demolding_specimen_groups")
                        .unwrap_or(0),
                ),
            ],
        };
        for (_, count) in counts.into_iter().filter(|(_, count)| *count > 0) {
            let sample_id = next_id(state, "boxun_wtsj_commission_order_sample").await?;
            state.database.execute(
                "INSERT INTO boxun_wtsj_commission_order_sample (create_time,id,fk_wt_id,sample_name,strength_grade,engineering_location,forming_date,sample_quantity,sample_status) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,'待送检')",
                &[&sample_id, &order_id.to_string(), &row.get::<_, Option<String>>("strength_grade"), &row.get::<_, Option<String>>("strength_grade"), &row.get::<_, Option<String>>("concat_position"), &date, &count.to_string()],
            ).await.map_err(db_error)?;
        }
    }
    Ok(json!(created))
}

async fn entrust_raw_materials(state: &AppState, body: Value) -> Result<Value, String> {
    let request = parse_raw_material_entrust_request(&body)?;
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_mobilization_of_raw_materials WHERE id=ANY($1)",
            &[&request.ids],
        )
        .await
        .map_err(db_error)?;
    if rows.len() != request.ids.len() {
        return Err("部分原材料台账不存在或已被删除".to_owned());
    }
    let project_ids = rows
        .iter()
        .map(|row| row.get::<_, Option<String>>("project_id"))
        .collect::<Vec<_>>();
    let project_id = validate_raw_material_project(&project_ids, request.project_id.as_deref())?;
    let records = rows
        .iter()
        .map(raw_material_record)
        .collect::<Result<Vec<_>, _>>()?;
    let groups = raw_material_groups(&records)?;
    let mut created = 0;
    for group in groups {
        let sample_id = group
            .first()
            .and_then(|row| row.sample_id.clone())
            .ok_or_else(|| "原材料缺少样品".to_owned())?;
        let raw_material_ids = group
            .iter()
            .map(|row| row.id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let order_id = next_id(state, "boxun_wtsj_commission_order").await?;
        state.database.execute(
            "INSERT INTO boxun_wtsj_commission_order (create_time,id,project_id,commission_date,sample_id,sample_type,fk_ycl_id,sy_bs) VALUES (CURRENT_TIMESTAMP,$1,$2,CURRENT_DATE,$3,$4,$5,'已取样')",
            &[
                &order_id,
                &project_id,
                &sample_id,
                &sample_id.parse::<i32>().ok(),
                &raw_material_ids,
            ],
        ).await.map_err(db_error)?;
        for row in group {
            let sample_pk = next_id(state, "boxun_wtsj_commission_order_sample").await?;
            state.database.execute(
                "INSERT INTO boxun_wtsj_commission_order_sample (create_time,id,fk_wt_id,strength_grade,engineering_location,forming_date,furnace_batch_number,manufacturer,sample_quantity,representative_batch,sample_status) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,'1组',$8,'□正常□异常')",
                &[
                    &sample_pk,
                    &order_id.to_string(),
                    &row.strength_grade,
                    &row.concat_position,
                    &row.entry_date,
                    &row.furnace_batch_number,
                    &row.manufacturer,
                    &row.representative_batch,
                ],
            ).await.map_err(db_error)?;
        }
        created += 1;
    }
    Ok(json!(created))
}

fn parse_raw_material_entrust_request(body: &Value) -> Result<RawMaterialEntrustRequest, String> {
    let (ids, project_id) = match body {
        Value::Array(values) => (values.clone(), None),
        Value::Object(values) => (
            values
                .get("ids")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            values
                .get("projectId")
                .and_then(Value::as_str)
                .map(str::to_owned),
        ),
        _ => return Err("请选择台账".to_owned()),
    };
    let ids = ids
        .into_iter()
        .map(|value| value_as_i64(&value).ok_or_else(|| "原材料台账 id 无效".to_owned()))
        .collect::<Result<Vec<_>, _>>()?;
    if ids.is_empty() {
        return Err("请选择台账".to_owned());
    }
    if ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != ids.len()
    {
        return Err("原材料台账不能重复选择".to_owned());
    }
    Ok(RawMaterialEntrustRequest { ids, project_id })
}

fn validate_raw_material_project(
    project_ids: &[Option<String>],
    requested_project_id: Option<&str>,
) -> Result<String, String> {
    let project_id = project_ids
        .first()
        .and_then(Clone::clone)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "原材料缺少项目".to_owned())?;
    if project_ids
        .iter()
        .any(|value| value.as_deref() != Some(project_id.as_str()))
    {
        return Err("所选原材料必须属于同一个项目".to_owned());
    }
    if requested_project_id.is_some_and(|value| value != project_id) {
        return Err("所选原材料不属于当前项目".to_owned());
    }
    Ok(project_id)
}

fn raw_material_record(row: &tokio_postgres::Row) -> Result<RawMaterialRecord, String> {
    Ok(RawMaterialRecord {
        id: row.get("id"),
        sample_id: row.get("sample_id"),
        entry_date: row.get("entry_date"),
        strength_grade: row.get("strength_grade"),
        concat_position: row.get("concat_position"),
        furnace_batch_number: row.get("furnace_batch_number"),
        manufacturer: row.get("manufacturer"),
        representative_batch: row.get("representative_batch"),
    })
}

fn raw_material_groups(rows: &[RawMaterialRecord]) -> Result<Vec<Vec<&RawMaterialRecord>>, String> {
    let mut grouped = BTreeMap::<(Option<NaiveDate>, String), Vec<&RawMaterialRecord>>::new();
    for row in rows {
        let sample_id = row
            .sample_id
            .clone()
            .ok_or_else(|| "原材料缺少样品".to_owned())?;
        grouped
            .entry((row.entry_date, sample_id))
            .or_default()
            .push(row);
    }
    Ok(grouped.into_values().collect())
}

async fn add_raw_materials(
    state: &AppState,
    user: &Authenticated,
    body: Value,
) -> Result<Value, String> {
    let rows = body
        .as_array()
        .cloned()
        .ok_or_else(|| "请求体必须是数组".to_owned())?;
    let mut ids = Vec::new();
    for row in rows {
        let id = next_id(state, "boxun_mobilization_of_raw_materials").await?;
        let entry_date = row
            .get("entryDate")
            .and_then(Value::as_str)
            .and_then(|value| value.parse::<chrono::NaiveDate>().ok());
        let sy_bs = if entry_date.is_none() {
            "无需委托"
        } else {
            "已取样"
        };
        state.database.execute(
            "INSERT INTO boxun_mobilization_of_raw_materials (create_time,creator,id,project_id,sample_id,entrustment_date,entry_date,strength_grade,manufacturer,furnace_batch_number,building_no,concat_position,representative_batch,number_of_pieces,batch_number,remarks,test_number,testing_conclusion,sy_bs) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)",
            &[&user.user_id, &id, &row.get("projectId").and_then(Value::as_str), &row.get("sampleId").and_then(Value::as_str), &row.get("entrustmentDate").and_then(Value::as_str).and_then(|v| v.parse::<chrono::NaiveDate>().ok()), &entry_date, &row.get("strengthGrade").and_then(Value::as_str), &row.get("manufacturer").and_then(Value::as_str), &row.get("furnaceBatchNumber").and_then(Value::as_str), &row.get("buildingNo").and_then(Value::as_str), &row.get("concatPosition").and_then(Value::as_str), &row.get("representativeBatch").and_then(Value::as_str), &row.get("numberOfPieces").and_then(Value::as_str), &row.get("batchNumber").and_then(Value::as_str), &row.get("remarks").and_then(Value::as_str), &row.get("testNumber").and_then(Value::as_str), &row.get("testingConclusion").and_then(Value::as_str), &sy_bs],
        ).await.map_err(db_error)?;
        ids.push(id);
    }
    Ok(Value::Array(ids.into_iter().map(Value::from).collect()))
}

async fn raw_material_pull_down(state: &AppState) -> Result<Value, String> {
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_delegation_order_context WHERE material_name NOT LIKE '%试块%' AND material_name NOT LIKE '%直螺纹连接%' AND material_name NOT LIKE '%电渣压力焊%' ORDER BY sort_no NULLS LAST,id",
            &[],
        )
        .await
        .map_err(db_error)?;
    Ok(Value::Array(
        rows.iter().map(crate::crud::row_value).collect(),
    ))
}

async fn generate_raw_material_witness_records(
    state: &AppState,
    user: &Authenticated,
    body: Value,
) -> Result<Value, String> {
    let ids = body
        .get("ids")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .unwrap_or_default();
    if ids.is_empty() {
        return Ok(Value::Array(Vec::new()));
    }
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_mobilization_of_raw_materials WHERE id=ANY($1) ORDER BY id",
            &[&ids],
        )
        .await
        .map_err(db_error)?;
    if rows.is_empty() {
        return Ok(Value::Array(Vec::new()));
    }
    let sample_ids = rows
        .iter()
        .filter_map(|row| row.get::<_, Option<String>>("sample_id"))
        .collect::<Vec<_>>();
    let sample_rows = state
        .database
        .query(
            "SELECT id,material_name FROM boxun_delegation_order_context WHERE id::text=ANY($1)",
            &[&sample_ids],
        )
        .await
        .map_err(db_error)?;
    let sample_names = sample_rows
        .into_iter()
        .map(|row| {
            (
                row.get::<_, i64>("id").to_string(),
                row.get::<_, Option<String>>("material_name")
                    .unwrap_or_default(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let project_id = rows
        .first()
        .and_then(|row| row.get::<_, Option<String>>("project_id"))
        .ok_or_else(|| "原材料缺少项目".to_owned())?;
    let testing_company = state
        .database
        .query_opt(
            "SELECT unit_name FROM boxun_project_conpany WHERE project_id=$1 AND unit_type=4 ORDER BY id LIMIT 1",
            &[&project_id],
        )
        .await
        .map_err(db_error)?
        .and_then(|row| row.get::<_, Option<String>>("unit_name"))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "您未配置送检单位,请完善项目信息".to_owned())?;
    let witness_count_query = if user.roles.iter().any(|role| role == "super_admin") {
        "SELECT count(*)::bigint FROM boxun_witness_record".to_owned()
    } else {
        "SELECT count(*)::bigint FROM boxun_witness_record WHERE creator=$1".to_owned()
    };
    let witness_count: i64 = if user.roles.iter().any(|role| role == "super_admin") {
        state
            .database
            .query_one(&witness_count_query, &[])
            .await
            .map_err(db_error)?
            .get(0)
    } else {
        state
            .database
            .query_one(&witness_count_query, &[&user.user_id])
            .await
            .map_err(db_error)?
            .get(0)
    };
    let grouped = group_raw_material_rows(&rows, &sample_names)?;
    let mut created = Vec::with_capacity(grouped.len());
    for group in grouped {
        let first = &group.rows[0];
        let sample_name = group.sample_name;
        let test_piece_number = pinyin_first_letters(&sample_name);
        let number = witness_number(&test_piece_number, witness_count);
        let file_name = format!("{}.xlsx", uuid::Uuid::new_v4());
        let record_id = next_id(state, "boxun_witness_record").await?;
        state.database.execute(
            "INSERT INTO boxun_witness_record (create_time,creator,id,project_id,number,sample_name,test_piece_number,sampling_quantity,building_no,sampling_location,sampling_date,sample_and_group_description,testing_company_name,witness_sampling_instructions,file_name,print_flag) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,'0')",
            &[&user.user_id, &record_id, &project_id, &number, &sample_name, &test_piece_number, &(group.rows.len() as i32), &first.get::<_, Option<String>>("building_no"), &first.get::<_, Option<String>>("concat_position"), &group.entry_date, &format!("{sample_name}1组"), &testing_company, &format!("{sample_name}1组"), &file_name],
        ).await.map_err(db_error)?;
        created.push(json!({
            "id": record_id,
            "projectId": project_id,
            "number": number,
            "sampleName": sample_name,
            "testPieceNumber": test_piece_number,
            "samplingQuantity": group.rows.len(),
            "buildingNo": first.get::<_, Option<String>>("building_no"),
            "samplingLocation": first.get::<_, Option<String>>("concat_position"),
            "samplingDate": group.entry_date.map(|value| value.to_string()),
            "sampleAndGroupDescription": format!("{sample_name}1组"),
            "testingCompanyName": testing_company,
            "witnessSamplingInstructions": format!("{sample_name}1组"),
            "fileName": file_name,
        }));
    }
    Ok(Value::Array(created))
}

struct RawMaterialGroup<'a> {
    entry_date: Option<chrono::NaiveDate>,
    sample_name: String,
    rows: Vec<&'a tokio_postgres::Row>,
}

fn group_raw_material_rows<'a>(
    rows: &'a [tokio_postgres::Row],
    sample_names: &BTreeMap<String, String>,
) -> Result<Vec<RawMaterialGroup<'a>>, String> {
    let mut grouped = BTreeMap::<(Option<String>, String), Vec<&tokio_postgres::Row>>::new();
    for row in rows {
        let sample_id = row
            .get::<_, Option<String>>("sample_id")
            .ok_or_else(|| "原材料缺少样品".to_owned())?;
        let entry_date = row.get::<_, Option<chrono::NaiveDate>>("entry_date");
        grouped
            .entry((entry_date.map(|value| value.to_string()), sample_id))
            .or_default()
            .push(row);
    }
    grouped
        .into_iter()
        .map(|((entry_date, sample_id), rows)| {
            let sample_name = sample_names
                .get(&sample_id)
                .cloned()
                .ok_or_else(|| format!("找不到样品配置：{sample_id}"))?;
            Ok(RawMaterialGroup {
                entry_date: entry_date.as_deref().and_then(|value| value.parse().ok()),
                sample_name,
                rows,
            })
        })
        .collect()
}

fn pinyin_first_letters(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            character
                .to_pinyin()
                .map(|pinyin| pinyin.first_letter().to_owned())
                .unwrap_or_else(|| character.to_string())
        })
        .collect::<String>()
        .to_uppercase()
}

fn witness_number(test_piece_number: &str, witness_count: i64) -> String {
    format!("{test_piece_number}_{witness_count}")
}

async fn raw_material_data(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
    body: &Value,
) -> Result<Value, String> {
    let project_id = query
        .get("projectId")
        .and_then(|values| values.first())
        .cloned()
        .or_else(|| body.get("projectId").and_then(value_as_string))
        .unwrap_or_default();
    if project_id.trim().is_empty() {
        return Err("请选择项目".to_owned());
    }
    let sample_ids = state
        .database
        .query(
            "SELECT id FROM boxun_delegation_order_context WHERE material_name NOT LIKE '%试块%' AND material_name NOT LIKE '%直螺纹连接%' AND material_name NOT LIKE '%电渣压力焊%' ORDER BY sort_no NULLS LAST,id",
            &[],
        )
        .await
        .map_err(db_error)?
        .into_iter()
        .map(|row| row.get::<_, i64>("id").to_string())
        .collect::<Vec<_>>();
    if sample_ids.is_empty() {
        return Err("请先配置原材料样品".to_owned());
    }
    let mut created = Vec::with_capacity(RAW_MATERIAL_MOCK_ROWS);
    for mock in raw_material_mock_rows(&sample_ids, chrono::Local::now().date_naive()) {
        let id = next_id(state, "boxun_mobilization_of_raw_materials").await?;
        state.database.execute(
            "INSERT INTO boxun_mobilization_of_raw_materials (create_time,id,project_id,sample_id,entrustment_date,entry_date,strength_grade,manufacturer,furnace_batch_number,building_no,concat_position,representative_batch,number_of_pieces,batch_number,remarks,test_number,testing_conclusion,sampling_and_inspection_status,sy_bs) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,CURRENT_DATE,CURRENT_DATE,$4,$5,$6,$7,$8,$9,'1件',$10,$10,$10,'合格','1','已取样')",
            &[&id, &project_id, &mock.sample_id, &mock.strength_grade, &mock.manufacturer, &mock.furnace_batch_number, &mock.building_no, &mock.concat_position, &mock.representative_batch, &mock.batch_number],
        ).await.map_err(db_error)?;
        created.push(json!({
            "id": id,
            "projectId": project_id,
            "sampleId": mock.sample_id,
            "entryDate": mock.entry_date.to_string(),
            "strengthGrade": mock.strength_grade,
            "manufacturer": mock.manufacturer,
            "buildingNo": mock.building_no,
            "concatPosition": mock.concat_position,
            "representativeBatch": mock.representative_batch,
            "numberOfPieces": "1件",
            "batchNumber": mock.batch_number,
            "syBs": "已取样",
        }));
    }
    Ok(Value::Array(created))
}

fn raw_material_mock_rows(sample_ids: &[String], entry_date: NaiveDate) -> Vec<RawMaterialMockRow> {
    const BUILDING_NUMBERS: [&str; 4] = ["1#", "2#", "3#", "4#"];
    const POSITIONS: [&str; 4] = ["筏板", "防水保护层", "墙柱连梁", "梁板梯"];
    const REPRESENTATIVE_BATCHES: [&str; 4] = ["1T", "2T", "3T", "4T"];

    (0..RAW_MATERIAL_MOCK_ROWS)
        .map(|index| RawMaterialMockRow {
            sample_id: sample_ids[index % sample_ids.len()].clone(),
            entry_date,
            strength_grade: format!("C{}", 30 + index % 16),
            manufacturer: "博勋",
            furnace_batch_number: format!("L{:03}", index + 1),
            building_no: BUILDING_NUMBERS[index % BUILDING_NUMBERS.len()],
            concat_position: POSITIONS[index % POSITIONS.len()],
            representative_batch: REPRESENTATIVE_BATCHES[index % REPRESENTATIVE_BATCHES.len()],
            batch_number: format!("P{:03}", index + 1),
        })
        .collect()
}

async fn add_commercial_ledgers(state: &AppState, body: Value) -> Result<Value, String> {
    let rows = body
        .as_array()
        .cloned()
        .ok_or_else(|| "请求体必须是数组".to_owned())?;
    let mut created = 0;
    for row in rows {
        let id = next_id(state, "boxun_wtsj_commercial_concrete_ledger").await?;
        state.database.execute(
            "INSERT INTO boxun_wtsj_commercial_concrete_ledger (create_time,id,project_id,production_date,building_no,concat_position,strength_grade,manufacturer,production_quantity,sum_volume) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[&id, &row.get("projectId").and_then(Value::as_str), &row.get("productionDate").and_then(Value::as_str).and_then(|v| v.parse::<chrono::NaiveDate>().ok()), &row.get("buildingNo").and_then(Value::as_str), &row.get("concatPosition").and_then(Value::as_str), &row.get("strengthGrade").and_then(Value::as_str), &row.get("manufacturer").and_then(Value::as_str), &row.get("productionQuantity").and_then(Value::as_f64), &row.get("sumVolume").and_then(Value::as_f64)],
        ).await.map_err(db_error)?;
        created += 1;
    }
    Ok(json!(created))
}

fn specimen_group_numbers(query: &BTreeMap<String, Vec<String>>) -> Result<Value, String> {
    let position = query
        .get("pouringPosition")
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
    let grade = query
        .get("strengthGrade")
        .and_then(|values| values.first())
        .map(String::as_str)
        .unwrap_or_default();
    let impermeability = query
        .get("impermeabilityLevel")
        .or_else(|| query.get("ksGrade"))
        .and_then(|values| values.first())
        .map(String::as_str)
        .unwrap_or_default();
    let volume = query
        .get("volumeSum")
        .and_then(|values| values.first())
        .map(|value| {
            value
                .parse::<f64>()
                .map_err(|_| "浇筑方量必须是有效数字".to_owned())
        })
        .transpose()?
        .unwrap_or(0.0);
    if !volume.is_finite() || volume < 0.0 || volume > f64::from(i32::MAX) * 100.0 {
        return Err("浇筑方量必须是有效的非负数，且不能超出组数计算范围".into());
    }
    // 沿用 Boxun 的阶梯方量规则；垫层固定三组标养。
    let standard = if position.contains("垫层") {
        3
    } else if volume < 1000.0 {
        (volume / 100.0).ceil() as i32
    } else {
        10 + ((volume - 1000.0) / 200.0).ceil() as i32
    };
    let impermeable =
        if !impermeability.trim().is_empty() || grade.to_ascii_uppercase().contains('P') {
            (volume / 500.0).ceil() as i32
        } else {
            0
        };
    let excluded = ["垫层", "防水保护层", "止水圈", "反坎", "带", "散水"];
    let same_culture = if excluded.iter().any(|keyword| position.contains(keyword)) {
        0
    } else if position.contains("筏板") {
        3
    } else {
        1
    };
    let demolding = if position.contains("梁") || position.contains("板") {
        1
    } else {
        0
    };
    Ok(json!({
        "numberOfStandardCuringSpecimenGroups": standard,
        "numberOfSetsOfImpermeableTestPieces": impermeable,
        "numberOfSpecimensInTheSameCulture": same_culture,
        "numberOfDemoldingSpecimenGroups": demolding,
    }))
}

async fn generate_commercial_records(
    state: &AppState,
    body: Value,
    kind: CommercialRecordKind,
) -> Result<Value, String> {
    let ids = body
        .get("ids")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .or_else(|| {
            body.as_array()
                .map(|values| values.iter().filter_map(value_as_i64).collect())
        })
        .unwrap_or_default();
    if ids.is_empty() {
        return Err("请选择商混台账".into());
    }
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_wtsj_commercial_concrete_ledger WHERE id=ANY($1) ORDER BY building_no,concat_position,id",
            &[&ids],
        )
        .await
        .map_err(db_error)?;
    let mut groups = BTreeMap::new();
    for row in rows {
        let building = row
            .get::<_, Option<String>>("building_no")
            .unwrap_or_default();
        let position = row
            .get::<_, Option<String>>("concat_position")
            .unwrap_or_default();
        let project_id = row
            .get::<_, Option<String>>("fk_project_id")
            .or_else(|| row.get::<_, Option<String>>("project_id"));
        let key = (project_id.clone(), building.clone(), position.clone());
        let group = groups.entry(key).or_insert_with(|| CommercialGroup {
            source_id: row.get::<_, i64>("id").to_string(),
            project_id,
            production_date: row
                .get::<_, Option<chrono::NaiveDate>>("production_date")
                .unwrap_or_else(|| chrono::Local::now().date_naive()),
            building_no: building,
            position,
            strength_grade: row.get::<_, Option<String>>("strength_grade"),
            impermeability_level: row.get::<_, Option<String>>("impermeability_level"),
            mixing_station: row
                .get::<_, Option<String>>("name_of_commercial_mixing_station")
                .or_else(|| row.get::<_, Option<String>>("manufacturer")),
            volume: 0.0,
        });
        group.volume += row.get::<_, Option<f64>>("sum_volume").unwrap_or(0.0);
    }
    let mut block_count = 0;
    let mut bystander_count = 0;
    let mut concrete_count = 0;
    for group in groups.values() {
        let parameters = [
            ("pouringPosition".into(), vec![group.position.clone()]),
            (
                "strengthGrade".into(),
                vec![group.strength_grade.clone().unwrap_or_default()],
            ),
            (
                "impermeabilityLevel".into(),
                vec![group.impermeability_level.clone().unwrap_or_default()],
            ),
            ("volumeSum".into(), vec![group.volume.to_string()]),
        ]
        .into_iter()
        .collect();
        let counts = specimen_group_numbers(&parameters)?;
        let standard = json_i32(&counts, "numberOfStandardCuringSpecimenGroups");
        let impermeable = json_i32(&counts, "numberOfSetsOfImpermeableTestPieces");
        let same = json_i32(&counts, "numberOfSpecimensInTheSameCulture");
        let demolding = json_i32(&counts, "numberOfDemoldingSpecimenGroups");
        if matches!(
            kind,
            CommercialRecordKind::All | CommercialRecordKind::Block
        ) {
            let block_id = next_id(state, "boxun_wtsj_block_retention_ledger").await?;
            state.database.execute(
            "INSERT INTO boxun_wtsj_block_retention_ledger (create_time,id,fk_sh_id,project_id,production_date,building_no,concat_position,strength_grade,impermeability_level,sum_volume,number_of_standard_curing_specimen_groups,number_of_sets_of_impermeable_test_pieces,number_of_specimens_in_the_same_culture,number_of_demolding_specimen_groups) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)",
            &[&block_id, &group.source_id, &group.project_id, &group.production_date, &group.building_no, &group.position, &group.strength_grade, &group.impermeability_level, &group.volume, &standard, &impermeable, &same, &demolding],
        ).await.map_err(db_error)?;
            block_count += 1;
        }
        if matches!(
            kind,
            CommercialRecordKind::All | CommercialRecordKind::Bystander
        ) {
            let bystander_id = next_id(state, "boxun_bystander_record").await?;
            state.database.execute(
            "INSERT INTO boxun_bystander_record (create_time,id,fk_sh_id,project_id,pouring_location,strength_grade,sum_volume,number_of_standard_curing_specimen_groups,number_of_specimens_in_the_same_culture,number_of_demolding_specimen_groups,number_of_sets_of_impermeable_test_pieces,date_of_bystander,key_parts_of_bystanders,name_of_commercial_mixing_station) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$4,$12)",
            &[&bystander_id, &group.source_id, &group.project_id, &group.position, &group.strength_grade, &group.volume, &standard, &same, &demolding, &impermeable, &group.production_date, &group.mixing_station],
        ).await.map_err(db_error)?;
            bystander_count += 1;
        }
        if matches!(
            kind,
            CommercialRecordKind::All | CommercialRecordKind::Concrete
        ) {
            let concrete_id = next_id(state, "boxun_wtsj_concrete_construction_record").await?;
            state.database.execute(
            "INSERT INTO boxun_wtsj_concrete_construction_record (create_time,id,fk_sh_id,project_id,production_date,building_no,concat_position,pouring_position,sum_volume,name_of_commercial_mixing_station,number_of_standard_curing_specimen_groups,number_of_specimens_in_the_same_culture,number_of_demolding_specimen_groups,number_of_sets_of_impermeable_test_pieces) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)",
            &[&concrete_id, &group.source_id, &group.project_id, &group.production_date, &group.building_no, &group.position, &group.position, &group.volume, &group.mixing_station, &standard, &same, &demolding, &impermeable],
        ).await.map_err(db_error)?;
            concrete_count += 1;
        }
    }
    Ok(json!({
        "blockRetentionLedgerCount": block_count,
        "bystanderRecordCount": bystander_count,
        "concreteRecordCount": concrete_count
    }))
}

struct CommercialGroup {
    source_id: String,
    project_id: Option<String>,
    production_date: chrono::NaiveDate,
    building_no: String,
    position: String,
    strength_grade: Option<String>,
    impermeability_level: Option<String>,
    mixing_station: Option<String>,
    volume: f64,
}

fn json_i32(value: &Value, key: &str) -> i32 {
    value
        .get(key)
        .and_then(Value::as_i64)
        .and_then(|value| i32::try_from(value).ok())
        .unwrap_or(0)
}

async fn delete_commercial_related(state: &AppState, body: Value) -> Result<Value, String> {
    let project_id = body
        .get("projectId")
        .and_then(Value::as_str)
        .ok_or_else(|| "请选择项目".to_owned())?;
    let source_ids = state
        .database
        .query(
            "SELECT id FROM boxun_wtsj_commercial_concrete_ledger WHERE fk_project_id=$1",
            &[&project_id],
        )
        .await
        .map_err(db_error)?
        .into_iter()
        .map(|row| row.get::<_, i64>(0).to_string())
        .collect::<Vec<_>>();
    if source_ids.is_empty() {
        return Ok(json!(0));
    }
    let mut removed = 0_u64;
    for table in [
        "boxun_wtsj_block_retention_ledger",
        "boxun_bystander_record",
        "boxun_wtsj_concrete_construction_record",
    ] {
        removed += state
            .database
            .execute(
                &format!("DELETE FROM {} WHERE fk_sh_id=ANY($1)", quote(table)),
                &[&source_ids],
            )
            .await
            .map_err(db_error)?;
    }
    Ok(json!(removed))
}

async fn delete_commercial_by_project(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let project_id = query
        .get("projectId")
        .and_then(|values| values.first())
        .ok_or_else(|| "请选择项目".to_owned())?;
    let affected = state
        .database
        .execute(
            "DELETE FROM boxun_wtsj_commercial_concrete_ledger WHERE fk_project_id=$1",
            &[&project_id],
        )
        .await
        .map_err(db_error)?;
    Ok(json!(affected))
}

async fn inherit_templates(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let current = query
        .get("currentProjectId")
        .or_else(|| query.get("targetProjectId"))
        .and_then(|values| values.first())
        .ok_or_else(|| "请选择项目".to_owned())?;
    let another = query
        .get("anotherProjectId")
        .or_else(|| query.get("sourceProjectId"))
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_else(|| "1733040597008125954".into());
    let affected = state.database.execute(
        "INSERT INTO boxun_my_template_management (create_time,id,project_id,enable_or_not,template_url,template_description,template_type,bytecode_ref) SELECT CURRENT_TIMESTAMP, (SELECT COALESCE(MAX(id),0) FROM boxun_my_template_management) + row_number() OVER (ORDER BY source.id), $1, source.enable_or_not, source.template_url, source.template_description, source.template_type, source.bytecode_ref FROM boxun_my_template_management source WHERE source.project_id=$2 AND NOT EXISTS (SELECT 1 FROM boxun_my_template_management target WHERE target.project_id=$1 AND target.template_type=source.template_type)",
        &[&current, &another],
    ).await.map_err(db_error)?;
    Ok(json!(affected > 0))
}

fn field_mapping(query: &BTreeMap<String, Vec<String>>) -> Result<Value, String> {
    let template_type = query
        .get("type")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(1);
    let fields = match template_type {
        2 => vec![
            "projectName",
            "sampleName",
            "buildingNo",
            "samplingDate",
            "witness",
        ],
        3 => vec![
            "pouringLocation",
            "constructionUnit",
            "strengthGrade",
            "dateOfBystander",
        ],
        4 => vec![
            "productionDate",
            "buildingNo",
            "pouringPosition",
            "constructionUnit",
        ],
        _ => vec![
            "projectNameWithLou",
            "commissionDate",
            "materialName",
            "inspectionBasis",
        ],
    };
    Ok(json!({
        "beanDesDTOS": fields.into_iter().map(|field| json!({"name": field, "fieldName": field})).collect::<Vec<_>>(),
        "businessDescription": "模板字段"
    }))
}

async fn weather_areas() -> Result<Value, String> {
    Ok(Value::Array(vec![
        json!({"areaId": "57073", "areaName": "洛阳市", "cityName": "洛阳市", "provinceName": "河南省", "countryName": "中国"}),
    ]))
}

async fn weather_area_name(query: &BTreeMap<String, Vec<String>>) -> Result<Value, String> {
    let area_id = query
        .get("areaId")
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
    Ok(json!(if area_id == "57073" {
        "洛阳市"
    } else {
        area_id.as_str()
    }))
}

async fn weather_page_view(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
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
    let total: i64 = state
        .database
        .query_one("SELECT count(*)::bigint FROM boxun_historical_weather", &[])
        .await
        .map_err(db_error)?
        .get(0);
    let rows = state
        .database
        .query(
            "SELECT id,weather_date,week,maximum_temperature,lowest_temperature,morning,afternoon,wind_direction,air_quality_index,area_id FROM boxun_historical_weather ORDER BY weather_date DESC LIMIT $1 OFFSET $2",
            &[&page_size, &((page_no - 1) * page_size)],
        )
        .await
        .map_err(db_error)?;
    Ok(api::page(
        Value::Array(
            rows.iter()
                .map(|row| {
                    let mut value = crate::crud::row_value(row);
                    let area_name = row
                        .get::<_, Option<String>>("area_id")
                        .filter(|value| value == "57073")
                        .map(|_| "洛阳市")
                        .unwrap_or_default();
                    value["areaName"] = Value::String(area_name.to_string());
                    value
                })
                .collect(),
        ),
        total,
    ))
}

async fn weather_by_date(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let year = query
        .get("year")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or_else(|| "year 不能为空".to_owned())?;
    let month = query
        .get("month")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| "month 不能为空".to_owned())?;
    let day = query
        .get("day")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| "day 不能为空".to_owned())?;
    let date =
        chrono::NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| "日期不合法".to_owned())?;
    let row = state
        .database
        .query_opt(
            "SELECT * FROM boxun_historical_weather WHERE weather_date=$1 ORDER BY id LIMIT 1",
            &[&date],
        )
        .await
        .map_err(db_error)?;
    Ok(row
        .as_ref()
        .map(crate::crud::row_value)
        .unwrap_or(Value::Null))
}

async fn test_weather_source(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let area_id = query
        .get("areaId")
        .and_then(|values| values.first())
        .map(String::as_str)
        .unwrap_or("57073");
    let now = chrono::Utc::now().date_naive();
    let year = query
        .get("year")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or_else(|| chrono::Datelike::year(&now));
    let month = query
        .get("month")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or_else(|| chrono::Datelike::month(&now));
    let days = crate::weather::probe_month(state, area_id, year, month).await?;
    Ok(Value::String(format!(
        "天气数据源连通，{year}-{month:02} 返回 {} 条",
        days
    )))
}

async fn sync_weather_month(state: &AppState, body: Value) -> Result<Value, String> {
    let area_id = body
        .get("areaId")
        .and_then(Value::as_str)
        .unwrap_or("57073");
    let year = body
        .get("year")
        .and_then(value_as_i64)
        .ok_or_else(|| "year 不能为空".to_owned())? as i32;
    let month = body
        .get("month")
        .and_then(value_as_i64)
        .ok_or_else(|| "month 不能为空".to_owned())? as u32;
    Ok(json!(
        crate::weather::sync_month(state, area_id, year, month).await?
    ))
}

async fn sync_weather_year(state: &AppState, body: Value) -> Result<Value, String> {
    let area_id = body
        .get("areaId")
        .and_then(Value::as_str)
        .unwrap_or("57073")
        .to_owned();
    let year = body
        .get("year")
        .and_then(value_as_i64)
        .ok_or_else(|| "year 不能为空".to_owned())? as i32;
    let mut affected = 0;
    for month in 1..=12 {
        affected += crate::weather::sync_month(state, &area_id, year, month).await?;
    }
    Ok(json!(affected))
}

async fn sync_weather_recent(
    state: &AppState,
    query: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let area_id = query
        .get("areaId")
        .and_then(|values| values.first())
        .map(String::as_str)
        .unwrap_or("57073")
        .to_owned();
    let years = query
        .get("years")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(1)
        .clamp(1, 5);
    let current = chrono::Utc::now().date_naive();
    let mut affected = 0;
    for offset in 0..(years * 12) {
        let date = current
            .with_day(1)
            .unwrap_or(current)
            .checked_sub_months(chrono::Months::new(offset as u32))
            .ok_or_else(|| "计算天气月份失败".to_owned())?;
        affected += crate::weather::sync_month(state, &area_id, date.year(), date.month()).await?;
    }
    Ok(json!(affected))
}

fn account_price_list() -> Value {
    json!([
        {"accountType": 0, "accountPrice": 0, "durationDays": 7},
        {"accountType": 1, "accountPrice": 30, "durationDays": 999},
        {"accountType": 2, "accountPrice": 120, "durationDays": 120},
        {"accountType": 3, "accountPrice": 365, "durationDays": 365}
    ])
}

async fn update_status_by_ids(
    state: &AppState,
    table: &str,
    column: &str,
    value: &str,
    ids: &[i64],
) -> Result<u64, String> {
    if ids.is_empty() {
        return Ok(0);
    }
    let sql = format!(
        "UPDATE {} SET {}=$1,update_time=CURRENT_TIMESTAMP WHERE id=ANY($2)",
        quote(table),
        quote(column)
    );
    state
        .database
        .execute(&sql, &[&value, &ids])
        .await
        .map_err(db_error)
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

fn value_as_i64(value: &Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_str()?.parse().ok())
}

fn value_as_string(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(str::to_owned)
        .or_else(|| value.as_i64().map(|value| value.to_string()))
}

fn quote(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

fn db_error(error: tokio_postgres::Error) -> String {
    format!("业务动作数据库操作失败：{error}")
}

#[cfg(test)]
mod tests {
    use super::{
        RAW_MATERIAL_MOCK_ROWS, RawMaterialRecord, parse_raw_material_entrust_request,
        pinyin_first_letters, raw_material_groups, raw_material_mock_rows, specimen_group_numbers,
        validate_raw_material_project, witness_number,
    };
    use chrono::NaiveDate;
    use serde_json::json;

    #[test]
    fn specimen_counts_follow_boxun_volume_and_position_rules() -> Result<(), String> {
        // 期望值来自 BoxunSpecimenGroupCalculatorImpl，覆盖阶梯边界和特殊部位。
        let cases = [
            ("垫层", "C15", "", 50.0, [3, 0, 0, 0]),
            ("筏板", "C30", "P6", 1200.0, [11, 3, 3, 1]),
            ("梁板", "C30P6", "", 501.0, [6, 2, 1, 1]),
            ("柱", "C30", "", 1000.0, [10, 0, 1, 0]),
            ("墙", "C30", "", 1001.0, [11, 0, 1, 0]),
            ("墙", "C30", "", 1201.0, [12, 0, 1, 0]),
            ("防水保护层", "C20", "", 100.0, [1, 0, 0, 0]),
            ("止水圈", "C20", "", 0.0, [0, 0, 0, 0]),
            ("后浇带", "C30", "", 101.0, [2, 0, 0, 0]),
        ];
        for (position, grade, impermeability, volume, expected) in cases {
            let query = [
                ("pouringPosition".into(), vec![position.into()]),
                ("strengthGrade".into(), vec![grade.into()]),
                ("impermeabilityLevel".into(), vec![impermeability.into()]),
                ("volumeSum".into(), vec![volume.to_string()]),
            ]
            .into_iter()
            .collect();
            let result = specimen_group_numbers(&query)?;
            assert_eq!(
                result,
                json!({
                    "numberOfStandardCuringSpecimenGroups": expected[0],
                    "numberOfSetsOfImpermeableTestPieces": expected[1],
                    "numberOfSpecimensInTheSameCulture": expected[2],
                    "numberOfDemoldingSpecimenGroups": expected[3],
                }),
                "{position} / {volume}"
            );
        }
        Ok(())
    }

    #[test]
    fn specimen_counts_accept_legacy_impermeability_and_reject_invalid_volumes()
    -> Result<(), String> {
        let mut query = [
            ("ksGrade".into(), vec!["P6".into()]),
            ("volumeSum".into(), vec!["501".into()]),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            specimen_group_numbers(&query)?["numberOfSetsOfImpermeableTestPieces"],
            2
        );
        for invalid in ["-1", "NaN", "inf", "abc", "1e100"] {
            query.insert("volumeSum".into(), vec![invalid.into()]);
            assert!(specimen_group_numbers(&query).is_err(), "{invalid}");
        }
        Ok(())
    }

    #[test]
    fn creates_raw_material_test_piece_number_like_hutool() {
        assert_eq!(pinyin_first_letters("钢筋"), "GJ");
        assert_eq!(pinyin_first_letters("HRB400钢筋"), "HRB400GJ");
    }

    #[test]
    fn creates_raw_material_witness_number_with_separator() {
        assert_eq!(witness_number("GJ", 12), "GJ_12");
    }

    #[test]
    fn creates_fifteen_raw_materials_like_legacy_generator() {
        let rows = raw_material_mock_rows(
            &["10".to_owned(), "20".to_owned()],
            NaiveDate::from_ymd_opt(2026, 9, 26).unwrap(),
        );
        assert_eq!(rows.len(), RAW_MATERIAL_MOCK_ROWS);
        assert_eq!(rows[0].sample_id, "10");
        assert_eq!(rows[0].strength_grade, "C30");
        assert_eq!(rows[0].furnace_batch_number, "L001");
        assert_eq!(rows[0].building_no, "1#");
        assert_eq!(rows[0].concat_position, "筏板");
        assert_eq!(rows[0].representative_batch, "1T");
        assert_eq!(rows[14].sample_id, "10");
        assert_eq!(rows[14].strength_grade, "C44");
        assert_eq!(rows[14].furnace_batch_number, "L015");
        assert_eq!(rows[14].batch_number, "P015");
    }

    #[test]
    fn parses_current_array_and_legacy_object_requests() {
        let current = parse_raw_material_entrust_request(&json!(["1", 2])).unwrap();
        assert_eq!(current.ids, vec![1, 2]);
        assert_eq!(current.project_id, None);

        let legacy = parse_raw_material_entrust_request(&json!({
            "ids": ["3", "4"],
            "type": "1",
            "projectId": "project-a"
        }))
        .unwrap();
        assert_eq!(legacy.ids, vec![3, 4]);
        assert_eq!(legacy.project_id.as_deref(), Some("project-a"));
    }

    #[test]
    fn rejects_invalid_or_duplicate_raw_material_ids() {
        assert!(parse_raw_material_entrust_request(&json!(["1", "invalid"])).is_err());
        assert!(parse_raw_material_entrust_request(&json!(["1", 1])).is_err());
    }

    #[test]
    fn validates_that_raw_materials_belong_to_one_project() {
        let project_ids = [Some("project-a".to_owned()), Some("project-a".to_owned())];
        assert_eq!(
            validate_raw_material_project(&project_ids, Some("project-a")).unwrap(),
            "project-a"
        );
        assert!(validate_raw_material_project(&project_ids, Some("project-b")).is_err());

        let mixed_projects = [Some("project-a".to_owned()), Some("project-b".to_owned())];
        assert!(validate_raw_material_project(&mixed_projects, None).is_err());
    }

    #[test]
    fn groups_raw_materials_by_entry_date_and_sample_id() {
        let records = vec![
            raw_material_record_for_test(1, "10", "2026-09-26"),
            raw_material_record_for_test(2, "10", "2026-09-26"),
            raw_material_record_for_test(3, "20", "2026-09-26"),
            raw_material_record_for_test(4, "10", "2026-09-27"),
        ];
        let groups = raw_material_groups(&records).unwrap();
        assert_eq!(groups.len(), 3);
        assert_eq!(
            groups[0].iter().map(|row| row.id).collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(
            groups[1].iter().map(|row| row.id).collect::<Vec<_>>(),
            vec![3]
        );
        assert_eq!(
            groups[2].iter().map(|row| row.id).collect::<Vec<_>>(),
            vec![4]
        );
    }

    fn raw_material_record_for_test(
        id: i64,
        sample_id: &str,
        entry_date: &str,
    ) -> RawMaterialRecord {
        RawMaterialRecord {
            id,
            sample_id: Some(sample_id.to_owned()),
            entry_date: Some(entry_date.parse().unwrap()),
            strength_grade: Some("C30".to_owned()),
            concat_position: Some("筏板".to_owned()),
            furnace_batch_number: Some("L001".to_owned()),
            manufacturer: Some("博勋".to_owned()),
            representative_batch: Some("1T".to_owned()),
        }
    }
}
