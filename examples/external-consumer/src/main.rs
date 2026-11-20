use gutrs::{arrays, auth_contracts, identity, json_utils, url_operations};
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
    assert!(auth_contracts::contract_app_id(&json!("kone")));
    assert!(!auth_contracts::contract_app_id(&json!("https://kone.example")));
    Ok(())
}
