//! JSON-object equivalents for GutTS object utilities.
//! No JavaScript prototype chain exists in this representation.
use serde_json::{Map, Value};

pub fn is_plain_object(value: &Value) -> bool {
    value.is_object()
}

/// Tests an own field (not an inherited JavaScript prototype property).
pub fn has_own_field(value: &Value, key: &str) -> bool {
    value
        .as_object()
        .is_some_and(|record| record.contains_key(key))
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
        assert_eq!(
            pick_fields(&value, &["b", "missing"]),
            Some(json!({"b":{"c":2}}))
        );
        assert_eq!(
            omit_fields(&value, &["a"]),
            Some(json!({"b":{"c":2},"__proto__":"literal"}))
        );
        assert_eq!(pick_fields(&json!(null), &["x"]), None);
    }
}


/// Parse lodash-compatible own-JSON property paths, including dot notation,
/// numeric array subscripts and quoted bracket property names.
/// If an exact literal field exists (e.g. "a.b"), lodash prefers that field.
fn property_segments(root: &Value, path: &str) -> Option<Vec<String>> {
    if root.as_object().is_some_and(|o| o.contains_key(path)) {
        return Some(vec![path.to_owned()]);
    }
    let chars: Vec<char> = path.chars().collect();
    let mut segments = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '.' {
            i += 1;
            continue;
        }
        if chars[i] == '[' {
            i += 1;
            if i >= chars.len() { return None; }
            if matches!(chars[i], '"' | '\'') {
                let quote = chars[i];
                i += 1;
                let mut key = String::new();
                let mut closed = false;
                while i < chars.len() {
                    let ch = chars[i];
                    i += 1;
                    if ch == '\\' {
                        if i >= chars.len() { return None; }
                        key.push(chars[i]);
                        i += 1;
                    } else if ch == quote {
                        closed = true;
                        break;
                    } else {
                        key.push(ch);
                    }
                }
                if !closed || chars.get(i) != Some(&']') { return None; }
                i += 1;
                segments.push(key);
            } else {
                let mut key = String::new();
                while i < chars.len() && chars[i] != ']' {
                    key.push(chars[i]);
                    i += 1;
                }
                if chars.get(i) != Some(&']') { return None; }
                i += 1;
                if key.is_empty() { return None; }
                segments.push(key);
            }
            continue;
        }
        let mut key = String::new();
        while i < chars.len() && chars[i] != '.' && chars[i] != '[' {
            key.push(chars[i]);
            i += 1;
        }
        if key.is_empty() { return None; }
        segments.push(key);
    }
    if segments.is_empty() { None } else { Some(segments) }
}

fn array_index(segment: &str) -> Option<usize> {
    if segment != "0" && segment.starts_with('0') { return None; }
    if segment.starts_with('-') || segment.starts_with('+') { return None; }
    segment.parse::<usize>().ok()
}
fn get_path<'a>(root: &'a Value, path: &[String]) -> Option<&'a Value> {
    let mut current = root;
    for segment in path {
        current = match current {
            Value::Object(map) => map.get(segment)?,
            Value::Array(items) => items.get(array_index(segment)?)?,
            _ => return None,
        };
    }
    Some(current)
}
fn set_path(output: &mut Value, path: &[String], value: &Value) {
    let Some(segment) = path.first() else {
        *output = value.clone();
        return;
    };
    if output.is_null() {
        *output = if array_index(segment).is_some() {
            Value::Array(Vec::new())
        } else {
            Value::Object(Map::new())
        };
    }
    match output {
        Value::Object(map) => {
            let item = map.entry(segment.clone()).or_insert(Value::Null);
            set_path(item, &path[1..], value);
        }
        Value::Array(items) => {
            let Some(index) = array_index(segment) else { return; };
            // Only source-present indices reach this helper; bound allocations
            // to a finite ceiling to avoid allocating huge sparse arrays.
            if index > 100_000 { return; }
            if items.len() <= index { items.resize(index + 1, Value::Null); }
            set_path(&mut items[index], &path[1..], value);
        }
        _ => {}
    }
}
fn unset_path(output: &mut Value, path: &[String]) {
    let Some(segment) = path.first() else { return; };
    if path.len() == 1 {
        match output {
            Value::Object(map) => { map.remove(segment); }
            Value::Array(items) => {
                if let Some(index) = array_index(segment) {
                    if let Some(item) = items.get_mut(index) { *item = Value::Null; }
                }
            }
            _ => {}
        }
        return;
    }
    match output {
        Value::Object(map) => {
            if let Some(item) = map.get_mut(segment) { unset_path(item, &path[1..]); }
        }
        Value::Array(items) => {
            if let Some(item) = array_index(segment).and_then(|index| items.get_mut(index)) {
                unset_path(item, &path[1..]);
            }
        }
        _ => {}
    }
}

