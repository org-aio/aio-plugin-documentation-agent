use serde_json::{Value, json};

pub(crate) fn page(list: Value, total: i64) -> Value {
    json!({"list": list, "total": total})
}
