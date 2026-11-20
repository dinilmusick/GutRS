//! Portable JSON contracts. `serde_json::Value` represents only valid JSON
//! (unlike arbitrary JavaScript objects, which may contain undefined/cycles/functions).
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};

pub type JsonValue = Value;
pub type JsonObject = serde_json::Map<String, Value>;

/// The typed Value boundary is a strict JSON-value admission mechanism.
/// JavaScript values outside the JSON domain cannot be represented here.
pub fn is_json_value(_value: &Value) -> bool {
    true
}

pub fn parse_json(text: &str) -> Result<JsonValue, serde_json::Error> {
    serde_json::from_str(text)
}

pub fn try_parse_json(text: &str) -> Option<JsonValue> {
    parse_json(text).ok()
}

/// JSON.stringify(value,null,space) with numeric space clamped to 0..10.
pub fn stringify_json(value: &Value, space: Option<i32>) -> Result<String, serde_json::Error> {
    let indent = space.unwrap_or(0).clamp(0, 10) as usize;
    if indent == 0 {
        return serde_json::to_string(value);
    }
    let mut buffer = Vec::new();
    let whitespace = vec![b' '; indent];
    {
        let formatter = PrettyFormatter::with_indent(&whitespace);
        let mut serializer = Serializer::with_formatter(&mut buffer, formatter);
        value.serialize(&mut serializer)?;
    }
    String::from_utf8(buffer)
        .map_err(|error| <serde_json::Error as serde::ser::Error>::custom(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn parses_and_serializes() {
        assert_eq!(parse_json("null").expect("valid"), Value::Null);
        assert_eq!(try_parse_json("{bad}"), None);
        assert!(parse_json("1e10000").is_err());
        assert_eq!(
            stringify_json(&json!([1, true, null]), None).unwrap(),
            "[1,true,null]"
        );
        assert_eq!(
            stringify_json(&json!({"x":[1]}), Some(2)).unwrap(),
            "{\n  \"x\": [\n    1\n  ]\n}"
        );
        assert!(is_json_value(&json!(["safe", {"nested": true}])));
        assert_eq!(stringify_json(&json!([1]), Some(-10)).unwrap(), "[1]");
    }
}
