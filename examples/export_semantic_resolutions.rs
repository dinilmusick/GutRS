//! Public Rust semantic resolution differential against pinned GutTS.
use gutrs::semantic_input::{resolve_semantic_input, SemanticInputRequest, SemanticInputRuntime};
use serde_json::{json, Value};

struct UnavailableModel;
impl SemanticInputRuntime for UnavailableModel {
    type Error = std::io::Error;
    fn similarity(&self, _: &str, _: &str) -> Result<f64, Self::Error> {
        Err(std::io::Error::other("inference forbidden in deterministic cases"))
    }
    fn rerank(&self, _: &str, _: &str, _: &[String]) -> Result<Vec<f64>, Self::Error> {
        Err(std::io::Error::other("inference forbidden in deterministic cases"))
    }
    fn complete(&self, _: &str, _: &Value) -> Result<Value, Self::Error> {
        Err(std::io::Error::other("inference forbidden in deterministic cases"))
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture: Value = serde_json::from_str(include_str!(
        "../testdata/upstream-golden/semantic-resolutions.json"
    ))?;
    let cases = fixture["cases"].as_array().ok_or("missing semantic cases")?;
    let mut results = Vec::with_capacity(cases.len());
    for case in cases {
        let request: SemanticInputRequest = serde_json::from_value(case["request"].clone())?;
        let aborted = case.get("aborted").and_then(Value::as_bool).unwrap_or(false);
        let result = resolve_semantic_input(&request, &UnavailableModel, None, aborted);
        results.push(json!(result));
    }
    println!("{}", serde_json::to_string(&results)?);
    Ok(())
}
