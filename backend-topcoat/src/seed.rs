use tokio_postgres::Client;

use crate::AppState;

const DEFAULT_ADMIN_PASSWORD: &str = "$2a$04$KljJDa/LK7QfDm0lF5OhuePhlPfjRH3tB2Wu351Uidz.oQGJXevPi";

pub(crate) async fn initialize(state: &AppState) -> Result<(), String> {
    let client = state.database.as_ref();
    create_admin(client).await?;
    create_dictionaries(client).await?;
    create_menus(client).await?;
    create_configs(client).await?;
    Ok(())
}

async fn create_admin(client: &Client) -> Result<(), String> {
    client
        .execute(
            "INSERT INTO system_users (create_time,id,tenant_id,status,username,password,nickname,realname,dept_id,email,mobile,sex) VALUES (CURRENT_TIMESTAMP,1,1,0,'admin',$1,'系统管理员','系统管理员',100,'','','1') ON CONFLICT (id) DO NOTHING",
            &[&DEFAULT_ADMIN_PASSWORD],
        )
        .await
        .map_err(|error| format!("初始化管理员失败：{error}"))?;
    client
        .execute(
            "INSERT INTO system_role (create_time,id,code,sort,name,tenant_id,remark,status,type,data_scope) VALUES (CURRENT_TIMESTAMP,1,'super_admin',1,'超级管理员',1,'超级管理员',0,1,1) ON CONFLICT (id) DO NOTHING",
            &[],
        )
        .await
        .map_err(|error| format!("初始化角色失败：{error}"))?;
    client
        .execute(
            "INSERT INTO system_user_role (user_id,role_id) VALUES (1,1) ON CONFLICT DO NOTHING",
            &[],
        )
        .await
        .map_err(|error| format!("初始化管理员角色失败：{error}"))?;
    Ok(())
}

async fn create_dictionaries(client: &Client) -> Result<(), String> {
    type Dictionary = (
        &'static str,
        &'static str,
        &'static [(&'static str, &'static str, i32)],
    );
    const DICTIONARIES: &[Dictionary] = &[
        (
            "common_status",
            "通用状态",
            &[("开启", "1", 1), ("关闭", "0", 0)],
        ),
        (
            "user_type",
            "用户类型",
            &[("会员", "1", 1), ("管理员", "2", 2)],
        ),
        (
            "system_user_sex",
            "用户性别",
            &[("男", "1", 1), ("女", "2", 2), ("未知", "0", 0)],
        ),
        (
            "system_menu_type",
            "菜单类型",
            &[("目录", "1", 1), ("菜单", "2", 2), ("按钮", "3", 3)],
        ),
        (
            "system_role_type",
            "角色类型",
            &[("内置角色", "1", 1), ("自定义角色", "2", 2)],
        ),
        (
            "system_data_scope",
            "数据范围",
            &[
                ("全部数据权限", "1", 1),
                ("指定部门数据权限", "2", 2),
                ("本部门数据权限", "3", 3),
                ("本部门及以下数据权限", "4", 4),
                ("仅本人数据权限", "5", 5),
            ],
        ),
        (
            "infra_boolean_string",
            "是否",
            &[("是", "true", 1), ("否", "false", 0)],
        ),
        (
            "infra_config_type",
            "参数类型",
            &[("系统内置", "1", 1), ("自定义", "2", 2)],
        ),
    ];
    for (dict_type, name, rows) in DICTIONARIES {
        let exists: bool = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM system_dict_type WHERE type=$1 AND deleted=0)",
                &[dict_type],
            )
            .await
            .map_err(|error| format!("查询字典类型失败：{error}"))?
            .get(0);
        if !exists {
            let next_id: i64 = client
                .query_one("SELECT COALESCE(MAX(id),0)+1 FROM system_dict_type", &[])
                .await
                .map_err(|error| format!("生成字典类型主键失败：{error}"))?
                .get(0);
            client
                .execute(
                    "INSERT INTO system_dict_type (create_time,id,name,type,status,deleted) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,0,0)",
                    &[&next_id, name, dict_type],
                )
                .await
                .map_err(|error| format!("初始化字典类型失败：{error}"))?;
        }
        for (label, value, sort) in *rows {
            let exists: bool = client
                .query_one(
                    "SELECT EXISTS(SELECT 1 FROM system_dict_data WHERE dict_type=$1 AND value=$2 AND deleted=0)",
                    &[dict_type, value],
                )
                .await
                .map_err(|error| format!("查询字典数据失败：{error}"))?
                .get(0);
            if exists {
                continue;
            }
            let data_id: i64 = client
                .query_one("SELECT COALESCE(MAX(id),0)+1 FROM system_dict_data", &[])
                .await
                .map_err(|error| format!("生成字典数据主键失败：{error}"))?
                .get(0);
            client
                .execute(
                    "INSERT INTO system_dict_data (create_time,id,sort,status,deleted,label,value,dict_type) VALUES (CURRENT_TIMESTAMP,$1,$2,0,0,$3,$4,$5)",
                    &[&data_id, sort, label, value, dict_type],
                )
                .await
                .map_err(|error| format!("初始化字典数据失败：{error}"))?;
        }
    }
    Ok(())
}

