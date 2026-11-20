//! Machine-readable public library manifests for independent cross-language
//! verification against actual pinned GutTS exports.
use gutrs::{api, models};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let export = json!({
        "models":models::gutts_model_definitions(),
        "apis":api::gutts_api_definitions()?,
        "fod":api::gutts_fod_node_definitions()?
    });
    println!("{}", serde_json::to_string(&export)?);
    Ok(())
}
