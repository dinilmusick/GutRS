//! Export real public Rust API results for exact cross-language comparison.
use gutrs::api::execute_gutts_api;
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture: Value = serde_json::from_str(include_str!(
        "../testdata/upstream-golden/api-utility-cases.json"
    ))?;
    let cases = fixture["cases"].as_array().ok_or("cases must be array")?;
    let mut results = Vec::new();
    for case in cases {
        let id = case["id"].as_str().ok_or("id missing")?;
        let result = match execute_gutts_api(id, &case["input"]) {
            Ok(actual) => json!({"id":id,"actual":actual}),
            Err(error) => json!({"id":id,"error":error.to_string()}),
        };
        results.push(result);
    }
    println!("{}", serde_json::to_string(&results)?);
    Ok(())
}