/// Actual nested-path equivalent of GutTS hasOwn (which delegates to lodash.has)
/// within the JSON own-property domain. JavaScript prototype chains are absent.
pub fn has_property_path(value: &Value, key: &str) -> bool {
    property_segments(value, key).and_then(|s| get_path(value, &s)).is_some()
}

/// Project nested, literal, quoted and array-index paths into a new JSON object.
pub fn pick_property_paths(value: &Value, keys: &[&str]) -> Option<Value> {
    value.as_object()?;
    let mut result = Value::Object(Map::new());
    for &key in keys {
        let Some(segments) = property_segments(value, key) else { continue; };
        if let Some(selected) = get_path(value, &segments) {
            set_path(&mut result, &segments, selected);
        }
    }
    Some(result)
}

/// Delete nested own fields from a deep clone, retaining all other values.
/// For JSON serialization, a removed array index becomes null (a JS sparse hole).
pub fn omit_property_paths(value: &Value, keys: &[&str]) -> Option<Value> {
    value.as_object()?;
    let mut result = value.clone();
    for &key in keys {
        if let Some(segments) = property_segments(value, key) {
            unset_path(&mut result, &segments);
        }
    }
    Some(result)
}

#[cfg(test)]
mod deep_path_tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn paths_are_shallow_or_nested_based_on_actual_source() {
        let value = json!({"a":{"b":7,"c":8},"a.b":"literal",
            "x":[{"name":"first"},{"name":"second"}],"s":{"a.b":"quoted"}});
        assert!(has_property_path(&value, "a.b"));
        assert!(has_property_path(&value, "x[0].name"));
        assert!(has_property_path(&value, "s[\"a.b\"]"));
        assert!(!has_property_path(&value, "x[2].name"));
        assert!(!has_property_path(&value, "x[-1].name"));
        assert_eq!(pick_property_paths(&value, &["a.b"]),Some(json!({"a.b":"literal"})));
        assert_eq!(pick_property_paths(&value, &["x[1].name"]),Some(json!({"x":[null,{"name":"second"}]})));
        assert_eq!(pick_property_paths(&value, &["s['a.b']"]),Some(json!({"s":{"a.b":"quoted"}})));
        assert_eq!(omit_property_paths(&value, &["a.b","x[0].name"]),Some(json!({
            "a":{"b":7,"c":8},"x":[{},{"name":"second"}],"s":{"a.b":"quoted"}
        })));
    }
    #[test]
    fn missing_paths_and_malformed_brackets_do_not_mutate_source() {
        let source = json!({"a":{"b":1,"c":2},"v":false});
        assert_eq!(pick_property_paths(&source, &["a.b"]),Some(json!({"a":{"b":1}})));
        assert_eq!(omit_property_paths(&source, &["a.b"]),Some(json!({"a":{"c":2},"v":false})));
        assert_eq!(pick_property_paths(&source, &["a["]),Some(json!({})));
        assert_eq!(omit_property_paths(&source, &["a[","missing.x"]),Some(source.clone()));
        assert!(!has_property_path(&source, "a["));
        assert_eq!(source["a"]["b"],1);
    }
}
