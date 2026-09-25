use std::collections::BTreeMap;

use crate::auth::Authenticated;
use crate::{AppState, api};
use chrono::Datelike;
use pinyin::ToPinyin;
use serde_json::{Value, json};

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
        ("raw-material-ledger", "POST", "data-generation") => raw_material_data(state, query).await,
        ("raw-material-ledger", "POST", "generate-various-raw-material-records") => {
            generate_raw_material_witness_records(state, user, body).await
        }
        ("commercial-concrete-ledger", "POST", "add-batch") => {
            add_commercial_ledgers(state, body).await
        }
        ("commercial-concrete-ledger", "GET", "specimen-group-numbers") => {
            specimen_group_numbers(query)
        }
        ("commercial-concrete-ledger", "POST", "generate-various-records") => {
            generate_commercial_records(state, body).await
        }
        (
            "commercial-concrete-ledger",
            "POST",
            "batch-generation-of-concrete-construction-records",
        ) => generate_all_commercial(state, "concrete").await,
        ("commercial-concrete-ledger", "POST", "delete-various-records") => {
            delete_commercial_related(state, body).await
        }
        ("commercial-concrete-ledger", "POST", "manually-generate-various-records") => {
            generate_commercial_records(state, body).await
        }
        ("commercial-concrete-ledger", "POST", "mock-sh") => mock_commercial(state, query).await,
        ("commercial-concrete-ledger", "POST", "generate-test-block-retention-account") => {
            generate_all_commercial(state, "block").await
        }
        ("commercial-concrete-ledger", "POST", "generate-side-station-records") => {
            generate_all_commercial(state, "bystander").await
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

async fn generate_all_commercial(state: &AppState, kind: &str) -> Result<Value, String> {
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
    let generated = generate_commercial_records(state, json!({"ids": ids})).await?;
    Ok(generated
        .get(match kind {
            "block" => "blockRetentionLedgerCount",
            "concrete" => "concreteRecordCount",
            _ => "bystanderRecordCount",
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
    let mut group_keys = std::collections::BTreeSet::new();
    for row in rows {
        let building = row
            .get::<_, Option<String>>("building_no")
            .unwrap_or_default();
        let date = row.get::<_, Option<chrono::NaiveDate>>("production_date");
        let key = format!(
            "{building}|{}",
            date.map(|value| value.to_string()).unwrap_or_default()
        );
        if !group_keys.insert(key) {
            continue;
        }
        created += 1;
        let order_id = next_id(state, "boxun_wtsj_commission_order").await?;
        state.database.execute(
            "INSERT INTO boxun_wtsj_commission_order (create_time,id,project_id,fk_sk_id,project_name_with_lou,commission_date,material_name,inspection_basis,sy_bs) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,'待送检')",
            &[&order_id, &row.get::<_, Option<String>>("project_id"), &row.get::<_, Option<String>>("fk_sh_id"), &building, &date, &row.get::<_, Option<String>>("strength_grade"), &row.get::<_, Option<String>>("impermeability_level")],
        ).await.map_err(db_error)?;
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
    let ids = body
        .as_array()
        .map(|values| values.iter().filter_map(value_as_i64).collect::<Vec<_>>())
        .unwrap_or_default();
    if ids.is_empty() {
        return Err("请选择台账".into());
    }
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_mobilization_of_raw_materials WHERE id=ANY($1)",
            &[&ids],
        )
        .await
        .map_err(db_error)?;
    let mut created = 0;
    for row in rows {
        let order_id = next_id(state, "boxun_wtsj_commission_order").await?;
        state.database.execute(
            "INSERT INTO boxun_wtsj_commission_order (create_time,id,project_id,commission_date,material_name,sy_bs) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,'待送检')",
            &[&order_id, &row.get::<_, Option<String>>("project_id"), &row.get::<_, Option<chrono::NaiveDate>>("entrustment_date"), &row.get::<_, Option<String>>("strength_grade")],
        ).await.map_err(db_error)?;
        created += 1;
    }
    Ok(json!(created))
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
) -> Result<Value, String> {
    let project_id = query
        .get("projectId")
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_default();
    let rows = state
        .database
        .query(
            "SELECT * FROM boxun_mobilization_of_raw_materials WHERE project_id=$1 ORDER BY id DESC LIMIT 1000",
            &[&project_id],
        )
        .await
        .map_err(db_error)?;
    Ok(Value::Array(
        rows.iter().map(crate::crud::row_value).collect(),
    ))
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
        .and_then(|values| values.first())
        .map(String::as_str)
        .unwrap_or_default();
    let volume = query
        .get("volumeSum")
        .and_then(|values| values.first())
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(0.0);
    let floor_groups = |value: f64| -> i32 { (value / 100.0).ceil() as i32 };
    let standard = if is_standard_position(&position) {
        floor_groups(volume).max(1)
    } else {
        0
    };
    let impermeable = if !impermeability.is_empty() || grade.to_ascii_uppercase().contains('P') {
        floor_groups(volume).max(1)
    } else {
        0
    };
    let same_culture = if position.contains("底板") || position.contains("基础") {
        1
    } else {
        0
    };
    let demolding = if position.contains("梁") || position.contains("板") || position.contains("柱")
    {
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

fn is_standard_position(position: &str) -> bool {
    !position.is_empty()
}

async fn generate_commercial_records(state: &AppState, body: Value) -> Result<Value, String> {
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
    let mut groups = BTreeMap::<String, CommercialGroup>::new();
    for row in rows {
        let building = row
            .get::<_, Option<String>>("building_no")
            .unwrap_or_default();
        let position = row
            .get::<_, Option<String>>("concat_position")
            .unwrap_or_default();
        let key = format!("{building}|{position}");
        let group = groups.entry(key).or_insert_with(|| CommercialGroup {
            source_id: row
                .get::<_, Option<String>>("id")
                .map(|value| value.to_string()),
            project_id: row.get::<_, Option<String>>("project_id"),
            production_date: row.get::<_, Option<chrono::NaiveDate>>("production_date"),
            building_no: building,
            position,
            strength_grade: row.get::<_, Option<String>>("strength_grade"),
            impermeability_level: row.get::<_, Option<String>>("impermeability_level"),
            manufacturer: row.get::<_, Option<String>>("manufacturer"),
            construction_unit: None,
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
        let block_id = next_id(state, "boxun_wtsj_block_retention_ledger").await?;
        state.database.execute(
            "INSERT INTO boxun_wtsj_block_retention_ledger (create_time,id,fk_sh_id,project_id,production_date,building_no,concat_position,strength_grade,impermeability_level,sum_volume,number_of_standard_curing_specimen_groups,number_of_sets_of_impermeable_test_pieces,number_of_specimens_in_the_same_culture,number_of_demolding_specimen_groups) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)",
            &[&block_id, &group.source_id, &group.project_id, &group.production_date, &group.building_no, &group.position, &group.strength_grade, &group.impermeability_level, &group.volume, &standard, &impermeable, &same, &demolding],
        ).await.map_err(db_error)?;
        block_count += 1;
        let bystander_id = next_id(state, "boxun_bystander_record").await?;
        state.database.execute(
            "INSERT INTO boxun_bystander_record (create_time,id,fk_sh_id,construction_unit,pouring_location,strength_grade,sum_volume,number_of_standard_curing_specimen_groups,number_of_specimens_in_the_same_culture,number_of_demolding_specimen_groups,number_of_sets_of_impermeable_test_pieces,date_of_bystander) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[&bystander_id, &group.source_id, &group.construction_unit, &group.position, &group.strength_grade, &group.volume, &standard, &same, &demolding, &impermeable, &group.production_date],
        ).await.map_err(db_error)?;
        bystander_count += 1;
        let concrete_id = next_id(state, "boxun_wtsj_concrete_construction_record").await?;
        state.database.execute(
            "INSERT INTO boxun_wtsj_concrete_construction_record (create_time,id,fk_sh_id,project_id,production_date,building_no,concat_position,pouring_position,sum_volume,name_of_commercial_mixing_station,number_of_standard_curing_specimen_groups,number_of_specimens_in_the_same_culture,number_of_demolding_specimen_groups,number_of_sets_of_impermeable_test_pieces) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)",
            &[&concrete_id, &group.source_id, &group.project_id, &group.production_date, &group.building_no, &group.position, &group.position, &group.volume, &group.manufacturer, &standard, &same, &demolding, &impermeable],
        ).await.map_err(db_error)?;
        concrete_count += 1;
    }
    Ok(json!({
        "blockRetentionLedgerCount": block_count,
        "bystanderRecordCount": bystander_count,
        "concreteRecordCount": concrete_count
    }))
}

struct CommercialGroup {
    source_id: Option<String>,
    project_id: Option<String>,
    production_date: Option<chrono::NaiveDate>,
    building_no: String,
    position: String,
    strength_grade: Option<String>,
    impermeability_level: Option<String>,
    manufacturer: Option<String>,
    construction_unit: Option<String>,
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
    use super::{pinyin_first_letters, witness_number};

    #[test]
    fn creates_raw_material_test_piece_number_like_hutool() {
        assert_eq!(pinyin_first_letters("钢筋"), "GJ");
        assert_eq!(pinyin_first_letters("HRB400钢筋"), "HRB400GJ");
    }

    #[test]
    fn creates_raw_material_witness_number_with_separator() {
        assert_eq!(witness_number("GJ", 12), "GJ_12");
    }
}
