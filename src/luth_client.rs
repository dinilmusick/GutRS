//! Hosted Luth authentication client with real transport injection, failover,
//! registration, bearer lifecycle and independently rechecked logout.
//! Pinned GutTS src/auth/luthAuthClient.ts. No SDK secrets or fake network IO.
use crate::auth_host::{resolve_client_environment, resolve_client_origin, DEFAULT_LUTH_URL};
use crate::auth_types::{
    DeploymentEnvironment, EmailSignInCredentials, EmailSignUpData, GoogleSignInOptions,
    LuthAuthError, LuthSessionResult, RegisterApplicationResult,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap, thread, time::Duration};

#[derive(Clone, Debug)]
pub struct LuthAuthClientConfig {
    pub ecosystem_id: String,
    pub application_id: String,
    pub luth_url: Option<String>,
    pub luth_urls: Option<Vec<String>>,
    pub origin: Option<String>,
    pub environment: Option<DeploymentEnvironment>,
    pub auto_register: bool,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub timeout_ms: u64,
}
impl LuthAuthClientConfig {
    pub fn new(ecosystem_id: &str, application_id: &str, origin: &str) -> Self {
        Self {
            ecosystem_id: ecosystem_id.into(),
            application_id: application_id.into(),
            luth_url: None,
            luth_urls: None,
            origin: Some(origin.into()),
            environment: None,
            auto_register: true,
            max_retries: 3,
            retry_delay_ms: 100,
            timeout_ms: 10000,
        }
    }
}
pub trait LuthAuthTokenStore {
    fn get(&self) -> String;
    fn set(&self, token: &str);
    fn clear(&self);
}
pub trait LuthAuthTransport {
    type Error: std::fmt::Display;
    /// Host must send a no-store request with credentials rules appropriate
    /// to its runtime and enforce deadline/timeout at the network boundary.
    fn request(
        &self,
        method: &str,
        url: &str,
        headers: &BTreeMap<String, String>,
        body: Option<&str>,
        timeout: Option<Duration>,
    ) -> Result<LuthTransportResponse, Self::Error>;
}
#[derive(Clone, Debug, PartialEq)]
pub struct LuthTransportResponse {
    pub status: u16,
    pub body: Value,
    pub headers: BTreeMap<String, String>,
}
impl LuthTransportResponse {
    pub fn ok(&self) -> bool {
        (200..300).contains(&self.status)
    }
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}
fn err(code: &str, message: &str, status: Option<u16>) -> LuthAuthError {
    LuthAuthError {
        code: code.into(),
        message: message.into(),
        status,
    }
}
pub struct LuthAuthClient<T: LuthAuthTransport, S: LuthAuthTokenStore> {
    pub ecosystem_id: String,
    pub application_id: String,
    pub origin: String,
    pub environment: DeploymentEnvironment,
    urls: Vec<String>,
    active_url_index: usize,
    next_url_index: usize,
    auto_register: bool,
    max_retries: u32,
    retry_delay_ms: u64,
    timeout_ms: u64,
    registered: Option<RegisterApplicationResult>,
    transport: T,
    token_store: S,
}
pub fn create_luth_auth_client<T: LuthAuthTransport, S: LuthAuthTokenStore>(
    config: LuthAuthClientConfig,
    transport: T,
    token_store: S,
) -> Result<LuthAuthClient<T, S>, LuthAuthError> {
    LuthAuthClient::new(config, transport, token_store)
}
impl<T: LuthAuthTransport, S: LuthAuthTokenStore> LuthAuthClient<T, S> {
    pub fn new(
        config: LuthAuthClientConfig,
        transport: T,
        token_store: S,
    ) -> Result<Self, LuthAuthError> {
        let origin = resolve_client_origin(config.origin.as_deref(), "");
        if origin.is_empty() {
            return Err(err("APPLICATION_ORIGIN_UNAVAILABLE",
                "Application deployment origin is unavailable. Supply an explicit origin; Node callers can use resolveServerRuntimeAppUrl().",None));
        }
        let initial = config
            .luth_urls
            .filter(|urls| !urls.is_empty())
            .unwrap_or_else(|| vec![config.luth_url.unwrap_or_else(|| DEFAULT_LUTH_URL.into())]);
        let mut urls = Vec::<String>::new();
        for value in initial {
            let normalized = value.trim_end_matches('/').to_owned();
            if !urls.contains(&normalized) {
                urls.push(normalized);
            }
        }
        if urls.is_empty() || urls.iter().any(|url| url::Url::parse(url).is_err()) {
            return Err(err(
                "LUTH_DEPLOYMENT_UNAVAILABLE",
                "No valid Luth deployment URL configured.",
                None,
            ));
        }
        let environment = config
            .environment
            .unwrap_or_else(|| resolve_client_environment(&origin));
        Ok(Self {
            ecosystem_id: config.ecosystem_id,
            application_id: config.application_id,
            origin,
            environment,
            urls,
            active_url_index: 0,
            next_url_index: 0,
            auto_register: config.auto_register,
            max_retries: config.max_retries,
            retry_delay_ms: config.retry_delay_ms,
            timeout_ms: config.timeout_ms,
            registered: None,
            transport,
            token_store,
        })
    }
    pub fn luth_url(&self) -> &str {
        self.urls
            .get(self.active_url_index)
            .map(String::as_str)
            .unwrap_or(DEFAULT_LUTH_URL)
    }
    pub fn luth_urls(&self) -> Vec<String> {
        self.urls.clone()
    }
    pub fn get_auth_headers(&self) -> BTreeMap<String, String> {
        let token = self.token_store.get();
        let mut result = BTreeMap::new();
        if !token.is_empty() {
            result.insert("Authorization".into(), format!("Bearer {token}"));
        }
        result
    }
    pub fn clear_auth_token(&self) {
        self.token_store.clear();
    }
    /// Merge a fresh host-verified deployment URL list without dropping
    /// existing failover endpoints. Discovery is controlled by the caller.
    pub fn update_urls(&mut self, discovered: &[String]) {
        let mut list = Vec::new();
        for url in discovered.iter().chain(self.urls.iter()) {
            let trimmed = url.trim_end_matches('/').to_owned();
            if !trimmed.is_empty() && !list.contains(&trimmed) && url::Url::parse(&trimmed).is_ok()
            {
                list.push(trimmed);
            }
        }
        if !list.is_empty() {
            self.urls = list;
            self.active_url_index = 0;
            self.next_url_index = 0;
        }
    }
    fn request(
        &mut self,
        path: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<LuthTransportResponse, LuthAuthError> {
        if !path.starts_with('/')
            || path.starts_with("//")
            || path.contains('\\')
            || path.contains("..")
        {
            return Err(err(
                "INVALID_LUTH_REQUEST_PATH",
                "Luth request path must be local.",
                None,
            ));
        }
        let pool = self.urls.len();
        if pool == 0 {
            return Err(err(
                "LUTH_DEPLOYMENT_UNAVAILABLE",
                "No Luth deployments available.",
                None,
            ));
        }
        let start = if pool > 1 {
            self.next_url_index % pool
        } else {
            0
        };
        let headers = self.get_auth_headers();
        let mut last = err(
            "LUTH_DEPLOYMENT_UNAVAILABLE",
            "All Luth deployments failed.",
            None,
        );
        for cycle in 0..=self.max_retries {
            for offset in 0..pool {
                let idx = (start + offset) % pool;
                let url = format!("{}{}", self.urls[idx], path);
                let timeout = (self.timeout_ms > 0).then(|| Duration::from_millis(self.timeout_ms));
                let response = self
                    .transport
                    .request(method, &url, &headers, body, timeout);
                match response {
                    Ok(response) if matches!(response.status, 502..=504) => {
                        last = err(
                            "LUTH_DEPLOYMENT_UNAVAILABLE",
                            &format!("Luth deployment unavailable ({})", response.status),
                            Some(response.status),
                        );
                    }
                    Ok(response) => {
                        self.active_url_index = idx;
                        self.next_url_index = if offset == 0 { (idx + 1) % pool } else { idx };
                        return Ok(response);
                    }
                    Err(error) => {
                        last = err("LUTH_DEPLOYMENT_UNAVAILABLE", &error.to_string(), None);
                    }
                }
            }
            if cycle < self.max_retries {
                let delay = self
                    .retry_delay_ms
                    .saturating_mul(2_u64.saturating_pow(cycle));
                thread::sleep(Duration::from_millis(delay));
            }
        }
        Err(last)
    }
    fn post_json(
        &mut self,
        path: &str,
        body: &Value,
    ) -> Result<LuthTransportResponse, LuthAuthError> {
        self.request(path, "POST", Some(&body.to_string()))
    }
    pub fn register(&mut self) -> Result<RegisterApplicationResult, LuthAuthError> {
        let body = json!({"ecosystemId":self.ecosystem_id,"applicationId":self.application_id,
            "origin":self.origin,"environment":self.environment});
        let mut last = err(
            "REGISTRATION_FAILED",
            "Application registration timed out",
            None,
        );
        for attempt in 0..=self.max_retries {
            match self.post_json("/api/registry/register", &body) {
                Ok(response) if response.ok() => {
                    return serde_json::from_value(response.body).map_err(|e| {
                        err("REGISTRATION_FAILED", &e.to_string(), Some(response.status))
                    });
                }
                Ok(response) => {
                    let code = response.body.get("code").and_then(Value::as_str).unwrap_or(
                        if response.status == 404 {
                            "APPLICATION_NOT_REGISTERED"
                        } else {
                            "REGISTRATION_FAILED"
                        },
                    );
                    let message = response
                        .body
                        .get("message")
                        .and_then(Value::as_str)
                        .unwrap_or("Application registration failed");
                    last = err(code, message, Some(response.status));
                    if response.status < 500 {
                        return Err(last);
                    }
                }
                Err(error) => {
                    if error.status.is_some_and(|s| s < 500) {
                        return Err(error);
                    }
                    last = error;
                }
            }
            if attempt < self.max_retries {
                let delay = self
                    .retry_delay_ms
                    .saturating_mul(2_u64.saturating_pow(attempt));
                thread::sleep(Duration::from_millis(delay));
            }
        }
        Err(err("REGISTRATION_FAILED", &last.to_string(), None))
    }
    pub fn ensure_registered(
        &mut self,
    ) -> Result<Option<RegisterApplicationResult>, LuthAuthError> {
        if !self.auto_register {
            return Ok(None);
        }
        if self.registered.is_none() {
            self.registered = Some(self.register()?);
        }
        Ok(self.registered.clone())
    }
    pub fn get_session(&mut self) -> Result<LuthSessionResult, LuthAuthError> {
        self.ensure_registered()?;
        let response = self.request("/api/session", "GET", None)?;
        if let Some(token) = response.header("set-auth-token") {
            if !token.is_empty() {
                self.token_store.set(token);
            }
        }
        if response.status == 401 {
            self.clear_auth_token();
            return Ok(LuthSessionResult {
                authenticated: false,
                user: None,
                session: None,
                error: Some("SESSION_INVALID".into()),
            });
        }
        if !response.ok() {
            return Err(err(
                "AUTHENTICATION_FAILED",
                &format!("Session fetch failed ({})", response.status),
                Some(response.status),
            ));
        }
        let result: LuthSessionResult = serde_json::from_value(response.body).map_err(|e| {
            err(
                "AUTHENTICATION_FAILED",
                &e.to_string(),
                Some(response.status),
            )
        })?;
        if let Some(token) = result.session.as_ref().and_then(|s| s.token.as_ref()) {
            self.token_store.set(token);
        }
        Ok(result)
    }
    pub fn refresh_session(&mut self) -> Result<LuthSessionResult, LuthAuthError> {
        self.get_session()
    }
    fn post_auth(
        &mut self,
        path: &str,
        body: &Value,
        fallback: &str,
    ) -> Result<Value, LuthAuthError> {
        let response = self.post_json(path, body)?;
        let token = response
            .header("set-auth-token")
            .map(str::to_owned)
            .or_else(|| {
                response
                    .body
                    .get("token")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .or_else(|| {
                response
                    .body
                    .get("session")
                    .and_then(|s| s.get("token"))
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            });
        if let Some(token) = token.filter(|token| !token.is_empty()) {
            self.token_store.set(&token);
        }
        if !response.ok() {
            let code = response
                .body
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or("AUTHENTICATION_FAILED");
            let message = response
                .body
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or(fallback);
            return Err(err(code, message, Some(response.status)));
        }
        Ok(response.body)
    }
    pub fn sign_in_with_email(
        &mut self,
        credentials: &EmailSignInCredentials,
    ) -> Result<Value, LuthAuthError> {
        self.ensure_registered()?;
        self.post_auth(
            "/api/auth/sign-in/email",
            &json!(credentials),
            "Sign in failed",
        )
    }
    pub fn sign_up_with_email(&mut self, data: &EmailSignUpData) -> Result<Value, LuthAuthError> {
        self.ensure_registered()?;
        self.post_auth("/api/auth/sign-up/email", &json!(data), "Sign up failed")
    }
    pub fn sign_in_with_google(
        &mut self,
        options: Option<&GoogleSignInOptions>,
    ) -> Result<Value, LuthAuthError> {
        self.ensure_registered()?;
        let callback = options
            .and_then(|o| o.callback_url.as_deref())
            .filter(|url| !url.is_empty())
            .unwrap_or(&self.origin);
        self.post_auth(
            "/api/auth/sign-in/social",
            &json!({"provider":"google","callbackURL":callback}),
            "Google sign-in failed",
        )
    }
    pub fn sign_out(&mut self) -> Result<(), LuthAuthError> {
        let response = self.post_json("/api/auth/sign-out", &json!({}))?;
        if !response.ok() {
            let code = response
                .body
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or("AUTHENTICATION_FAILED");
            let message = response
                .body
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("Sign out failed");
            return Err(err(code, message, Some(response.status)));
        }
        self.clear_auth_token();
        if self.get_session()?.authenticated {
            return Err(err(
                "SESSION_INVALIDATION_FAILED",
                "Luth session is still active after sign-out.",
                None,
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::VecDeque, sync::Mutex};
    #[derive(Default)]
    struct Token(Mutex<String>);
    impl LuthAuthTokenStore for Token {
        fn get(&self) -> String {
            self.0.lock().unwrap().clone()
        }
        fn set(&self, value: &str) {
            *self.0.lock().unwrap() = value.into();
        }
        fn clear(&self) {
            self.0.lock().unwrap().clear();
        }
    }
    struct Transport {
        responses: Mutex<VecDeque<LuthTransportResponse>>,
        urls: Mutex<Vec<String>>,
    }
    impl LuthAuthTransport for Transport {
        type Error = std::io::Error;
        fn request(
            &self,
            _: &str,
            url: &str,
            _: &BTreeMap<String, String>,
            _: Option<&str>,
            _: Option<Duration>,
        ) -> Result<LuthTransportResponse, Self::Error> {
            self.urls.lock().unwrap().push(url.into());
            self.responses
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| std::io::Error::other("fixture exhausted"))
        }
    }
    fn response(status: u16, body: Value, token: Option<&str>) -> LuthTransportResponse {
        let mut headers = BTreeMap::new();
        if let Some(token) = token {
            headers.insert("Set-Auth-Token".into(), token.into());
        }
        LuthTransportResponse {
            status,
            body,
            headers,
        }
    }
    fn transport(responses: Vec<LuthTransportResponse>) -> Transport {
        Transport {
            responses: Mutex::new(responses.into()),
            urls: Mutex::new(vec![]),
        }
    }
    fn config() -> LuthAuthClientConfig {
        let mut cfg = LuthAuthClientConfig::new("ultrst", "aiko", "https://aiko.example");
        cfg.luth_urls = Some(vec![
            "https://luth-a.example".into(),
            "https://luth-b.example/".into(),
        ]);
        cfg.auto_register = false;
        cfg.max_retries = 0;
        cfg
    }
    #[test]
    fn failover_rotation_and_verifiable_session_refresh() {
        let body = json!({"authenticated":true,
            "user":{"id":"u1","name":"User","email":"user@example.com"},
            "session":{"id":"s1","userId":"u1","expiresAt":"2030-01-01","token":"session-token"}});
        let mut client = create_luth_auth_client(
            config(),
            transport(vec![
                response(503, json!({}), None),
                response(200, body, Some("header-token")),
                response(200, json!({}), None),
                response(401, json!({}), None),
            ]),
            Token::default(),
        )
        .unwrap();
        let session = client.get_session().unwrap();
        assert!(session.authenticated);
        assert_eq!(client.luth_url(), "https://luth-b.example");
        assert_eq!(
            client.get_auth_headers().get("Authorization").unwrap(),
            "Bearer session-token"
        );
        assert!(client.sign_out().is_ok());
        assert!(client.get_auth_headers().is_empty());
    }
    #[test]
    fn registration_and_auth_failure_fail_closed() {
        let reg = json!({"registered":true,"ecosystemId":"ultrst","applicationId":"aiko",
            "origin":"https://aiko.example","environment":"production","authenticationEnabled":true});
        let mut cfg = config();
        cfg.auto_register = true;
        let mut client = create_luth_auth_client(
            cfg,
            transport(vec![
                response(200, reg, None),
                response(403, json!({"code":"DENIED","message":"blocked"}), None),
            ]),
            Token::default(),
        )
        .unwrap();
        assert_eq!(
            client
                .sign_in_with_email(&EmailSignInCredentials {
                    email: "user@example.com".into(),
                    password: "invalid".into()
                })
                .unwrap_err()
                .code,
            "DENIED"
        );
    }
    #[test]
    fn bad_origin_and_unauthorized_response_do_not_grant_session() {
        let mut bad = config();
        bad.origin = None;
        let r = create_luth_auth_client(bad, transport(vec![]), Token::default());
        assert!(r.is_err());
        let mut client = create_luth_auth_client(
            config(),
            transport(vec![response(401, json!({}), None)]),
            Token::default(),
        )
        .unwrap();
        assert!(!client.get_session().unwrap().authenticated);
        assert!(client.get_auth_headers().is_empty());
    }
}
