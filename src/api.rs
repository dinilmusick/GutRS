//! Executable GutTS API registry and typed Feature/Operation Definition (FOD)
//! nodes. Metadata is extracted from the PINNED TypeScript defineApi objects.
//! Missing runtime adapters fail explicitly; model inference is never invented.
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::sync::OnceLock;

pub type JsonSchema = Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InvalidManifest(String),
    UnknownApi(String),
    NoRuntimeAdapter(String),
    InvalidArguments(String),
    Operation(String),
}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownApi(id) => write!(f, "Unknown GutTS API '{id}'."),
            Self::NoRuntimeAdapter(id) => write!(
                f,
                "GutTS API '{id}' has metadata but no embedded runtime adapter."
            ),
            Self::InvalidManifest(reason) => write!(f, "Invalid pinned API manifest: {reason}"),
            Self::InvalidArguments(reason) => write!(f, "Invalid API arguments: {reason}"),
            Self::Operation(reason) => write!(f, "GutTS operation failed: {reason}"),
        }
    }
}
impl std::error::Error for ApiError {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiParameter {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub schema: JsonSchema,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSemantics {
    pub deterministic: bool,
    pub pure: bool,
    pub side_effects: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiExample {
    pub input: Map<String, Value>,
    pub output: Value,
}

/// Input to GutTS defineApi (without the generated inputSchema).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiBlueprint {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub parameters: Vec<ApiParameter>,
    pub output: JsonSchema,
    pub semantics: ApiSemantics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<ApiExample>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiDefinition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub parameters: Vec<ApiParameter>,
    pub output: JsonSchema,
    pub semantics: ApiSemantics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<ApiExample>>,
    pub input_schema: JsonSchema,
}

/// Match GutTS defineApi: augment each parameter's schema with description and
/// build required names in source order, rejecting additional properties.
pub fn define_api(blueprint: ApiBlueprint) -> ApiDefinition {
    let mut properties = Map::new();
    let mut required = Vec::new();
    for parameter in &blueprint.parameters {
        let mut schema = parameter.schema.clone();
        if let Some(object) = schema.as_object_mut() {
            object.insert(
                "description".into(),
                Value::String(parameter.description.clone()),
            );
        }
        properties.insert(parameter.name.clone(), schema);
        if parameter.required {
            required.push(parameter.name.clone());
        }
    }
    let input_schema = json!({
        "type":"object",
        "description":format!("Arguments for {}.",blueprint.name),
        "properties":properties,
        "required":required,
        "additionalProperties":false
    });
    ApiDefinition {
        id: blueprint.id,
        name: blueprint.name,
        category: blueprint.category,
        description: blueprint.description,
        parameters: blueprint.parameters,
        output: blueprint.output,
        semantics: blueprint.semantics,
        errors: blueprint.errors,
        examples: blueprint.examples,
        input_schema,
    }
}

static DEFINITIONS: OnceLock<Result<Vec<ApiDefinition>, ApiError>> = OnceLock::new();

pub fn gutts_api_definitions() -> Result<&'static [ApiDefinition], ApiError> {
    let definitions = DEFINITIONS.get_or_init(|| {
        let blueprints: Vec<ApiBlueprint> = serde_json::from_str(include_str!(
            "../testdata/upstream-golden/api-blueprints.json"
        ))
        .map_err(|e| ApiError::InvalidManifest(e.to_string()))?;
        if blueprints.len() != 38 {
            return Err(ApiError::InvalidManifest(format!(
                "expected 38 pinned exported APIs, got {}",
                blueprints.len()
            )));
        }
        let mut ids = std::collections::HashSet::new();
        if !blueprints.iter().all(|entry| ids.insert(entry.id.clone())) {
            return Err(ApiError::InvalidManifest(
                "duplicate API identifiers".into(),
            ));
        }
        Ok(blueprints.into_iter().map(define_api).collect())
    });
    definitions
        .as_ref()
        .map(Vec::as_slice)
        .map_err(Clone::clone)
}

pub fn gutts_api_by_id(id: &str) -> Result<Option<&'static ApiDefinition>, ApiError> {
    Ok(gutts_api_definitions()?
        .iter()
        .find(|definition| definition.id == id))
}

