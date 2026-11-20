//! Portable typed semantic-input event, evidence, provenance and policy
//! contracts, transcribed from pinned GutTS semantic-input/index.ts.
//! A model/provenance store is always explicitly supplied by the host.
use crate::semantic_input::{SemanticInputCandidate, SemanticInputRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SemanticInputErrorCode {
    EmptyInput,
    ModelFailure,
    ModelOutputEmpty,
    SchemaMismatch,
    AmbiguousCandidate,
    UnsupportedType,
    Aborted,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SemanticInputRisk {
    Low,
    Medium,
    High,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SemanticInputModelPolicy {
    Fast,
    Balanced,
    Accurate,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SemanticInputPhase {
    Started,
    Deterministic,
    Ranking,
    Structured,
    Completed,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticInputProgress {
    pub phase: SemanticInputPhase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SemanticInputEvent {
    #[serde(rename = "semantic.started")]
    Started { input: String },
    #[serde(rename = "semantic.deterministic")]
    Deterministic { value: Value },
    #[serde(rename = "semantic.resolved")]
    Resolved { value: Value },
    #[serde(rename = "semantic.ambiguous")]
    Ambiguous { candidates: Vec<String> },
    #[serde(rename = "semantic.clarification.required")]
    ClarificationRequired { question: String },
    #[serde(rename = "semantic.invalid")]
    Invalid { code: SemanticInputErrorCode },
    #[serde(rename = "semantic.failed")]
    Failed { code: SemanticInputErrorCode },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputEvidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needle_output: Option<Value>,
    pub candidates: Vec<SemanticInputCandidate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_candidate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relevance_margin_threshold: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticInputProvenance {
    pub request_fingerprint: String,
    pub resolver: String,
    pub model_policy: SemanticInputModelPolicy,
    pub models: [String; 3],
    pub deterministic: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SemanticInputProvenanceRecord {
    pub provenance: SemanticInputProvenance,
    pub status: String,
    pub timestamp: String,
}
pub trait SemanticInputProvenanceStore {
    type Error: std::fmt::Display;
    fn append(&self, record: SemanticInputProvenanceRecord) -> Result<(), Self::Error>;
}

/// Deterministic GutTS FNV-1a fingerprint over JavaScript UTF-16 code units,
/// using the canonical property order defined in the pinned source function.
pub fn semantic_input_fingerprint(request: &SemanticInputRequest) -> String {
    let serialize = |value: &Value| serde_json::to_string(value).unwrap_or_default();
    let mut source = format!(
        "{{\"input\":{},\"schema\":{}",
        serialize(&Value::String(request.input.clone())),
        serialize(&request.schema)
    );
    if let Some(context) = &request.context {
        source.push_str(&format!(",\"context\":{}", serialize(context)));
    }
    if let Some(candidates) = &request.candidates {
        source.push_str(&format!(
            ",\"candidates\":{}",
            serialize(&serde_json::json!(candidates))
        ));
    }
    let policy = request
        .policy
        .as_ref()
        .and_then(|p| p.model_policy.as_deref())
        .unwrap_or("balanced");
    source.push_str(&format!(
        ",\"modelPolicy\":{}}}",
        serialize(&Value::String(policy.into()))
    ));
    let mut hash = 2166136261u32;
    for unit in source.encode_utf16() {
        hash ^= u32::from(unit);
        hash = hash.wrapping_mul(16777619);
    }
    format!("{hash:08x}")
}
pub fn semantic_input_provenance(
    request: &SemanticInputRequest,
    deterministic: bool,
) -> SemanticInputProvenance {
    let policy = request
        .policy
        .as_ref()
        .and_then(|p| p.model_policy.as_deref())
        .unwrap_or("balanced");
    let (policy, reranker) = match policy {
        "fast" => (SemanticInputModelPolicy::Fast, "EttinReranker17M"),
        "accurate" => (SemanticInputModelPolicy::Accurate, "EttinReranker68M"),
        _ => (SemanticInputModelPolicy::Balanced, "EttinReranker32M"),
    };
    SemanticInputProvenance {
        request_fingerprint: semantic_input_fingerprint(request),
        resolver: "GutTS.semantic-input".into(),
        model_policy: policy,
        models: ["Needle3".into(), "MiniLM".into(), reranker.into()],
        deterministic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_input::SemanticInputRequest;
    use serde_json::json;
    #[test]
    fn wire_events_preserve_exact_public_discriminators() {
        let event = SemanticInputEvent::Invalid {
            code: SemanticInputErrorCode::SchemaMismatch,
        };
        assert_eq!(
            serde_json::to_value(&event).unwrap(),
            json!({"type":"semantic.invalid","code":"SCHEMA_MISMATCH"})
        );
        assert_eq!(
            serde_json::from_value::<SemanticInputEvent>(
                json!({"type":"semantic.clarification.required","question":"Which?"})
            )
            .unwrap(),
            SemanticInputEvent::ClarificationRequired {
                question: "Which?".into()
            }
        );
        assert!(serde_json::from_value::<SemanticInputEvent>(
            json!({"type":"semantic.bypass","code":"OK"})
        )
        .is_err());
    }
    #[test]
    fn provenance_is_deterministic_and_model_policy_aware() {
        let request = SemanticInputRequest {
            input: "42".into(),
            schema: json!({"type":"integer"}),
            context: None,
            candidates: None,
            preserve_raw_input: None,
            min_candidate_similarity: None,
            min_candidate_margin: None,
            policy: None,
        };
        let provenance = semantic_input_provenance(&request, true);
        assert_eq!(provenance.resolver, "GutTS.semantic-input");
        assert_eq!(provenance.models[2], "EttinReranker32M");
        assert_eq!(provenance.request_fingerprint.len(), 8);
        assert_eq!(
            semantic_input_fingerprint(&request),
            provenance.request_fingerprint
        );
        assert_eq!(
            serde_json::to_value(&provenance).unwrap()["deterministic"],
            true
        );
    }
}
