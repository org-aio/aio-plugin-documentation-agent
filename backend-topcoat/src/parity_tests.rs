use super::*;

// 在独立事务和 schema 中应用完整安装迁移，检查真实路由与 PostgreSQL 写入。
#[tokio::test]
#[ignore = "需要 BOXUN_TEST_DATABASE_URL 指向测试 PostgreSQL"]
async fn fresh_tenant_record_routes_and_specimen_calculation()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = env::var("BOXUN_TEST_DATABASE_URL")?;
    let (database, connection) = tokio_postgres::connect(&url, tokio_postgres::NoTls).await?;
    let connection_task = tokio::spawn(connection);
    let schema = format!("boxun_parity_{}", uuid::Uuid::new_v4().simple());
    database
        .batch_execute(&format!(
            "BEGIN; CREATE SCHEMA {schema}; SET LOCAL search_path TO {schema}"
        ))
        .await?;
    let mut migrations = std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    migrations.retain(|path| path.extension().is_some_and(|ext| ext == "sql"));
    migrations.sort();
    for migration in migrations {
        database
            .batch_execute(&std::fs::read_to_string(migration)?)
            .await?;
    }
    database.batch_execute(
        "INSERT INTO boxun_wtsj_commission_order (id,create_time,project_id,project_name,project_name_with_lou,commission_date,material_name,fk_sk_id)
         VALUES (101,CURRENT_TIMESTAMP,'project-a','测试项目','1号楼','2026-09-26','C30','201')"
    ).await?;
    let state = AppState {
        database: Arc::new(database),
        ingress_token: "test-ingress".into(),
        jwt_secret: "test-secret".into(),
        broker_socket: None,
        channel: Arc::new(Mutex::new(Vec::new())),
    };
    let user = auth::Authenticated {
        user_id: 1,
        tenant_id: 1,
        roles: vec!["super_admin".into()],
        permissions: vec![],
    };
    for route in [
        "witness-record",
        "bystander-record",
        "concrete-construction-record",
    ] {
        let response = dispatch(
            &state,
            &user,
            &Method::POST,
            &format!("/boxun/{route}/generate-from-commission-order"),
            None,
            json!({"commissionOrderId": "101"}),
        )
        .await
        .map_err(|error| format!("路由调用失败：{error:?}"))?;
        let body = to_bytes(response.into_body(), 1024 * 1024)
            .await
            .map_err(|error| format!("读取响应失败：{error:?}"))?;
        let payload: Value = serde_json::from_slice(&body)?;
        assert_eq!(payload["code"], 0, "{route}: {payload}");
        assert_eq!(payload["data"].as_array().map(Vec::len), Some(1));
        let entity = crud::entity(route).ok_or("业务实体不存在")?;
        let row = state
            .database
            .query_one(&format!("SELECT * FROM {}", entity.table), &[])
            .await?;
        assert_eq!(
            row.get::<_, Option<String>>("print_flag").as_deref(),
            Some("0")
        );
        if route == "concrete-construction-record" {
            assert_eq!(
                row.get::<_, Option<String>>("project_id").as_deref(),
                Some("project-a")
            );
            assert_eq!(
                row.get::<_, Option<String>>("fk_sk_id").as_deref(),
                Some("201")
            );
        } else {
            assert_eq!(
                row.get::<_, Option<String>>("fk_wt_id").as_deref(),
                Some("101")
            );
        }
        let response = dispatch(
            &state,
            &user,
            &Method::GET,
            &format!("/boxun/{route}/page"),
            None,
            Value::Null,
        )
        .await
        .map_err(|error| format!("路由调用失败：{error:?}"))?;
        let body = to_bytes(response.into_body(), 1024 * 1024)
            .await
            .map_err(|error| format!("读取响应失败：{error:?}"))?;
        let payload: Value = serde_json::from_slice(&body)?;
        assert_eq!(payload["code"], 0, "CRUD: {payload}");
        assert_eq!(payload["data"]["total"], 1);
    }
    for route in ["commercial-concrete-ledger", "block-retention-ledger"] {
        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pouringPosition", "筏板")
            .append_pair("volumeSum", "1200")
            .append_pair("ksGrade", "P6")
            .finish();
        let response = dispatch(
            &state,
            &user,
            &Method::GET,
            &format!("/boxun/{route}/specimen-group-numbers"),
            Some(&query),
            Value::Null,
        )
        .await
        .map_err(|error| format!("路由调用失败：{error:?}"))?;
        let body = to_bytes(response.into_body(), 1024 * 1024)
            .await
            .map_err(|error| format!("读取响应失败：{error:?}"))?;
        let payload: Value = serde_json::from_slice(&body)?;
        assert_eq!(payload["code"], 0, "{route}: {payload}");
        assert_eq!(
            payload["data"],
            json!({
                "numberOfStandardCuringSpecimenGroups": 11,
                "numberOfSetsOfImpermeableTestPieces": 3,
                "numberOfSpecimensInTheSameCulture": 3,
                "numberOfDemoldingSpecimenGroups": 1,
            })
        );
    }
    commercial_generation_and_crud(&state, &user).await?;
    block_entrust_keeps_every_sample(&state, &user).await?;
    state.database.batch_execute("ROLLBACK").await?;
    drop(state);
    connection_task.await??;
    Ok(())
}