pub fn gutts_api_to_fod_node(api: &ApiDefinition) -> Value {
    let inputs: Vec<Value> = api
        .parameters
        .iter()
        .map(|parameter| {
            json!({
                "id":parameter.name,
                "name":parameter.name,
                "direction":"input",
                "dataType":parameter.schema.get("type").and_then(Value::as_str).unwrap_or("any"),
                "required":parameter.required,
                "description":parameter.description,
                "schema":parameter.schema
            })
        })
        .collect();
    let mut result = json!({
        "id":"result",
        "name":"result",
        "direction":"output",
        "dataType":api.output.get("type").and_then(Value::as_str).unwrap_or("any"),
        "schema":api.output
    });
    if let Some(description) = api.output.get("description") {
        result["description"] = description.clone();
    }
    json!({
        "type":format!("gutts/{}",api.id),
        "label":api.name,
        "description":api.description,
        "category":"gutts",
        "group":api.category,
        "inputs":inputs,
        "outputs":[result],
        "metadata":{"apiId":api.id,"package":"@dinilmusick/gutts","semantics":api.semantics}
    })
}

pub fn gutts_fod_node_definitions() -> Result<Vec<Value>, ApiError> {
    Ok(gutts_api_definitions()?
        .iter()
        .map(gutts_api_to_fod_node)
        .collect())
}

fn require<'a>(input: &'a Value, key: &str) -> Result<&'a Value, ApiError> {
    input
        .as_object()
        .and_then(|record| record.get(key))
        .ok_or_else(|| ApiError::InvalidArguments(format!("missing field '{key}'")))
}
fn string<'a>(input: &'a Value, key: &str) -> Result<&'a str, ApiError> {
    require(input, key)?
        .as_str()
        .ok_or_else(|| ApiError::InvalidArguments(format!("'{key}' must be string")))
}
fn number(input: &Value, key: &str) -> Result<f64, ApiError> {
    require(input, key)?
        .as_f64()
        .ok_or_else(|| ApiError::InvalidArguments(format!("'{key}' must be number")))
}
fn array<'a>(input: &'a Value, key: &str) -> Result<&'a [Value], ApiError> {
    require(input, key)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| ApiError::InvalidArguments(format!("'{key}' must be array")))
}
fn strings<'a>(input: &'a Value, key: &str) -> Result<Vec<&'a str>, ApiError> {
    array(input, key)?
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| ApiError::InvalidArguments(format!("'{key}' contains non-string")))
        })
        .collect()
}
fn numbers(input: &Value, key: &str) -> Result<Vec<f64>, ApiError> {
    array(input, key)?
        .iter()
        .map(|v| {
            v.as_f64()
                .ok_or_else(|| ApiError::InvalidArguments(format!("'{key}' contains non-number")))
        })
        .collect()
}
fn js_truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|v| v != 0.0 && !v.is_nan()),
        Value::String(s) => !s.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

