use serde_json::{json, Value};

pub(crate) fn handle_query(json_str: String, key_name: &str) -> String {
    let parsed_json: Result<Value, _> = serde_json::from_str(&json_str);

    return if let Ok(json_value) = parsed_json {
        if let Some(value) = json_value.get(key_name) {
            value.to_string()
        } else {
            json!({"error": "Key not found"}).to_string()
        }
    } else {
        json!({"error": "Invalid JSON"}).to_string()
    }
}