type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn request_data(
    state: &AppState,
    user: &auth::Authenticated,
    method: Method,
    path: &str,
    query: Option<&str>,
    body: Value,
) -> TestResult<Value> {
    let response = dispatch(state, user, &method, path, query, body)
        .await
        .map_err(|error| format!("路由调用失败：{error:?}"))?;
    let bytes = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .map_err(|error| format!("读取响应失败：{error:?}"))?;
    let payload: Value = serde_json::from_slice(&bytes)?;
    assert_eq!(payload["code"], 0, "{path}: {payload}");
    Ok(payload["data"].clone())
}

async fn commercial_generation_and_crud(
    state: &AppState,
    user: &auth::Authenticated,
) -> TestResult {
    let first = request_data(
        state,
        user,
        Method::POST,
        "/boxun/commercial-concrete-ledger/create",
        None,
        json!({
            "fkProjectId": "project-a", "productionDate": "2026-09-26", "buildingNo": "1号楼",
            "concatPosition": "筏板", "strengthGrade": "C30", "impermeabilityLevel": "P6",
            "nameOfCommercialMixingStation": "测试商混站", "sumVolume": 1200,
            "numberOfStandardCuringSpecimenGroups": 11, "numberOfSetsOfImpermeableTestPieces": 3,
            "numberOfSpecimensInTheSameCulture": 3, "numberOfDemoldingSpecimenGroups": 1,
            "syntrophicTemperature": null, "day28Data": "检测报告"
        }),
    )
    .await?;
    assert_eq!(first["fkProjectId"], "project-a");
    assert_eq!(first["productionDate"], "2026-09-26");
    assert_eq!(first["sumVolume"], 1200.0);
    assert_eq!(first["numberOfStandardCuringSpecimenGroups"], 11);
    assert_eq!(first["day28Data"], "检测报告");
    assert!(first["syntrophicTemperature"].is_null());
    let mut second = first.clone();
    second["fkProjectId"] = json!("project-b");
    second["sumVolume"] = json!(100);
    request_data(
        state,
        user,
        Method::POST,
        "/boxun/commercial-concrete-ledger/create",
        None,
        second,
    )
    .await?;
    request_data(
        state,
        user,
        Method::PUT,
        "/boxun/commercial-concrete-ledger/update",
        None,
        json!({"id": first["id"], "strengthRemarks": "已复核", "productionDate": null}),
    )
    .await?;
    let page = request_data(
        state,
        user,
        Method::GET,
        "/boxun/commercial-concrete-ledger/page",
        Some("fkProjectId=project-a"),
        Value::Null,
    )
    .await?;
    assert_eq!(page["total"], 1);
    assert_eq!(page["list"][0]["strengthRemarks"], "已复核");
    assert!(page["list"][0]["productionDate"].is_null());

    // 每种单类型入口只写自己的表；相同楼号和部位的不同项目保持独立。
    let routes = [
        "generate-test-block-retention-account",
        "generate-side-station-records",
        "batch-generation-of-concrete-construction-records",
    ];
    let tables = [
        "boxun_wtsj_block_retention_ledger",
        "boxun_bystander_record",
        "boxun_wtsj_concrete_construction_record",
    ];
    for (index, route) in routes.iter().enumerate() {
        state.database.batch_execute("DELETE FROM boxun_wtsj_block_retention_ledger; DELETE FROM boxun_bystander_record; DELETE FROM boxun_wtsj_concrete_construction_record").await?;
        let count = request_data(
            state,
            user,
            Method::POST,
            &format!("/boxun/commercial-concrete-ledger/{route}"),
            None,
            Value::Null,
        )
        .await?;
        assert_eq!(count, 2);
        for (table_index, table) in tables.iter().enumerate() {
            let count: i64 = state
                .database
                .query_one(&format!("SELECT count(*) FROM {table}"), &[])
                .await?
                .get(0);
            assert_eq!(
                count,
                if index == table_index { 2 } else { 0 },
                "{route}: {table}"
            );
        }
        let row = state
            .database
            .query_one(
                &format!(
                    "SELECT * FROM {} WHERE project_id='project-a'",
                    tables[index]
                ),
                &[],
            )
            .await?;
        assert_eq!(
            row.get::<_, Option<String>>("fk_sh_id"),
            Some(first["id"].to_string())
        );
        assert_eq!(
            row.get::<_, Option<i32>>("number_of_standard_curing_specimen_groups"),
            Some(11)
        );
        assert_eq!(row.get::<_, Option<f64>>("sum_volume"), Some(1200.0));
        if index != 0 {
            assert_eq!(
                row.get::<_, Option<String>>("name_of_commercial_mixing_station")
                    .as_deref(),
                Some("测试商混站")
            );
        }
    }
    let generated = request_data(
        state,
        user,
        Method::POST,
        "/boxun/commercial-concrete-ledger/generate-various-records",
        None,
        json!({"ids": [first["id"].to_string()]}),
    )
    .await?;
    assert_eq!(
        generated,
        json!({"blockRetentionLedgerCount": 1, "bystanderRecordCount": 1, "concreteRecordCount": 1})
    );
    Ok(())
}

