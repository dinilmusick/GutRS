//! JSON-object equivalents for GutTS object utilities.
//! No JavaScript prototype chain exists in this representation.
use serde_json::{Map, Value};

pub fn is_plain_object(value: &Value) -> bool { value.is_object() }

/// Tests an own field (not an inherited JavaScript prototype property).
pub fn has_own_field(value: &Value, key: &str) -> bool {
    value.as_object().is_some_and(|record| record.contains_key(key))
}

/// Exact shallow-field equivalent for keys that are not lodash deep-path expressions.
pub fn pick_fields(value: &Value, keys: &[&str]) -> Option<Value> {
    let source = value.as_object()?;
    let mut result = Map::new();
    for &key in keys {
        if let Some(found) = source.get(key) {
            result.insert(key.to_owned(), found.clone());
        }
    }
    Some(Value::Object(result))
}

pub fn omit_fields(value: &Value, keys: &[&str]) -> Option<Value> {
    let source = value.as_object()?;
    let mut result = source.clone();
    for &key in keys {
        result.remove(key);
    }
    Some(Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn preserves_own_fields_without_prototype_pollution() {
        let value = json!({"a":1,"b":{"c":2},"__proto__":"literal"});
        assert!(has_own_field(&value, "__proto__"));
        assert!(!has_own_field(&value, "constructor"));
        assert_eq!(pick_fields(&value, &["b","missing"]), Some(json!({"b":{"c":2}})));
        assert_eq!(omit_fields(&value, &["a"]), Some(json!({"b":{"c":2},"__proto__":"literal"})));
        assert_eq!(pick_fields(&json!(null), &["x"]), None);
    }
}