fn execute_utility(name: &str, input: &Value) -> Result<Option<Value>, ApiError> {
    use crate::{
        json_utils, numbers as native_numbers, objects, strings as native_strings, truncate,
        url_operations, urls,
    };
    let result = match name {
        "compact" => json!(array(input, "values")?
            .iter()
            .filter(|v| js_truthy(v))
            .cloned()
            .collect::<Vec<_>>()),
        "unique" => {
            let mut output = Vec::<Value>::new();
            let mut seen = Vec::<Value>::new();
            for value in array(input, "values")? {
                // References are not representable in plain JSON. Distinct
                // object/array elements must not collapse via deep equality.
                if value.is_object() || value.is_array() || !seen.contains(value) {
                    if !value.is_object() && !value.is_array() {
                        seen.push(value.clone());
                    }
                    output.push(value.clone());
                }
            }
            json!(output)
        }
        "chunk" => {
            let values = array(input, "values")?;
            let size = number(input, "size")?;
            if !size.is_finite() || size <= 0.0 || size.fract() != 0.0 {
                return Err(ApiError::Operation(
                    "size must be a positive integer".into(),
                ));
            }
            let max = if size >= values.len() as f64 {
                values.len().max(1)
            } else {
                size as usize
            };
            json!(values.chunks(max).collect::<Vec<_>>())
        }
        "flatten" => {
            let mut output = Vec::new();
            for value in array(input, "values")? {
                if let Some(items) = value.as_array() {
                    output.extend(items.iter().cloned());
                } else {
                    output.push(value.clone());
                }
            }
            json!(output)
        }
        "last" => json!(array(input, "values")?.last()),
        "isJsonValue" => json!(true), // typed JSON input; other JS values cannot cross boundary.
        "parseJson" => json_utils::parse_json(string(input, "text")?)
            .map_err(|e| ApiError::Operation(e.to_string()))?,
        "tryParseJson" => json!(json_utils::try_parse_json(string(input, "text")?)),
        "stringifyJson" => {
            let spaces = input.get("space").and_then(Value::as_f64).map(|n| n as i32);
            json!(json_utils::stringify_json(require(input, "value")?, spaces)
                .map_err(|e| ApiError::Operation(e.to_string()))?)
        }
        "clamp" => json!(native_numbers::clamp(
            number(input, "value")?,
            number(input, "min")?,
            number(input, "max")?
        )
        .map_err(|e| ApiError::Operation(format!("{e:?}")))?),
        "inRange" => json!(native_numbers::in_range(
            number(input, "value")?,
            number(input, "min")?,
            number(input, "max")?
        )
        .map_err(|e| ApiError::Operation(format!("{e:?}")))?),
        "roundTo" => json!(native_numbers::round_to(
            number(input, "value")?,
            input.get("decimals").and_then(Value::as_f64).unwrap_or(0.0)
        )
        .map_err(|e| ApiError::Operation(format!("{e:?}")))?),
        "isFiniteNumber" => json!(input
            .get("value")
            .and_then(Value::as_f64)
            .is_some_and(f64::is_finite)),
        "sum" => json!(native_numbers::sum(&numbers(input, "values")?)),
        "hasOwn" => json!(objects::has_own_field(
            require(input, "value")?,
            string(input, "key")?
        )),
        "pick" => json!(objects::pick_fields(
            require(input, "value")?,
            &strings(input, "keys")?
        )),
        "omit" => json!(objects::omit_fields(
            require(input, "value")?,
            &strings(input, "keys")?
        )),
        "isPlainObject" => json!(objects::is_plain_object(require(input, "value")?)),
        "isBlank" => json!(native_strings::is_blank(string(input, "value")?)),
        "isNonBlank" => json!(native_strings::is_non_blank(string(input, "value")?)),
        "isEmail" => json!(crate::string_ext::is_email(string(input, "value")?)),
        "capitalize" => json!(crate::string_ext::capitalize(string(input, "value")?)),
        "camelCase" => json!(crate::string_ext::camel_case(string(input, "value")?)),
        "kebabCase" => json!(crate::string_ext::kebab_case(string(input, "value")?)),
        "snakeCase" => json!(crate::string_ext::snake_case(string(input, "value")?)),
        "startsWithAny" => json!(native_strings::starts_with_any(
            string(input, "value")?,
            &strings(input, "prefixes")?
        )),
        "endsWithAny" => json!(native_strings::ends_with_any(
            string(input, "value")?,
            &strings(input, "suffixes")?
        )),
        "truncate" => json!(truncate::truncate_numeric(
            string(input, "value")?,
            number(input, "maxLength")?,
            input.get("suffix").and_then(Value::as_str).unwrap_or("…")
        )
        .map_err(|e| ApiError::Operation(format!("{e:?}")))?),
        "truncateWords" => json!(native_strings::truncate_words(
            string(input, "value")?,
            number(input, "maxWords")?,
            input.get("suffix").and_then(Value::as_str).unwrap_or("…")
        )
        .map_err(|e| ApiError::Operation(format!("{e:?}")))?),
        "isUrl" => json!(url_operations::is_url(string(input, "value")?)),
        "joinUrl" => json!(urls::join_url(
            string(input, "base")?,
            &strings(input, "parts")?
        )),
        "getQueryParam" => json!(url_operations::get_query_param(
            string(input, "url")?,
            string(input, "key")?
        )
        .map_err(|e| ApiError::Operation(format!("{e:?}")))?),
        "setQueryParam" => json!(url_operations::set_query_param(
            string(input, "url")?,
            string(input, "key")?,
            string(input, "value")?
        )
        .map_err(|e| ApiError::Operation(format!("{e:?}")))?),
        _ => return Ok(None),
    };
    Ok(Some(result))
}

