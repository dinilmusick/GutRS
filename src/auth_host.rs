//! Hosted auth URL and application backend-for-frontend client primitives.
//! These implement the pinned GutTS auth/Luth client pure functions and
//! applicationAuthClient.ts, using an explicit host HTTP transport.
//! OAuth codes, verifiers, refresh credentials and Luth cookies MUST remain
//! on the application backend; this API does not expose or store bearer tokens.
use crate::auth_types::{DeploymentEnvironment, LuthAuthError, LuthSessionResult};
use serde_json::Value;
use std::collections::BTreeMap;
use url::Url;

pub const DEFAULT_LUTH_URL: &str = "https://luth.blueprint-ninjastudios.workers.dev";

pub fn normalize_origin(value: &str) -> String {
    Url::parse(value.trim())
        .map_or_else(|_| String::new(), |url| url.origin().ascii_serialization())
}

/// Runtime origin is an explicit adapter argument in native Rust instead of
/// browser globalThis.location. An explicit origin has precedence when nonblank.
pub fn resolve_client_origin(explicit_origin: Option<&str>, runtime_origin: &str) -> String {
    let explicit = explicit_origin.map(str::trim).filter(|v| !v.is_empty());
    normalize_origin(explicit.unwrap_or(runtime_origin))
}
pub fn resolve_runtime_app_url(runtime_origin: Option<&str>) -> String {
    runtime_origin.unwrap_or_default().trim().to_owned()
}

pub fn resolve_client_environment(origin: &str) -> DeploymentEnvironment {
    let Ok(url) = Url::parse(origin) else {
        return DeploymentEnvironment::Production;
    };
    let host = url.host_str().unwrap_or("").to_lowercase();
    if matches!(host.as_str(), "localhost" | "127.0.0.1" | "::1" | "[::1]") {
        DeploymentEnvironment::Development
    } else if host.contains("staging") {
        DeploymentEnvironment::Staging
    } else if host.contains("preview") || host.contains("-git-") || host.ends_with(".workers.dev") {
        DeploymentEnvironment::Preview
    } else {
        DeploymentEnvironment::Production
    }
}
pub fn resolve_server_runtime_app_url() -> String {
    std::env::var("APP_URL")
        .unwrap_or_default()
        .trim()
        .to_owned()
}

/// Portable phase of GutTS hydrateLuthTokenFromUrl: returns the token and
/// URL to install via the native host's secure store and history adapter.
/// DOES NOT persist the token or navigate a browser by itself.
pub fn extract_luth_token_and_clean_url(
    href: &str,
) -> Result<Option<(String, String)>, LuthAuthError> {
    let mut url = Url::parse(href).map_err(|_| {
        auth_error(
            "INVALID_AUTH_CALLBACK_URL",
            "Invalid authentication callback URL.",
            None,
        )
    })?;
    let token = url
        .query_pairs()
        .find(|(key, _)| key == "token")
        .map(|(_, value)| value.trim().to_owned())
        .filter(|value| !value.is_empty());
    let Some(token) = token else {
        return Ok(None);
    };
    let remaining: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(key, _)| key != "token")
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    url.query_pairs_mut()
        .clear()
        .extend_pairs(remaining.iter().map(|(k, v)| (&**k, &**v)));
    let location = format!(
        "{}{}{}",
        url.path(),
        url.query().map(|v| format!("?{v}")).unwrap_or_default(),
        url.fragment().map(|v| format!("#{v}")).unwrap_or_default()
    );
    Ok(Some((token, location)))
}
fn auth_error(code: &str, message: &str, status: Option<u16>) -> LuthAuthError {
    LuthAuthError {
        code: code.into(),
        message: message.into(),
        status,
    }
}

/// Host HTTP request. Only local, application-owned authentication paths are
/// passed to this transport. The host enforces same-origin and no-store.
pub trait ApplicationAuthTransport {
    type Error: std::fmt::Display;
    fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<AuthResponse, Self::Error>;
}
#[derive(Debug, Clone, PartialEq)]
pub struct AuthResponse {
    pub status: u16,
    pub body: Value,
}
impl AuthResponse {
    pub fn ok(&self) -> bool {
        (200..300).contains(&self.status)
    }
}
#[derive(Clone, Debug, Default)]
pub struct ApplicationAuthClientConfig {
    pub base_path: Option<String>,
}