async fn block_entrust_keeps_every_sample(
    state: &AppState,
    user: &auth::Authenticated,
) -> TestResult {
    state.database.batch_execute(
        "DELETE FROM boxun_wtsj_commission_order; DELETE FROM boxun_wtsj_commission_order_sample;
         DELETE FROM boxun_wtsj_block_retention_ledger;
         INSERT INTO boxun_wtsj_block_retention_ledger
         (id,create_time,project_id,fk_sh_id,building_no,production_date,concat_position,strength_grade,number_of_standard_curing_specimen_groups,number_of_specimens_in_the_same_culture)
         VALUES (201,CURRENT_TIMESTAMP,'project-a','901','1号楼','2026-09-26','一层梁板','C30',2,1),
                (202,CURRENT_TIMESTAMP,'project-a','902','1号楼','2026-09-26','二层梁板','C35',3,2),
                (203,CURRENT_TIMESTAMP,'project-b','903','1号楼','2026-09-26','一层梁板','C30',1,1)"
    ).await?;
    let count = request_data(
        state,
        user,
        Method::POST,
        "/boxun/block-retention-ledger/entrust-the-test-block",
        None,
        json!([201, 202, 203]),
    )
    .await?;
    assert_eq!(count, 2);
    let order = state
        .database
        .query_one(
            "SELECT id,fk_sk_id FROM boxun_wtsj_commission_order WHERE project_id='project-a'",
            &[],
        )
        .await?;
    assert_eq!(
        order.get::<_, Option<String>>("fk_sk_id").as_deref(),
        Some("201")
    );
    let samples = state.database.query("SELECT engineering_location,sample_quantity FROM boxun_wtsj_commission_order_sample WHERE fk_wt_id=$1 ORDER BY engineering_location,sample_quantity", &[&order.get::<_, i64>("id").to_string()]).await?;
    assert_eq!(samples.len(), 4);
    let total = samples
        .iter()
        .map(|row| row.get::<_, String>("sample_quantity").parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()?
        .iter()
        .sum::<i32>();
    assert_eq!(total, 8);
    let second_position_count = samples
        .iter()
        .filter(|row| row.get::<_, String>("engineering_location") == "二层梁板")
        .count();
    assert_eq!(second_position_count, 2);
    Ok(())
}
