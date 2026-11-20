//! Real public-path API output for independently pinned TypeScript differential.
use gutrs::{path_safety, trusted_api};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture: Value = serde_json::from_str(include_str!(
        "../testdata/upstream-golden/path-safety-cases.json"
    ))?;
    let cases = fixture["cases"].as_array().ok_or("missing path cases")?;
    let mut output = Vec::with_capacity(cases.len());
    for case in cases {
        let path = case["path"].as_str().ok_or("missing path")?;
        let result: Result<String, ()> = match case["kind"].as_str().ok_or("missing kind")? {
            "normalize" => path_safety::normalize_storage_path(path).map_err(|_| ()),
            "encode" => path_safety::encode_safe_path_segments(path).map_err(|_| ()),
            "resolve" => trusted_api::resolve_trusted_api_url(
                case["origin"].as_str().ok_or("missing origin")?,
                path,
            )
            .map(|url| url.to_string())
            .map_err(|_| ()),
            kind => return Err(format!("unrecognized operation {kind}").into()),
        };
        match result {
            Ok(value) => output.push(json!({"ok":true,"value":value})),
            Err(()) => output.push(json!({"ok":false})),
        }
    }
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}
