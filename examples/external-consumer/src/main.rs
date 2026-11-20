use gutrs::{
    api, arrays, auth_contracts, auth_wires, identity, json_utils,
    objects, semantic_input, url_operations,
};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reference = identity::identity_ref("application", "aiko")?;
    assert_eq!(reference.id, "aiko");
    let document = json_utils::parse_json(r#"{"enabled":true}"#)?;
    assert_eq!(document["enabled"], true);
    let numbers = arrays::chunk(&[1, 2, 3], 2.0)
        .map_err(|_| std::io::Error::other("unexpected chunk rejection"))?;
    assert_eq!(numbers, vec![vec![1, 2], vec![3]]);
    assert_eq!(
        url_operations::get_query_param("https://example.com/?q=hello+world", "q")
            .map_err(|_| std::io::Error::other("invalid URL"))?,
        Some("hello world".to_owned())
    );
    let app = auth_wires::ApplicationId::parse("aiko")
        .ok_or_else(|| std::io::Error::other("invalid application ID"))?;
    assert_eq!(app.as_str(), reference.id);
    assert_eq!(
        objects::pick_property_paths(&json!({"a":{"b":1,"c":2}}), &["a.b"]),
        Some(json!({"a":{"b":1}}))
    );
    let resolution = semantic_input::resolve_deterministic_semantic_input(
        "42", &json!({"type":"integer"})
    ).ok_or_else(|| std::io::Error::other("failed deterministic resolution"))?;
    assert_eq!(resolution.value, Some(json!(42)));
    assert_eq!(
        api::execute_gutts_api("gutts.strings.camelCase", &json!({"value":"hello world"}))?,
        json!("helloWorld")
    );
    assert!(auth_contracts::contract_app_id(&json!("kone")));
    assert!(!auth_contracts::contract_app_id(&json!("https://kone.example")));
    Ok(())
}
