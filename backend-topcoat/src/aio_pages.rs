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
                "scene": ["workspace", "工作空间"],
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
        assert_eq!(pages.len(), 30);
        assert_eq!(pages[0]["entry"], "pages/menu-5.html");
        assert!(pages.iter().any(|page| page["label"] == "试块评定"));
        assert_eq!(pages[0]["menu_path"], json!([]));
        assert_eq!(pages[1]["menu_path"], json!(["系统管理"]));
    }
}