/// Execute a pinned GutTS metadata ID using actual Rust utility exports.
/// Model APIs and the not-yet-ported transformations return an explicit
/// NoRuntimeAdapter error; they never manufacture success.
pub fn execute_gutts_api(api_id: &str, input: &Value) -> Result<Value, ApiError> {
    let definition =
        gutts_api_by_id(api_id)?.ok_or_else(|| ApiError::UnknownApi(api_id.to_owned()))?;
    execute_utility(&definition.name, input)?
        .ok_or_else(|| ApiError::NoRuntimeAdapter(api_id.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exactly_38_metadata_exports_have_generated_valid_input_schemas() {
        let definitions = gutts_api_definitions().expect("pinned manifest parses");
        assert_eq!(definitions.len(), 38);
        for definition in definitions {
            assert_eq!(definition.input_schema["type"], "object");
            assert_eq!(definition.input_schema["additionalProperties"], false);
            assert_eq!(
                definition.input_schema["required"]
                    .as_array()
                    .unwrap()
                    .len(),
                definition.parameters.iter().filter(|v| v.required).count()
            );
            let round_trip = serde_json::to_value(definition).unwrap();
            assert_eq!(
                serde_json::from_value::<ApiDefinition>(round_trip).unwrap(),
                *definition
            );
        }
        assert_eq!(gutts_fod_node_definitions().unwrap().len(), 38);
        assert_eq!(
            gutts_api_to_fod_node(&definitions[0])["type"],
            "gutts/gutts.arrays.compact"
        );
    }
    #[test]
    fn actual_public_ids_dispatch_and_missing_adapters_fail_closed() {
        assert_eq!(
            execute_gutts_api("gutts.arrays.chunk", &json!({"values":[1,2,3],"size":2})).unwrap(),
            json!([[1, 2], [3]])
        );
        assert_eq!(
            execute_gutts_api(
                "gutts.urls.getQueryParam",
                &json!({"url":"https://example.com/?q=x+y","key":"q"})
            )
            .unwrap(),
            json!("x y")
        );
        assert_eq!(
            execute_gutts_api("gutts.strings.isBlank", &json!({"value":"\u{feff}"})).unwrap(),
            json!(true)
        );
        assert!(matches!(
            execute_gutts_api("gutts.arrays.chunk", &json!({"values":[1],"size":0})),
            Err(ApiError::Operation(_))
        ));
        assert!(matches!(
            execute_gutts_api("model.cactus-needle3.complete", &json!({})),
            Err(ApiError::NoRuntimeAdapter(_))
        ));
        assert!(matches!(
            execute_gutts_api("gutts.nonexistent", &json!({})),
            Err(ApiError::UnknownApi(_))
        ));
    }
}
