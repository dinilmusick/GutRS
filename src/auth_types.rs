//! Typed Luth data contracts; network/credential handling is intentionally
//! separate from JSON shape models and does not imply Luth authentication.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentEnvironment { Production, Staging, Preview, Development }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuthDeploymentRecord {
    pub id: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<DeploymentEnvironment>,
    pub is_active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuthUser {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuthSession {
    pub id: String,
    pub user_id: String,
    pub expires_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuthSessionResult {
    pub authenticated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<LuthUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<LuthSession>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterApplicationResult {
    pub registered: bool,
    pub ecosystem_id: String,
    pub application_id: String,
    pub origin: String,
    pub environment: DeploymentEnvironment,
    pub authentication_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailSignInCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailSignUpData {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSignInOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LuthAuthError {
    pub code: String,
    pub message: String,
    pub status: Option<u16>,
}
impl std::fmt::Display for LuthAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}
impl std::error::Error for LuthAuthError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn session_round_trip_and_negative_authenticated_state() {
        let wire = json!({
            "authenticated":true,
            "user":{"id":"user-1","name":"Test","email":"person@example.com","emailVerified":true},
            "session":{"id":"session-1","userId":"user-1","expiresAt":"2030-01-01T00:00:00Z"}
        });
        let typed: LuthSessionResult = serde_json::from_value(wire.clone()).unwrap();
        assert!(typed.authenticated);
        assert_eq!(serde_json::to_value(typed).unwrap(), wire);
        let unauthenticated: LuthSessionResult = serde_json::from_value(json!({
            "authenticated":false,"error":"SESSION_INVALID"
        })).unwrap();
        assert!(!unauthenticated.authenticated);
        assert!(unauthenticated.session.is_none());
        let bad = json!({"authenticated":"yes"});
        assert!(serde_json::from_value::<LuthSessionResult>(bad).is_err());
    }
}
