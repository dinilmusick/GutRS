//! Schema-aware semantic input resolution with an explicit, real model host.
//!
//! Port of GutTS semantic-input/index.ts at pinned
//! 3e2cbc0990ba29830ce91c5d1a1d552675d5334c.
//! Deterministic resolution is entirely portable. Ranking and model inference
//! require a caller-supplied, working inference runtime; never return fake scores.
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, VecDeque};

pub type SemanticInputSchema = Value;
pub type SemanticInputContext = Value;
pub type SemanticInputCacheKey = String;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_confirmation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_policy: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputCandidate {
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relevance: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputClarification {
    pub question: String,
    pub candidates: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputError {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputResolution {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_input: Option<String>,
    pub candidates: Vec<SemanticInputCandidate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_output: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clarification: Option<SemanticInputClarification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<SemanticInputError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_confirmation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Value>,
}
impl SemanticInputResolution {
    fn resolved(value: Value) -> Self {
        Self {
            status: "resolved".into(),
            model_output: Some(value.clone()),
            value: Some(value),
            raw_input: None,
            candidates: vec![],
            evidence: None,
            clarification: None,
            error: None,
            requires_confirmation: None,
            provenance: None,
        }
    }
    fn failed(status: &str, code: &str, message: &str, raw_input: Option<String>) -> Self {
        Self {
            status: status.into(),
            value: None,
            raw_input,
            candidates: vec![],
            model_output: None,
            evidence: None,
            clarification: None,
            error: Some(SemanticInputError {
                code: code.into(),
                message: message.into(),
            }),
            requires_confirmation: None,
            provenance: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputRequest {
    pub input: String,
    pub schema: SemanticInputSchema,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<SemanticInputContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_raw_input: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_candidate_similarity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_candidate_margin: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<SemanticInputPolicy>,
}
pub struct SemanticInputCache {
    max_entries: usize,
    order: VecDeque<String>,
    entries: HashMap<String, SemanticInputResolution>,
}
impl Default for SemanticInputCache {
    fn default() -> Self {
        Self::new(128)
    }
}
impl SemanticInputCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            order: VecDeque::new(),
            entries: HashMap::new(),
        }
    }
    pub fn get(&mut self, key: &str) -> Option<&SemanticInputResolution> {
        if self.entries.contains_key(key) {
            self.order.retain(|existing| existing != key);
            self.order.push_back(key.into());
        }
        self.entries.get(key)
    }
    pub fn set(&mut self, key: String, value: SemanticInputResolution) {
        self.order.retain(|existing| existing != &key);
        self.entries.remove(&key);
        self.order.push_back(key.clone());
        self.entries.insert(key, value);
        while self.order.len() > self.max_entries {
            if let Some(oldest) = self.order.pop_front() {
                self.entries.remove(&oldest);
            }
        }
    }
    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
fn js_trim(value: &str) -> &str {
    value.trim_matches(|c: char| {
        c == '\u{feff}'
            || matches!(c,'\u{0009}'..='\u{000d}'|'\u{0020}'|'\u{00a0}'|'\u{1680}'|
        '\u{2000}'..='\u{200a}'|'\u{2028}'..='\u{2029}'|'\u{202f}'|'\u{205f}'|'\u{3000}')
    })
}
fn js_number(text: &str) -> Option<f64> {
    let number = if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok().map(|v| v as f64)?
    } else if let Some(binary) = text.strip_prefix("0b").or_else(|| text.strip_prefix("0B")) {
        u64::from_str_radix(binary, 2).ok().map(|v| v as f64)?
    } else if let Some(octal) = text.strip_prefix("0o").or_else(|| text.strip_prefix("0O")) {
        u64::from_str_radix(octal, 8).ok().map(|v| v as f64)?
    } else {
        text.parse::<f64>().ok()?
    };
    number.is_finite().then_some(number)
}
fn deterministic_value(input: &str, schema: &Value) -> Option<Value> {
    let text = js_trim(input);
    if schema
        .get("enum")
        .and_then(Value::as_array)
        .is_some_and(|entries| {
            entries
                .iter()
                .any(|candidate| candidate.as_str() == Some(text))
        })
    {
        return Some(json!(text));
    }
    match schema.get("type").and_then(Value::as_str) {
        Some("string") => {
            let format = schema.get("format").and_then(Value::as_str);
            let matches = match format {
                Some("email") => Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
                    .ok()?
                    .is_match(text),
                Some("uri") => Regex::new(r"(?i)^https?://").ok()?.is_match(text),
                Some("date") => Regex::new(r"^\d{4}-\d{2}-\d{2}$").ok()?.is_match(text),
                Some("date-time") => Regex::new(r"^\d{4}-\d{2}-\d{2}T").ok()?.is_match(text),
                _ => false,
            };
            if matches {
                return Some(json!(text));
            }
            if let Some(pattern) = schema.get("pattern").and_then(Value::as_str) {
                if Regex::new(pattern).ok()?.is_match(text) {
                    return Some(json!(text));
                }
            }
            None
        }
        Some("number") => js_number(text).map(|v| json!(v)),
        Some("integer") => js_number(text).filter(|v| v.fract() == 0.0).map(|v| {
            // Preserve the JSON integer wire type for exact integers in
            // JavaScript's safe-integer interval. Larger values retain
            // their f64 representation rather than silently truncating.
            if v.abs() <= 9_007_199_254_740_991.0 {
                json!(v as i64)
            } else {
                json!(v)
            }
        }),
        Some("boolean") => {
            if text.eq_ignore_ascii_case("true") {
                Some(json!(true))
            } else if text.eq_ignore_ascii_case("false") {
                Some(json!(false))
            } else {
                None
            }
        }
        _ => None,
    }
}
/// Only returns a value when the real upstream deterministic rules resolve it.
pub fn resolve_deterministic_semantic_input(
    input: &str,
    schema: &Value,
) -> Option<SemanticInputResolution> {
    if js_trim(input).is_empty() {
        return None;
    }
    deterministic_value(input, schema).map(SemanticInputResolution::resolved)
}
pub trait SemanticInputRuntime {
    type Error: std::fmt::Display;
    fn similarity(&self, left: &str, right: &str) -> Result<f64, Self::Error>;
    fn rerank(
        &self,
        model: &str,
        query: &str,
        candidates: &[String],
    ) -> Result<Vec<f64>, Self::Error>;
    fn complete(&self, prompt: &str, schema: &Value) -> Result<Value, Self::Error>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticRuntimeError(pub String);
impl std::fmt::Display for SemanticRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for SemanticRuntimeError {}

fn model_policy(policy: Option<&SemanticInputPolicy>) -> &str {
    policy
        .and_then(|p| p.model_policy.as_deref())
        .unwrap_or("balanced")
}
fn reranker_model(policy: &str) -> &'static str {
    match policy {
        "fast" => "EttinReranker17M",
        "accurate" => "EttinReranker68M",
        _ => "EttinReranker32M",
    }
}
/// Reproduce the exact prompt passed to the pinned GutTS Needle3 model host.
/// JSON is stringified with ECMAScript property-order and number rules.
pub fn build_semantic_prompt(request: &SemanticInputRequest) -> Result<String, SemanticRuntimeError> {
    let mut lines = vec![
        "Convert the natural-language input into the supplied target schema.".to_owned(),
        "Return only data conforming to the target schema. Never invent information.".to_owned(),
    ];
    if let Some(context) = request.context.as_ref() {
        for (field, label) in [
            ("application", "Application"),
            ("workspace", "Workspace"),
            ("page", "Page"),
            ("section", "Section"),
            ("form", "Form"),
            ("field", "Field"),
            ("label", "Label"),
            ("description", "Description"),
            ("semanticType", "Semantic type"),
        ] {
            if let Some(value) = context
                .get(field)
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
            {
                lines.push(format!("{label}: {value}"));
            }
        }
        for (field, label, separator) in [
            ("vocabulary", "Vocabulary", ", "),
            ("examples", "Examples", " | "),
            ("aliases", "Aliases", ", "),
            ("constraints", "Constraints", "; "),
            ("relatedFields", "Related fields", ", "),
        ] {
            if let Some(values) = context.get(field).and_then(Value::as_array) {
                if !values.is_empty() {
                    let strings: Vec<&str> = values.iter().filter_map(Value::as_str).collect();
                    lines.push(format!("{label}: {}", strings.join(separator)));
                }
            }
        }
        if let Some(locale) = context
            .get("locale")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
        {
            lines.push(format!("Locale: {locale}"));
        }
    }
    if let Some(candidates) = request.candidates.as_deref().filter(|values| !values.is_empty()) {
        lines.push(format!("Candidate values: {}", candidates.join(" | ")));
    }
    let schema = crate::json_utils::stringify_json(&request.schema, None)
        .map_err(|error| SemanticRuntimeError(format!("Invalid target schema: {error}")))?;
    lines.push(format!("Target schema: {schema}"));
    lines.push(format!("Input: {}", request.input));
    Ok(lines.join("\n"))
}

/// Requires actual MiniLM and Ettin runtimes provided by the caller.
pub fn rank_semantic_input_candidates<R: SemanticInputRuntime>(
    input: &str,
    candidates: &[String],
    policy: &str,
    runtime: &R,
) -> Result<Vec<SemanticInputCandidate>, SemanticRuntimeError> {
    if candidates.is_empty() {
        return Ok(vec![]);
    }
    let mut similarities = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        similarities.push(
            runtime
                .similarity(input, candidate)
                .map_err(|e| SemanticRuntimeError(e.to_string()))?,
        );
    }
    let relevance = runtime
        .rerank(reranker_model(policy), input, candidates)
        .map_err(|e| SemanticRuntimeError(e.to_string()))?;
    if relevance.len() != candidates.len() {
        return Err(SemanticRuntimeError(
            "Reranker score count differs from candidate count".into(),
        ));
    }
    let mut results: Vec<_> = candidates
        .iter()
        .enumerate()
        .map(|(idx, candidate)| SemanticInputCandidate {
            value: candidate.clone(),
            similarity: Some(similarities[idx]),
            relevance: Some(relevance[idx]),
        })
        .collect();
    results.sort_by(|a, b| {
        b.relevance
            .unwrap_or(f64::NEG_INFINITY)
            .total_cmp(&a.relevance.unwrap_or(f64::NEG_INFINITY))
            .then_with(|| {
                b.similarity
                    .unwrap_or(f64::NEG_INFINITY)
                    .total_cmp(&a.similarity.unwrap_or(f64::NEG_INFINITY))
            })
            .then_with(|| a.value.cmp(&b.value))
    });
    Ok(results)
}
fn clarification(
    candidates: &[SemanticInputCandidate],
    context: Option<&Value>,
) -> Option<SemanticInputClarification> {
    if candidates.len() < 2 {
        return None;
    }
    let top: Vec<String> = candidates.iter().take(3).map(|c| c.value.clone()).collect();
    let label = context
        .and_then(|o| o.get("label").or_else(|| o.get("field")))
        .and_then(Value::as_str)
        .unwrap_or("this field");
    Some(SemanticInputClarification {
        question: format!("Which value did you mean for {label}: {}?", top.join(", ")),
        candidates: top,
    })
}
/// Native synchronous equivalent with injected actual models, optional cache and
/// explicit cancellation. Model and ranking errors cannot silently succeed.
pub fn resolve_semantic_input<R: SemanticInputRuntime>(
    request: &SemanticInputRequest,
    runtime: &R,
    mut cache: Option<&mut SemanticInputCache>,
    aborted: bool,
) -> SemanticInputResolution {
    let raw = request
        .preserve_raw_input
        .unwrap_or(false)
        .then_some(request.input.clone());
    if aborted {
        let mut result = SemanticInputResolution::failed(
            "failed",
            "ABORTED",
            "Semantic resolution was aborted.",
            raw,
        );
        result.provenance = Some(json!(crate::semantic_types::semantic_input_provenance(
            request, false
        )));
        return result;
    }
    let cache_key = serde_json::to_string(request).unwrap_or_else(|_| request.input.clone());
    if let Some(value) = cache
        .as_deref_mut()
        .and_then(|c| c.get(&cache_key))
        .cloned()
    {
        return value;
    }
    if js_trim(&request.input).is_empty() {
        let result = SemanticInputResolution::failed(
            "invalid",
            "EMPTY_INPUT",
            "Semantic input is empty.",
            raw,
        );
        if let Some(cache) = cache.as_deref_mut() {
            cache.set(cache_key, result.clone());
        }
        return result;
    }
    let requires_confirmation = request
        .policy
        .as_ref()
        .is_some_and(|p| p.require_confirmation == Some(true) || p.risk.as_deref() == Some("high"));
    if let Some(mut result) = resolve_deterministic_semantic_input(&request.input, &request.schema)
    {
        if requires_confirmation {
            result.requires_confirmation = Some(true);
        }
        result.provenance = Some(json!(crate::semantic_types::semantic_input_provenance(
            request, true
        )));
        if let Some(cache) = cache.as_deref_mut() {
            cache.set(cache_key, result.clone());
        }
        return result;
    }
    let vocabulary = request
        .candidates
        .as_deref()
        .filter(|items| !items.is_empty())
        .map(|items| items.to_vec())
        .or_else(|| {
            request
                .context
                .as_ref()
                .and_then(|c| c.get("vocabulary"))
                .and_then(Value::as_array)
                .map(|v| {
                    v.iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                })
        })
        .unwrap_or_default();
    let candidates = match rank_semantic_input_candidates(
        &request.input,
        &vocabulary,
        model_policy(request.policy.as_ref()),
        runtime,
    ) {
        Ok(candidates) => candidates,
        Err(error) => {
            let mut result =
                SemanticInputResolution::failed("failed", "MODEL_FAILURE", &error.0, raw);
            result.provenance = Some(json!(crate::semantic_types::semantic_input_provenance(
                request, false
            )));
            return result;
        }
    };
    let prompt = match build_semantic_prompt(request) {
        Ok(prompt) => prompt,
        Err(error) => {
            let mut result = SemanticInputResolution::failed(
                "invalid",
                "SCHEMA_MISMATCH",
                &error.to_string(),
                raw,
            );
            result.provenance = Some(json!(crate::semantic_types::semantic_input_provenance(
                request, false
            )));
            return result;
        }
    };
    let output = match runtime.complete(&prompt, &request.schema) {
        Ok(value) => value,
        Err(error) => {
            let mut result =
                SemanticInputResolution::failed("failed", "MODEL_FAILURE", &error.to_string(), raw);
            result.provenance = Some(json!(crate::semantic_types::semantic_input_provenance(
                request, false
            )));
            return result;
        }
    };
    let model_output = ["output", "result", "data", "structuredOutput"]
        .iter()
        .find_map(|k| output.as_object().and_then(|o| o.get(*k)).cloned())
        .unwrap_or(output);
    if model_output.is_null() || model_output == json!("") {
        let mut result = SemanticInputResolution::failed(
            "unresolved",
            "MODEL_OUTPUT_EMPTY",
            "Needle returned no usable semantic value.",
            raw,
        );
        result.model_output = Some(model_output);
        result.candidates = candidates;
        result.provenance = Some(json!(crate::semantic_types::semantic_input_provenance(
            request, false
        )));
        return result;
    }
    let mut selected_candidate = None;
    let value = if !candidates.is_empty() && model_output.is_string() {
        let text = model_output.as_str().unwrap_or("");
        if let Some(candidate) = candidates.iter().find(|c| c.value == text) {
            selected_candidate = Some(candidate.value.clone());
            Some(json!(candidate.value))
        } else {
            let top = candidates.first();
            let second = candidates.get(1);
            let min_similarity = request.min_candidate_similarity.unwrap_or(0.65);
            let min_margin = request.min_candidate_margin.unwrap_or(0.15);
            let passes = top
                .is_some_and(|c| c.similarity.unwrap_or(f64::NEG_INFINITY) >= min_similarity)
                && second.is_none_or(|s| {
                    top.and_then(|t| t.relevance).unwrap_or(f64::NEG_INFINITY)
                        - s.relevance.unwrap_or(f64::NEG_INFINITY)
                        >= min_margin
                });
            if passes {
                selected_candidate = top.map(|c| c.value.clone());
                top.map(|c| json!(c.value))
            } else {
                None
            }
        }
    } else {
        Some(model_output.clone())
    };
    let mut evidence = serde_json::Map::new();
    evidence.insert("needleOutput".into(), model_output.clone());
    evidence.insert("candidates".into(), json!(candidates));
    if let Some(selected) = selected_candidate
        .as_ref()
        .filter(|value| !value.is_empty())
    {
        evidence.insert("selectedCandidate".into(), json!(selected));
    }
    if !candidates.is_empty() {
        evidence.insert(
            "similarityThreshold".into(),
            json!(request.min_candidate_similarity.unwrap_or(0.65)),
        );
        evidence.insert(
            "relevanceMarginThreshold".into(),
            json!(request.min_candidate_margin.unwrap_or(0.15)),
        );
    }
    let evidence = Value::Object(evidence);
    let mut result = if let Some(value) = value {
        let mut result = SemanticInputResolution::resolved(value);
        result.raw_input = raw;
        if requires_confirmation {
            result.requires_confirmation = Some(true);
        }
        result
    } else {
        let mut result = SemanticInputResolution::failed(
            "ambiguous",
            "AMBIGUOUS_CANDIDATE",
            "Semantic evidence did not uniquely identify a candidate.",
            raw,
        );
        result.clarification = clarification(&candidates, request.context.as_ref());
        result
    };
    result.candidates = candidates;
    result.model_output = Some(model_output);
    result.evidence = Some(evidence);
    result.provenance = Some(json!(crate::semantic_types::semantic_input_provenance(
        request, false
    )));
    if let Some(cache) = cache {
        cache.set(cache_key, result.clone());
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Model;
    impl SemanticInputRuntime for Model {
        type Error = SemanticRuntimeError;
        fn similarity(&self, _: &str, right: &str) -> Result<f64, Self::Error> {
            Ok(if right == "guitar" { 0.9 } else { 0.2 })
        }
        fn rerank(&self, _: &str, _: &str, candidates: &[String]) -> Result<Vec<f64>, Self::Error> {
            Ok(candidates
                .iter()
                .map(|c| if c == "guitar" { 0.95 } else { 0.1 })
                .collect())
        }
        fn complete(&self, _: &str, _: &Value) -> Result<Value, Self::Error> {
            Ok(json!("guitar"))
        }
    }
    fn req(input: &str, schema: Value) -> SemanticInputRequest {
        SemanticInputRequest {
            input: input.into(),
            schema,
            context: None,
            candidates: None,
            preserve_raw_input: None,
            min_candidate_similarity: None,
            min_candidate_margin: None,
            policy: None,
        }
    }
    #[test]
    fn upstream_deterministic_cases() {
        assert_eq!(
            resolve_deterministic_semantic_input(" true ", &json!({"type":"boolean"}))
                .unwrap()
                .value,
            Some(json!(true))
        );
        assert_eq!(
            resolve_deterministic_semantic_input("0x10", &json!({"type":"integer"}))
                .unwrap()
                .value,
            Some(json!(16))
        );
        assert_eq!(
            resolve_deterministic_semantic_input(
                "person@example.com",
                &json!({"type":"string","format":"email"})
            )
            .unwrap()
            .value,
            Some(json!("person@example.com"))
        );
        assert!(resolve_deterministic_semantic_input("bad", &json!({"type":"integer"})).is_none());
        assert!(resolve_deterministic_semantic_input("  ", &json!({"type":"string"})).is_none());
        assert!(resolve_deterministic_semantic_input("hello", &json!({"type":"string"})).is_none());
        assert_eq!(
            resolve_deterministic_semantic_input(
                "green",
                &json!({"type":"string","enum":["red","green"]})
            )
            .unwrap()
            .value,
            Some(json!("green"))
        );
    }
    #[test]
    fn real_adapter_dispatch_cache_and_high_risk_confirmation() {
        let mut request = req("play my instrument", json!({"type":"string"}));
        request.candidates = Some(vec!["piano".into(), "guitar".into()]);
        request.policy = Some(SemanticInputPolicy {
            risk: Some("high".into()),
            ..Default::default()
        });
        let mut cache = SemanticInputCache::new(1);
        let resolved = resolve_semantic_input(&request, &Model, Some(&mut cache), false);
        assert_eq!(resolved.value, Some(json!("guitar")));
        assert_eq!(resolved.requires_confirmation, Some(true));
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.get("missing"), None);
        assert_eq!(
            resolve_semantic_input(&request, &Model, Some(&mut cache), false),
            resolved
        );
        let aborted = resolve_semantic_input(&request, &Model, None, true);
        assert_eq!(aborted.error.unwrap().code, "ABORTED");
        cache.clear();
        assert_eq!(cache.size(), 0);
    }
    #[test]
    fn pinned_contextual_prompt_includes_all_fields_in_source_order() {
        let mut request = req("play the keys", json!({"type":"string","enum":["piano","guitar"]}));
        request.context = Some(json!({
            "application": "Kone",
            "workspace": "Performance",
            "page": "Live",
            "section": "Song",
            "form": "Sound selector",
            "field": "instrument",
            "label": "Instrument",
            "description": "Choose a sound",
            "semanticType": "musical-instrument",
            "vocabulary": ["piano", "guitar"],
            "examples": ["play piano", "try guitar"],
            "aliases": ["keys", "axe"],
            "constraints": ["choose one", "no drums"],
            "relatedFields": ["volume", "patch"],
            "locale": "en-IN"
        }));
        request.candidates = Some(vec!["piano".to_owned(), "guitar".to_owned()]);
        let expected = concat!(
            "Convert the natural-language input into the supplied target schema.\n",
            "Return only data conforming to the target schema. Never invent information.\n",
            "Application: Kone\n",
            "Workspace: Performance\n",
            "Page: Live\n",
            "Section: Song\n",
            "Form: Sound selector\n",
            "Field: instrument\n",
            "Label: Instrument\n",
            "Description: Choose a sound\n",
            "Semantic type: musical-instrument\n",
            "Vocabulary: piano, guitar\n",
            "Examples: play piano | try guitar\n",
            "Aliases: keys, axe\n",
            "Constraints: choose one; no drums\n",
            "Related fields: volume, patch\n",
            "Locale: en-IN\n",
            "Candidate values: piano | guitar\n",
            "Target schema: {\"type\":\"string\",\"enum\":[\"piano\",\"guitar\"]}\n",
            "Input: play the keys"
        );
        assert_eq!(build_semantic_prompt(&request).unwrap(), expected);
        struct Capture(std::sync::Mutex<Vec<String>>);
        impl SemanticInputRuntime for Capture {
            type Error = SemanticRuntimeError;
            fn similarity(&self, _: &str, _: &str) -> Result<f64, Self::Error> {
                Ok(0.9)
            }
            fn rerank(&self, _: &str, _: &str, candidates: &[String]) -> Result<Vec<f64>, Self::Error> {
                Ok(vec![0.9; candidates.len()])
            }
            fn complete(&self, prompt: &str, _: &Value) -> Result<Value, Self::Error> {
                self.0.lock().unwrap().push(prompt.to_owned());
                Ok(json!("guitar"))
            }
        }
        let runtime = Capture(std::sync::Mutex::new(Vec::new()));
        let result = resolve_semantic_input(&request, &runtime, None, false);
        assert_eq!(result.status, "resolved");
        assert_eq!(result.value, Some(json!("guitar")));
        assert_eq!(runtime.0.lock().unwrap().as_slice(), &[expected.to_owned()]);
    }
    #[test]
    fn prompt_without_context_preserves_minimal_pinned_order() {
        let request = req("hello", json!({"type":"string"}));
        assert_eq!(
            build_semantic_prompt(&request).unwrap(),
            concat!(
                "Convert the natural-language input into the supplied target schema.\n",
                "Return only data conforming to the target schema. Never invent information.\n",
                "Target schema: {\"type\":\"string\"}\n",
                "Input: hello"
            )
        );
    }

}
