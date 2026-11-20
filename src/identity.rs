//! Stable, role-tagged identity references from GutTS auth/identityIds.ts.
//! Wire validators intentionally do not imply authorization.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const IDENTITY_KINDS: &[&str] = &[
    "application",
    "deployment",
    "oauth_client",
    "user",
    "workspace",
    "ui_session",
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityRef {
    pub kind: String,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityError {
    InvalidId,
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid identity kind or identifier")
    }
}
impl std::error::Error for IdentityError {}

pub fn is_identity_kind(value: &str) -> bool {
    IDENTITY_KINDS.contains(&value)
}

/// Matches /^[A-Za-z0-9][A-Za-z0-9._:-]{0,255}$/ with byte-exact ASCII semantics.
pub fn valid_identity_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    (1..=256).contains(&bytes.len())
        && bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        && bytes[1..]
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(*byte, b'.' | b'_' | b':' | b'-'))
}

pub fn parse_identity_id<'a>(kind: &str, value: &'a Value) -> Option<&'a str> {
    if !is_identity_kind(kind) {
        return None;
    }
    value.as_str().filter(|id| valid_identity_id(id))
}

pub fn require_identity_id<'a>(kind: &str, value: &'a Value) -> Result<&'a str, IdentityError> {
    parse_identity_id(kind, value).ok_or(IdentityError::InvalidId)
}

pub fn identity_ref(kind: &str, id: &str) -> Result<IdentityRef, IdentityError> {
    let parsed = require_identity_id(kind, &json!(id))?.to_owned();
    Ok(IdentityRef {
        kind: kind.to_owned(),
        id: parsed,
    })
}

pub fn parse_identity_ref(value: &Value) -> Option<IdentityRef> {
    let record = value.as_object()?;
    let kind = record.get("kind")?.as_str()?;
    let id = parse_identity_id(kind, record.get("id")?)?;
    Some(IdentityRef {
        kind: kind.to_owned(),
        id: id.to_owned(),
    })
}

/// Reject conflicting aliases instead of treating deployment URLs as application IDs.
pub fn parse_legacy_application_id(value: &Value) -> Option<String> {
    if value.is_string() {
        return parse_identity_id("application", value).map(str::to_owned);
    }
    let object = value.as_object()?;
    let mut found: Option<&str> = None;
    for key in ["app_id", "appId", "applicationId"] {
        if let Some(candidate) = object.get(key) {
            let id = parse_identity_id("application", candidate)?;
            if found.is_some_and(|previous| previous != id) {
                return None;
            }
            found = Some(id);
        }
    }
    found.map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pinned_identity_cases() {
        assert_eq!(
            parse_identity_id("application", &json!("aiko")),
            Some("aiko")
        );
        assert!(parse_identity_id("application", &json!("https://aiko.example")).is_none());
        assert!(parse_identity_id("application", &json!(" aiko")).is_none());
        assert!(parse_identity_id("application", &json!("aiko\n")).is_none());
        assert!(parse_identity_id("application", &json!("")).is_none());
        assert!(parse_identity_id("origin", &json!("aiko")).is_none());
        for kind in IDENTITY_KINDS {
            let reference = identity_ref(kind, "same-id").expect("valid fixture");
            assert_eq!(parse_identity_ref(&json!(reference)), Some(reference));
        }
        assert!(parse_identity_ref(&json!({"id":"aiko"})).is_none());
        assert!(parse_identity_ref(&json!({"kind":"origin","id":"aiko"})).is_none());
        assert_eq!(
            parse_legacy_application_id(&json!({"applicationId":"srkl"})),
            Some("srkl".into())
        );
        assert_eq!(
            parse_legacy_application_id(&json!({"app_id":"aiko","appId":"aiko"})),
            Some("aiko".into())
        );
        assert!(
            parse_legacy_application_id(&json!({"app_id":"aiko","applicationId":"kone"})).is_none()
        );
        assert!(parse_legacy_application_id(&json!({"appId":null})).is_none());
        assert!(parse_legacy_application_id(&json!({"origin":"https://aiko.example"})).is_none());
    }
    #[test]
    fn boundaries() {
        assert!(valid_identity_id(&format!("a{}", "x".repeat(255))));
        assert!(!valid_identity_id(&format!("a{}", "x".repeat(256))));
        for bad in ["-a", ".a", "é", "a/b", "a b", "a\u{feff}"] {
            assert!(!valid_identity_id(bad), "accepted {bad:?}");
        }
    }
}
