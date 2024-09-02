use serde_json::json;

use crate::sepa::get_structured_refference;

pub async fn make_structured_refference() -> String {
    get_structured_refference()
}

pub async fn make_structured_refference_json() -> String {
    json!({ "structured_refference": get_structured_refference() }).to_string()
}
