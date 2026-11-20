//! Portable JSON value validation and ECMAScript-compatible JSON.stringify.
//!
//! GutTS pinned source: src/json/json-parse.ts. Rust values cannot express
//! JS undefined, functions, symbol keys, cycles or lone UTF-16 surrogates.
//! Within the valid JSON domain, preserve JS own-property ordering,
//! JavaScript double-precision numeric output and indentation rules.
use serde_json::Value;

pub type JsonValue = Value;
pub type JsonObject = serde_json::Map<String, Value>;

/// Typed serde_json::Value is already a valid JSON value by construction.
pub fn is_json_value(_value: &Value) -> bool {
    true
}

pub fn parse_json(text: &str) -> Result<JsonValue, serde_json::Error> {
    serde_json::from_str(text)
}

pub fn try_parse_json(text: &str) -> Option<JsonValue> {
    parse_json(text).ok()
}

/// JavaScript's enumerable integer-index properties sort first in ascending
/// numeric order. The string "4294967295" is not an array index, nor are
/// spellings such as "01" or "-1".
fn js_property_index(key: &str) -> Option<u32> {
    if key.is_empty()
        || (key.len() > 1 && key.starts_with('0'))
        || !key.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    key.parse::<u32>().ok().filter(|n| *n < u32::MAX)
}

/// Serialize the Rust JSON domain using ECMAScript property enumeration
/// and IEEE-754 number-to-string rules, not serde_json's default BTreeMap
/// sorting and Rust's different floating-point formatting.
fn write_js_value(
    value: &Value,
    depth: usize,
    indent: usize,
    output: &mut String,
) -> Result<(), serde_json::Error> {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(v) => output.push_str(if *v { "true" } else { "false" }),
        Value::Number(v) => {
            let number = v.as_f64().ok_or_else(|| {
                <serde_json::Error as serde::ser::Error>::custom("non-JSON number")
            })?;
            if !number.is_finite() {
                return Err(<serde_json::Error as serde::ser::Error>::custom(
                    "non-finite JSON number",
                ));
            }
            let mut buffer = ryu_js::Buffer::new();
            output.push_str(buffer.format(number));
        }
        Value::String(v) => output.push_str(&serde_json::to_string(v)?),
        Value::Array(values) => {
            output.push('[');
            for (i, item) in values.iter().enumerate() {
                if i > 0 {
                    output.push(',');
                }
                if indent != 0 {
                    output.push('\n');
                    output.push_str(&" ".repeat((depth + 1) * indent));
                }
                write_js_value(item, depth + 1, indent, output)?;
            }
            if !values.is_empty() && indent != 0 {
                output.push('\n');
                output.push_str(&" ".repeat(depth * indent));
            }
            output.push(']');
        }
        Value::Object(map) => {
            // serde_json preserve_order retains the insertion sequence for
            // non-index keys; ECMAScript enumerates numeric keys before them.
            let mut numeric = Vec::new();
            let mut ordinary = Vec::new();
            for key in map.keys() {
                if let Some(index) = js_property_index(key) {
                    numeric.push((index, key.as_str()));
                } else {
                    ordinary.push(key.as_str());
                }
            }
            numeric.sort_unstable_by_key(|(index, _)| *index);
            let keys: Vec<&str> = numeric
                .into_iter()
                .map(|(_, key)| key)
                .chain(ordinary)
                .collect();
            output.push('{');
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    output.push(',');
                }
                if indent != 0 {
                    output.push('\n');
                    output.push_str(&" ".repeat((depth + 1) * indent));
                }
                output.push_str(&serde_json::to_string(key)?);
                output.push(':');
                if indent != 0 {
                    output.push(' ');
                }
                write_js_value(&map[*key], depth + 1, indent, output)?;
            }
            if !keys.is_empty() && indent != 0 {
                output.push('\n');
                output.push_str(&" ".repeat(depth * indent));
            }
            output.push('}');
        }
    }
    Ok(())
}

/// JS JSON.stringify(value, null, space) numeric-space contract.
/// Negative indent clamps to zero; positive values clamp to ten.
pub fn stringify_json(value: &Value, space: Option<i32>) -> Result<String, serde_json::Error> {
    let indent = space.unwrap_or(0).clamp(0, 10) as usize;
    let mut output = String::new();
    write_js_value(value, 0, indent, &mut output)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Map};
    #[test]
    fn json_domain_parsing_and_nested_pretty_printing() {
        assert_eq!(parse_json("null").unwrap(), Value::Null);
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
    #[test]
    fn preserves_object_insertion_order_and_js_array_index_order() {
        let source = parse_json(r#"{"z":1,"a":2,"nested":{"zz":1,"aa":2}}"#).unwrap();
        assert_eq!(
            stringify_json(&source, None).unwrap(),
            r#"{"z":1,"a":2,"nested":{"zz":1,"aa":2}}"#
        );
        let source =
            parse_json(r#"{"11":"eleven","2":"two","z":3,"01":"leading","0":"zero"}"#).unwrap();
        assert_eq!(
            stringify_json(&source, None).unwrap(),
            r#"{"0":"zero","2":"two","11":"eleven","z":3,"01":"leading"}"#
        );
        let mut manual = Map::new();
        manual.insert("10".into(), json!("ten"));
        manual.insert("2".into(), json!("two"));
        assert_eq!(
            stringify_json(&Value::Object(manual), None).unwrap(),
            r#"{"2":"two","10":"ten"}"#
        );
        assert_eq!(js_property_index("4294967295"), None);
        assert_eq!(js_property_index("01"), None);
        assert_eq!(js_property_index("4294967294"), Some(u32::MAX - 1));
    }
    #[test]
    fn formats_js_numbers_and_numeric_space_without_lossy_lexemes() {
        let source = parse_json(r#"[-0.0,1.0,1e20,1e-7,1e-6]"#).unwrap();
        assert_eq!(
            stringify_json(&source, None).unwrap(),
            "[0,1,100000000000000000000,1e-7,0.000001]"
        );
        assert_eq!(
            stringify_json(&source, Some(1)).unwrap(),
            "[\n 0,\n 1,\n 100000000000000000000,\n 1e-7,\n 0.000001\n]"
        );
        assert_eq!(stringify_json(&json!({}), Some(3)).unwrap(), "{}");
        assert_eq!(stringify_json(&json!([]), Some(3)).unwrap(), "[]");
        assert_eq!(
            stringify_json(&json!({"a":1}), Some(500)).unwrap(),
            "{\n          \"a\": 1\n}"
        );
    }
}
