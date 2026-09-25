use serde::Deserialize;
use serde_json::{Value, json};

const AIO_PAGES: &str = include_str!("../../frontend/src/features/navigation/aio-pages.json");

#[derive(Deserialize)]
struct AioPage {
    id: String,
    label: String,
    #[allow(dead_code)]
    route: String,
    menu_path: Vec<String>,
}

pub(crate) fn description() -> Value {
    let pages: Vec<AioPage> = serde_json::from_str(AIO_PAGES)
        .expect("frontend/src/features/navigation/aio-pages.json 必须是有效菜单清单");
    let pages = pages
        .into_iter()
        .map(|page| {
            json!({
                "id": page.id,
                "label": page.label,
                "entry": format!("pages/{}.html", page.id_for_entry()),
                "scene": ["documentation-agent", "资料员服务平台"],
                "menu_path": page.menu_path,
                "permission": null,
                "surface": "workspace"
            })
        })
        .collect::<Vec<_>>();
    json!({
        "label": "资料员服务平台",
        "pages": pages
    })
}

impl AioPage {
    fn id_for_entry(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_matches_boxun_visible_leaves() {
        let value = description();
        assert_eq!(value["label"], "资料员服务平台");
        let pages = value["pages"].as_array().expect("pages 必须是数组");
        assert_eq!(pages.len(), 14);
        assert_eq!(pages[0]["scene"], json!(["documentation-agent", "资料员服务平台"]));
        assert_eq!(pages[0]["entry"], "pages/menu-5.html");
        assert!(pages.iter().any(|page| page["label"] == "试块评定"));
        assert_eq!(pages[0]["menu_path"], json!([]));
        assert_eq!(pages[1]["menu_path"], json!([]));
        assert!(pages.iter().all(|page| {
            !matches!(page["label"].as_str(), Some("用户管理" | "角色管理" | "菜单管理" | "文件管理"))
                && !page["menu_path"].as_array().is_some_and(|path| {
                    path.iter().any(|group| matches!(group.as_str(), Some("系统管理" | "基础设施" | "功能示例")))
                })
        }));
    }
}
