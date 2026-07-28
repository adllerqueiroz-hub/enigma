use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub fn infer_rust_type(value: &Value) -> String {
    match value {
        Value::Null => "Option<String>".to_string(),
        Value::Bool(_) => "bool".to_string(),
        Value::Number(n) => if n.is_i64() { "i32" } else { "f32" }.to_string(),
        Value::String(_) => "String".to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "Vec<serde_json::Value>".to_string()
            } else {
                let element_types: HashSet<_> = arr.iter().map(infer_rust_type).collect();
                let element_type = if element_types.iter().all(|ty| ty == "i32" || ty == "f32") {
                    if element_types.contains("f32") {
                        "f32"
                    } else {
                        "i32"
                    }
                } else if element_types.len() == 1 {
                    element_types.iter().next().unwrap()
                } else {
                    "serde_json::Value"
                };
                format!("Vec<{element_type}>")
            }
        }
        Value::Object(_) => "serde_json::Value".to_string(),
    }
}

pub fn analyze_field_types(records: &[Value]) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();

    for record in records {
        if let Value::Object(obj) = record {
            for (k, v) in obj {
                map.entry(k.clone())
                    .or_insert_with(HashSet::new)
                    .insert(infer_rust_type(v));
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn widens_mixed_numeric_arrays() {
        assert_eq!(infer_rust_type(&json!([1, 1.5, 2])), "Vec<f32>");
    }
}