/// A native counterpart to browser-side ApplicationAuthClient. Start-sign-in
/// returns a local navigation destination for the caller's host adapter.
pub struct ApplicationAuthClient<T: ApplicationAuthTransport> {
    base_path: String,
    transport: T,
}
pub fn create_application_auth_client<T: ApplicationAuthTransport>(
    config: ApplicationAuthClientConfig,
    transport: T,
) -> Result<ApplicationAuthClient<T>, LuthAuthError> {
    ApplicationAuthClient::new(config, transport)
}
impl<T: ApplicationAuthTransport> ApplicationAuthClient<T> {
    pub fn new(config: ApplicationAuthClientConfig, transport: T) -> Result<Self, LuthAuthError> {
        let path = config
            .base_path
            .filter(|p| !p.is_empty())
            .unwrap_or_else(|| "/api/auth/oidc".into());
        let valid = path.starts_with('/')
            && !path.contains("//")
            && path
                .as_bytes()
                .iter()
                .skip(1)
                .all(|b| b.is_ascii_alphanumeric() || matches!(*b, b'/' | b'_' | b'-'))
            && path.len() > 1;
        if !valid {
            return Err(auth_error(
                "INVALID_APPLICATION_AUTH_PATH",
                "App authentication must use a fixed local path.",
                None,
            ));
        }
        Ok(Self {
            base_path: path.trim_end_matches('/').into(),
            transport,
        })
    }
    pub fn get_auth_headers(&self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }
    /// Return an HTTPS-safe local redirect path. No external return URL is accepted.
    pub fn start_sign_in(&self, return_to: Option<&str>) -> Result<String, LuthAuthError> {
        let next = return_to.filter(|s| !s.is_empty()).unwrap_or("/");
        if !next.starts_with('/')
            || next.starts_with("//")
            || next.contains('\\')
            || next.bytes().any(|b| b < 32 || b == 127)
        {
            return Err(auth_error(
                "INVALID_AUTH_RETURN_PATH",
                "Login return path must be local.",
                None,
            ));
        }
        let encoded = encode_uri_component(next);
        Ok(format!("{}/start?next={encoded}", self.base_path))
    }
    pub fn sign_in_with_google(&self, return_to: Option<&str>) -> Result<String, LuthAuthError> {
        self.start_sign_in(return_to)
    }
    pub fn sign_in_with_email(&self, return_to: Option<&str>) -> Result<String, LuthAuthError> {
        self.start_sign_in(return_to)
    }
    pub fn get_session(&self) -> Result<LuthSessionResult, LuthAuthError> {
        let response = self
            .transport
            .request("GET", &format!("{}/session", self.base_path), None)
            .map_err(|_| {
                auth_error(
                    "AUTH_SESSION_UNAVAILABLE",
                    "Application authentication service is unavailable.",
                    Some(503),
                )
            })?;
        if response.status == 401 {
            return Ok(LuthSessionResult {
                authenticated: false,
                user: None,
                session: None,
                error: Some("SESSION_INVALID".into()),
            });
        }
        if !response.ok() {
            return Err(auth_error(
                "AUTH_SESSION_UNAVAILABLE",
                "Application authentication service is unavailable.",
                Some(response.status),
            ));
        }
        let result: LuthSessionResult = serde_json::from_value(response.body).map_err(|_| {
            auth_error(
                "AUTH_SESSION_INVALID",
                "Application returned an invalid authenticated session.",
                None,
            )
        })?;
        if !result.authenticated
            || !result
                .user
                .as_ref()
                .is_some_and(|u| !u.id.is_empty() && !u.email.is_empty())
            || !result.session.as_ref().is_some_and(|s| !s.id.is_empty())
        {
            return Err(auth_error(
                "AUTH_SESSION_INVALID",
                "Application returned an invalid authenticated session.",
                None,
            ));
        }
        Ok(result)
    }
    pub fn sign_out(&self) -> Result<(), LuthAuthError> {
        let response = self
            .transport
            .request("POST", &format!("{}/logout", self.base_path), Some("{}"))
            .map_err(|_| {
                auth_error(
                    "AUTH_LOGOUT_UNAVAILABLE",
                    "Logout service is unavailable.",
                    Some(503),
                )
            })?;
        if !response.ok() {
            return Err(auth_error(
                "AUTH_LOGOUT_REJECTED",
                "Logout was rejected by the application.",
                Some(response.status),
            ));
        }
        if self.get_session()?.authenticated {
            return Err(auth_error(
                "AUTH_LOGOUT_UNVERIFIED",
                "Application session remains valid after logout.",
                None,
            ));
        }
        Ok(())
    }
}
fn encode_uri_component(value: &str) -> String {
    use std::fmt::Write;
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
            )
        {
            encoded.push(byte as char);
        } else {
            write!(&mut encoded, "%{byte:02X}").expect("formatting String cannot fail");
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::VecDeque;
    use std::sync::Mutex;
    struct TestTransport {
        responses: Mutex<VecDeque<AuthResponse>>,
        paths: Mutex<Vec<String>>,
    }
    impl ApplicationAuthTransport for TestTransport {
        type Error = std::io::Error;
        fn request(
            &self,
            method: &str,
            path: &str,
            _: Option<&str>,
        ) -> Result<AuthResponse, Self::Error> {
            self.paths.lock().unwrap().push(format!("{method} {path}"));
            self.responses
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| std::io::Error::other("fixture exhausted"))
        }
    }
    fn transport(responses: Vec<AuthResponse>) -> TestTransport {
        TestTransport {
            responses: Mutex::new(responses.into()),
            paths: Mutex::new(vec![]),
        }
    }
    fn response(status: u16, body: Value) -> AuthResponse {
        AuthResponse { status, body }
    }
    #[test]
    fn trusted_local_redirects_and_environment() {
        let client = create_application_auth_client(
            ApplicationAuthClientConfig::default(),
            transport(vec![]),
        )
        .unwrap();
        assert_eq!(
            client.start_sign_in(Some("/dash?q=one two")).unwrap(),
            "/api/auth/oidc/start?next=%2Fdash%3Fq%3Done%20two"
        );
        assert_eq!(
            client.sign_in_with_google(None).unwrap(),
            "/api/auth/oidc/start?next=%2F"
        );
        assert!(client.sign_in_with_email(Some("//evil.com")).is_err());
        assert!(client.get_auth_headers().is_empty());
        assert_eq!(
            resolve_client_origin(Some("https://example.com/a?x=1"), ""),
            "https://example.com"
        );
        assert_eq!(
            resolve_client_environment("https://some.workers.dev"),
            DeploymentEnvironment::Preview
        );
        assert_eq!(
            resolve_client_environment("http://localhost:3000"),
            DeploymentEnvironment::Development
        );
        assert_eq!(
            resolve_client_environment("https://staging.example"),
            DeploymentEnvironment::Staging
        );
        assert!(create_application_auth_client(
            ApplicationAuthClientConfig {
                base_path: Some("//outside".into())
            },
            transport(vec![])
        )
        .is_err());
    }
    #[test]
    fn fail_closed_session_logout_and_verification() {
        let session = json!({"authenticated":true,
            "user":{"id":"person","name":"Test","email":"x@example.com"},
            "session":{"id":"s1","userId":"person","expiresAt":"2030-01-01"}});
        let client = create_application_auth_client(
            ApplicationAuthClientConfig::default(),
            transport(vec![
                response(200, session),
                response(204, json!({})),
                response(401, json!({})),
            ]),
        )
        .unwrap();
        assert!(client.get_session().unwrap().authenticated);
        assert!(client.sign_out().is_ok());
        let bad = create_application_auth_client(
            ApplicationAuthClientConfig::default(),
            transport(vec![response(200, json!({"authenticated":false}))]),
        )
        .unwrap();
        assert_eq!(bad.get_session().unwrap_err().code, "AUTH_SESSION_INVALID");
        let still_logged_in = create_application_auth_client(
            ApplicationAuthClientConfig::default(),
            transport(vec![
                response(200, json!({})),
                response(
                    200,
                    json!({"authenticated":true,
                "user":{"id":"person","name":"Test","email":"x@example.com"},
                "session":{"id":"s1","userId":"person","expiresAt":"2030-01-01"}}),
                ),
            ]),
        )
        .unwrap();
        assert_eq!(
            still_logged_in.sign_out().unwrap_err().code,
            "AUTH_LOGOUT_UNVERIFIED"
        );
    }
    #[test]
    fn callback_token_requires_url_scrub() {
        let (token, path) = extract_luth_token_and_clean_url(
            "https://example.com/callback?token=secret&next=%2Fapp#status",
        )
        .unwrap()
        .unwrap();
        assert_eq!(token, "secret");
        assert_eq!(path, "/callback?next=%2Fapp#status");
        assert_eq!(
            extract_luth_token_and_clean_url("https://example.com/?x=1").unwrap(),
            None
        );
    }
}