async fn create_menus(client: &Client) -> Result<(), String> {
    const MENUS: &[(i64, i64, &str, &str, i32, i32)] = &[
        (1, 0, "系统管理", "/system", 1, 1),
        (2, 1, "用户管理", "user", 2, 1),
        (3, 1, "角色管理", "role", 2, 2),
        (4, 1, "菜单管理", "menu", 2, 3),
        (5, 1, "部门管理", "dept", 2, 4),
        (6, 1, "岗位管理", "post", 2, 5),
        (7, 1, "字典管理", "dict", 2, 6),
        (8, 0, "基础设施", "/infra", 1, 2),
        (9, 8, "参数配置", "config", 2, 1),
        (10, 8, "文件管理", "file", 2, 2),
        (11, 8, "邮件配置", "mail-account", 2, 3),
        (12, 0, "业务台账", "/boxun", 1, 3),
        (13, 12, "首页看板", "homepage", 2, 1),
        (14, 12, "项目管理", "project-info", 2, 2),
        (15, 12, "商混台账", "commercial-concrete-ledger", 2, 3),
        (16, 12, "原材料台账", "raw-material-ledger", 2, 4),
        (17, 12, "试块台账", "block-retention-ledger", 2, 5),
        (18, 12, "委托单", "commission-order", 2, 6),
        (19, 12, "历史天气", "historical-weather", 2, 7),
        (20, 12, "模板管理", "template-management", 2, 8),
    ];
    for (id, parent_id, name, path, menu_type, sort) in MENUS {
        let parent = if *parent_id == 0 {
            None
        } else {
            Some(parent_id)
        };
        client
            .execute(
                "INSERT INTO system_menu (create_time,id,name,permission,type,sort,parent_id,path,icon,status,visible,keep_alive,always_show) VALUES (CURRENT_TIMESTAMP,$1,$2,'',$3,$4,$5,$6,'ep:document',0,TRUE,TRUE,FALSE) ON CONFLICT (id) DO NOTHING",
                &[id, name, menu_type, sort, &parent, path],
            )
            .await
            .map_err(|error| format!("初始化菜单失败：{error}"))?;
    }
    Ok(())
}

async fn create_configs(client: &Client) -> Result<(), String> {
    const CONFIGS: &[(i64, &str, &str, &str, i32)] = &[
        (1, "系统配置", "system.title", "博勋技术服务平台", 1),
        (2, "支付配置", "boxun.pay.alipay.enabled", "false", 2),
    ];
    for (id, name, key, value, config_type) in CONFIGS {
        client
            .execute(
                "INSERT INTO infra_config (create_time,id,name,config_key,value,type,visible,category) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,TRUE,'boxun') ON CONFLICT (id) DO NOTHING",
                &[id, name, key, value, config_type],
            )
            .await
            .map_err(|error| format!("初始化参数失败：{error}"))?;
    }
    Ok(())
}
