//! Deterministic, fail-closed JSON wire-contract validators.
//!
//! Upstream: GutTS@3e2cbc0990ba29830ce91c5d1a1d552675d5334c
//! auth/oauthClientContract.ts, capabilityContract.ts, invocationContract.ts.
//! These are SHAPE validators only. They never authenticate principals,
//! establish workspace membership, or authorize an invocation.
use crate::identity::valid_identity_id;
use serde_json::{json, Map, Value};
use std::collections::HashSet;
use url::Url;

pub const ULTRST_APP_CONTRACT_VERSION: u64 = 1;
const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractError(pub &'static str);
impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ContractError {}
pub type ContractCheck<T> = Result<T, ContractError>;

pub fn contract_record(value: &Value) -> Option<&Map<String, Value>> {
    value.as_object()
}

pub fn contract_id(value: &Value) -> bool {
    value.as_str().is_some_and(|s| {
        !s.is_empty()
            && s.len() <= 256
            && s.chars().all(|c| {
                c != '\\' && c != '\u{feff}' && !c.is_whitespace()
            })
    })
}

/// Application ID is a stable slug, not a URI scheme or deployment URL.
pub fn contract_app_id(value: &Value) -> bool {
    let Some(s) = value.as_str() else { return false; };
    if !valid_identity_id(s) { return false; }
    if let Some((prefix, _)) = s.split_once(':') {
        if prefix.starts_with(|c: char| c.is_ascii_alphabetic())
            && prefix.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
        {
            return false;
        }
    }
    true
}

pub fn contract_exact_keys(value: &Map<String, Value>, allowed: &[&str]) -> bool {
    value.keys().all(|key| allowed.contains(&key.as_str()))
}

fn exact_fields(value: &Value, allowed: &[&str]) -> bool {
    value.as_object().is_some_and(|o| contract_exact_keys(o, allowed))
}

fn js_trimmed(value: &str) -> bool {
    value.trim_matches(|c: char| c.is_whitespace() || c == '\u{feff}') == value
}

fn url_has_nonempty_query(url: &Url) -> bool {
    url.query().is_some_and(|value| !value.is_empty())
}

/// Strict canonical callback validator. Distinct from proving domain ownership.
pub fn validate_oauth_redirect_uri(
    candidate: &Value,
    platform: &str,
    verified_hosts: &[String],
) -> ContractCheck<String> {
    let text = candidate
        .as_str()
        .filter(|s| s.len() <= 2048 && js_trimmed(s) && !s.contains('*') && !s.contains('#'))
        .ok_or(ContractError("INVALID_REDIRECT_URI"))?;
    let parsed = Url::parse(text).map_err(|_| ContractError("INVALID_REDIRECT_URI"))?;
    let canonical = parsed.to_string();
    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.fragment().is_some_and(|f| !f.is_empty())
        || url_has_nonempty_query(&parsed)
        || parsed.path().is_empty()
        || canonical != text
    {
        return Err(ContractError("NON_CANONICAL_REDIRECT_URI"));
    }
    if platform == "desktop" {
        if parsed.scheme() != "http"
            || !matches!(parsed.host_str(), Some("127.0.0.1" | "::1" | "[::1]"))
            || parsed.port().is_none()
            || parsed.path() == "/"
        {
            return Err(ContractError("DESKTOP_REQUIRES_EXACT_LOOPBACK_CALLBACK"));
        }
    } else {
        let host = parsed.host_str().unwrap_or("");
        if parsed.scheme() != "https"
            || host.is_empty()
            || host == "localhost"
            || host.ends_with(".localhost")
            || host.contains(':')
        {
            return Err(ContractError("WEB_OR_MOBILE_REQUIRES_HTTPS"));
        }
        if matches!(platform, "ios" | "android") && !verified_hosts.iter().any(|h| h == host) {
            return Err(ContractError("MOBILE_HOST_NOT_VERIFIED"));
        }
    }
    Ok(text.to_owned())
}

pub fn validate_oauth_client_registration(input: &Value) -> ContractCheck<Value> {
    let allowed = [
        "schemaVersion", "appId", "clientId", "platform", "publicClient", "pkceMethod",
        "redirectUris", "verifiedHosts",
    ];
    let o = input.as_object().filter(|o| contract_exact_keys(o, &allowed))
        .ok_or(ContractError("INVALID_CLIENT_FIELDS"))?;
    let platform = o.get("platform").and_then(Value::as_str).unwrap_or("");
    if o.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || !o.get("appId").is_some_and(contract_app_id)
        || !o.get("clientId").is_some_and(|v| v.as_str().is_some_and(valid_identity_id))
        || !matches!(platform, "web" | "ios" | "android" | "desktop")
        || o.get("publicClient") != Some(&Value::Bool(true))
        || o.get("pkceMethod").and_then(Value::as_str) != Some("S256")
    {
        return Err(ContractError("INVALID_CLIENT_IDENTITY_OR_PKCE"));
    }
    let hosts = match o.get("verifiedHosts") {
        None => Vec::new(),
        Some(Value::Array(values)) => {
            let mut result = Vec::new();
            for item in values {
                let Some(host) = item.as_str() else {
                    return Err(ContractError("INVALID_VERIFIED_HOSTS"));
                };
                if host.is_empty()
                    || host != host.to_ascii_lowercase()
                    || !host.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
                    || host.contains("..")
                    || host.starts_with('.')
                {
                    return Err(ContractError("INVALID_VERIFIED_HOSTS"));
                }
                result.push(host.to_owned());
            }
            result
        }
        _ => return Err(ContractError("INVALID_VERIFIED_HOSTS")),
    };
    let uris = o.get("redirectUris").and_then(Value::as_array)
        .filter(|a| (1..=32).contains(&a.len()))
        .ok_or(ContractError("INVALID_REDIRECT_SET"))?;
    let mut seen = HashSet::new();
    for uri in uris {
        if !seen.insert(uri.to_string()) {
            return Err(ContractError("INVALID_REDIRECT_SET"));
        }
    }
    for uri in uris {
        validate_oauth_redirect_uri(uri, platform, &hosts)?;
    }
    Ok(input.clone())
}

const CAP_BASE: &[&str] = &[
    "schemaVersion", "appId", "operationId", "revision", "displayName",
    "requiredScopes", "inputParameters", "outputParameters", "kind",
];

fn valid_scopes(value: Option<&Value>) -> bool {
    let Some(values) = value.and_then(Value::as_array) else { return false; };
    if values.len() > 64 { return false; }
    let mut seen = HashSet::new();
    values.iter().all(|v| contract_id(v) && seen.insert(v.as_str().unwrap_or("")))
}
fn valid_parameters(value: Option<&Value>) -> bool {
    let Some(values) = value.and_then(Value::as_array) else { return false; };
    if values.len() > 128 { return false; }
    let mut seen = HashSet::new();
    values.iter().all(|v| {
        let Some(p) = v.as_object() else { return false; };
        contract_exact_keys(p, &["name", "kind", "required"])
            && p.get("name").is_some_and(contract_id)
            && matches!(p.get("kind").and_then(Value::as_str),
                Some("string" | "number" | "integer" | "boolean" | "object" | "array"))
            && p.get("required").is_some_and(Value::is_boolean)
            && seen.insert(p.get("name").and_then(Value::as_str).unwrap_or(""))
    })
}

pub fn validate_capability_descriptor(input: &Value) -> ContractCheck<Value> {
    let o = input.as_object().ok_or(ContractError("INVALID_CAPABILITY_BASE"))?;
    if o.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || !o.get("appId").is_some_and(contract_app_id)
        || !o.get("operationId").is_some_and(contract_id)
        || !o.get("revision").is_some_and(contract_id)
        || !o.get("displayName").and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty() && s.len() <= 160)
        || !valid_scopes(o.get("requiredScopes"))
        || !valid_parameters(o.get("inputParameters"))
        || !valid_parameters(o.get("outputParameters"))
    {
        return Err(ContractError("INVALID_CAPABILITY_BASE"));
    }
    match o.get("kind").and_then(Value::as_str) {
        Some("workflow") => {
            let mut allowed = CAP_BASE.to_vec();
            allowed.extend(["workflowId", "ownerUserId", "sourceWorkspaceId", "published"]);
            if !exact_fields(input, &allowed)
                || !o.get("workflowId").is_some_and(contract_id)
                || !o.get("ownerUserId").is_some_and(contract_id)
                || !o.get("sourceWorkspaceId").is_some_and(contract_id)
                || o.get("published") != Some(&Value::Bool(true))
            {
                return Err(ContractError("INVALID_OR_PRIVATE_WORKFLOW_PUBLICATION"));
            }
        }
        Some("backend" | "frontend") => {
            let mut allowed = CAP_BASE.to_vec();
            allowed.extend(["deploymentId", "sessionPermission"]);
            let is_frontend = o.get("kind").and_then(Value::as_str) == Some("frontend");
            if !exact_fields(input, &allowed)
                || !o.get("deploymentId").is_some_and(contract_id)
                || (is_frontend && !matches!(o.get("sessionPermission").and_then(Value::as_str), Some("inspect" | "act")))
                || (!is_frontend && o.contains_key("sessionPermission"))
            {
                return Err(ContractError("INVALID_STATIC_CAPABILITY"));
            }
        }
        _ => return Err(ContractError("UNKNOWN_CAPABILITY_KIND")),
    }
    Ok(input.clone())
}

const INVOCATION_KEYS: &[&str] = &[
    "schemaVersion", "callerAppId", "targetAppId", "audienceAppId", "principalUserId",
    "callerWorkspaceId", "targetWorkspaceId", "operationId", "expectedRevision",
    "operationKind", "frontendSessionId", "frontendPermission", "frontendConsentId",
    "grantId", "traceId", "activeAppChain", "deadlineAtMs", "maxDepth",
    "remainingCalls", "remainingRetries",
];

fn safe_integer(v: Option<&Value>) -> Option<f64> {
    let n = v?.as_f64()?;
    (n.is_finite() && n.fract() == 0.0 && n.abs() <= MAX_SAFE_INTEGER).then_some(n)
}
pub fn validate_scoped_invocation(input: &Value) -> ContractCheck<Value> {
    let o = input.as_object().filter(|o| contract_exact_keys(o, INVOCATION_KEYS))
        .ok_or(ContractError("INVALID_INVOCATION_IDENTITY_OR_AUDIENCE"))?;
    let string = |key: &str| o.get(key).and_then(Value::as_str);
    let kind = string("operationKind").unwrap_or("");
    if o.get("schemaVersion").and_then(Value::as_u64) != Some(1)
        || !["callerAppId", "targetAppId", "audienceAppId"].iter().all(|key| o.get(*key).is_some_and(contract_app_id))
        || !["principalUserId", "callerWorkspaceId", "targetWorkspaceId", "operationId",
            "expectedRevision", "grantId", "traceId"].iter().all(|key| o.get(*key).is_some_and(contract_id))
        || !matches!(kind, "backend" | "frontend" | "workflow")
        || string("audienceAppId") != string("targetAppId")
    {
        return Err(ContractError("INVALID_INVOCATION_IDENTITY_OR_AUDIENCE"));
    }
    if kind == "frontend" {
        if !o.get("frontendSessionId").is_some_and(contract_id)
            || !o.get("frontendConsentId").is_some_and(contract_id)
            || !matches!(string("frontendPermission"), Some("inspect" | "act"))
        {
            return Err(ContractError("FRONTEND_REQUIRES_LIVE_SESSION_AND_CONSENT"));
        }
    } else if ["frontendSessionId", "frontendConsentId", "frontendPermission"].iter().any(|key| o.contains_key(*key)) {
        return Err(ContractError("NON_FRONTEND_SESSION_NOT_ALLOWED"));
    }
    let chain = o.get("activeAppChain").and_then(Value::as_array)
        .ok_or(ContractError("INVALID_CALL_ANCESTRY"))?;
    if !(1..=16).contains(&chain.len())
        || !chain.iter().all(contract_app_id)
        || chain.last().and_then(Value::as_str) != string("callerAppId")
    {
        return Err(ContractError("INVALID_CALL_ANCESTRY"));
    }
    let depth = safe_integer(o.get("maxDepth"));
    let calls = safe_integer(o.get("remainingCalls"));
    let retries = safe_integer(o.get("remainingRetries"));
    if safe_integer(o.get("deadlineAtMs")).is_none()
        || !depth.is_some_and(|n| (1.0..=16.0).contains(&n))
        || !calls.is_some_and(|n| (0.0..=1000.0).contains(&n))
        || !retries.is_some_and(|n| (0.0..=20.0).contains(&n))
    {
        return Err(ContractError("INVALID_EXECUTION_BUDGET"));
    }
    Ok(input.clone())
}

pub fn check_invocation_budget(input: &Value, now_ms: f64) -> ContractCheck<Value> {
    let call = validate_scoped_invocation(input)?;
    let o = call.as_object().ok_or(ContractError("INVALID_INVOCATION_IDENTITY_OR_AUDIENCE"))?;
    let deadline = o.get("deadlineAtMs").and_then(Value::as_f64).unwrap_or(0.0);
    if !now_ms.is_finite() || now_ms >= deadline {
        return Err(ContractError("INVOCATION_DEADLINE_EXCEEDED"));
    }
    let chain = o.get("activeAppChain").and_then(Value::as_array).ok_or(ContractError("INVALID_CALL_ANCESTRY"))?;
    let max_depth = o.get("maxDepth").and_then(Value::as_u64).unwrap_or(0) as usize;
    if chain.len() >= max_depth {
        return Err(ContractError("INVOCATION_MAX_DEPTH"));
    }
    if chain.iter().any(|v| v == &o["targetAppId"]) {
        return Err(ContractError("INVOCATION_CYCLE"));
    }
    if safe_integer(o.get("remainingCalls")) == Some(0.0) {
        return Err(ContractError("INVOCATION_WORK_BUDGET_EXHAUSTED"));
    }
    Ok(call)
}

fn budget_result(input: &Value, now_ms: f64, retry: bool) -> ContractCheck<Value> {
    let call = check_invocation_budget(input, now_ms)?;
    let o = call.as_object().ok_or(ContractError("INVALID_INVOCATION_IDENTITY_OR_AUDIENCE"))?;
    let calls = safe_integer(o.get("remainingCalls")).unwrap_or(0.0) as i64;
    let retries = safe_integer(o.get("remainingRetries")).unwrap_or(0.0) as i64;
    if retry && retries == 0 {
        return Err(ContractError("INVOCATION_RETRY_BUDGET_EXHAUSTED"));
    }
    let mut chain = o.get("activeAppChain").and_then(Value::as_array).cloned()
        .ok_or(ContractError("INVALID_CALL_ANCESTRY"))?;
    if !retry { chain.push(o["targetAppId"].clone()); }
    Ok(json!({
        "activeAppChain":chain,
        "remainingCalls":if retry { calls } else { calls - 1 },
        "remainingRetries":if retry { retries - 1 } else { retries },
        "deadlineAtMs":o["deadlineAtMs"],
    }))
}
pub fn advance_invocation_budget(input: &Value, now_ms: f64) -> ContractCheck<Value> {
    budget_result(input, now_ms, false)
}
pub fn consume_invocation_retry(input: &Value, now_ms: f64) -> ContractCheck<Value> {
    budget_result(input, now_ms, true)
}

pub fn validate_invocation_against_descriptor(
    invocation: &Value, descriptor: &Value, now_ms: f64,
) -> ContractCheck<Value> {
    let call = check_invocation_budget(invocation, now_ms)?;
    let capability = validate_capability_descriptor(descriptor)?;
    let ci = call.as_object().ok_or(ContractError("INVALID_INVOCATION_IDENTITY_OR_AUDIENCE"))?;
    let cd = capability.as_object().ok_or(ContractError("INVALID_CAPABILITY_BASE"))?;
    if cd["appId"] != ci["targetAppId"]
        || cd["operationId"] != ci["operationId"]
        || cd["revision"] != ci["expectedRevision"]
        || cd["kind"] != ci["operationKind"]
    {
        return Err(ContractError("TARGET_CAPABILITY_REVISION_OR_KIND_MISMATCH"));
    }
    if cd["kind"] == "workflow" && cd["sourceWorkspaceId"] != ci["targetWorkspaceId"] {
        return Err(ContractError("WORKFLOW_WORKSPACE_MISMATCH"));
    }
    if cd["kind"] == "frontend" && cd["sessionPermission"] != ci["frontendPermission"] {
        return Err(ContractError("FRONTEND_PERMISSION_MISMATCH"));
    }
    Ok(json!({"invocation":call,"descriptor":capability}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invocation() -> Value {
        json!({
            "schemaVersion":1,"callerAppId":"aiko","targetAppId":"kone","audienceAppId":"kone",
            "principalUserId":"user-1","callerWorkspaceId":"ws-1","targetWorkspaceId":"ws-2",
            "operationId":"play","expectedRevision":"v1","operationKind":"backend",
            "grantId":"grant-1","traceId":"trace-1","activeAppChain":["aiko"],
            "deadlineAtMs":2000000000000_i64,"maxDepth":4,
            "remainingCalls":2,"remainingRetries":1
        })
    }
    fn capability() -> Value {
        json!({
            "schemaVersion":1,"appId":"kone","operationId":"play","revision":"v1",
            "displayName":"Play","requiredScopes":["play.read"],
            "inputParameters":[{"name":"track","kind":"string","required":true}],
            "outputParameters":[],"kind":"backend","deploymentId":"dep-1"
        })
    }
    #[test]
    fn canonical_web_and_loopback_redirects() {
        assert!(validate_oauth_redirect_uri(&json!("https://example.com/callback"),"web",&[]).is_ok());
        assert!(validate_oauth_redirect_uri(&json!("https://example.com"),"web",&[]).is_err());
        assert!(validate_oauth_redirect_uri(&json!("https://example.com/cb?token=x"),"web",&[]).is_err());
        assert!(validate_oauth_redirect_uri(&json!("http://127.0.0.1:3100/callback"),"desktop",&[]).is_ok());
        assert!(validate_oauth_redirect_uri(&json!("http://localhost:3100/callback"),"desktop",&[]).is_err());
        assert!(validate_oauth_redirect_uri(&json!("https://mobile.example/cb"),"ios",&["mobile.example".into()]).is_ok());
        assert!(validate_oauth_redirect_uri(&json!("https://mobile.example/cb"),"ios",&[]).is_err());
    }
    #[test]
    fn registered_client_requires_only_canonical_fields() {
        let good = json!({"schemaVersion":1,"appId":"aiko","clientId":"client-1","platform":"web",
            "publicClient":true,"pkceMethod":"S256","redirectUris":["https://example.com/cb"]});
        assert!(validate_oauth_client_registration(&good).is_ok());
        let mut wrong = good.clone();
        wrong["clientSecret"] = json!("should-never-publish");
        assert_eq!(validate_oauth_client_registration(&wrong),Err(ContractError("INVALID_CLIENT_FIELDS")));
    }
    #[test]
    fn validates_public_capabilities_and_execution_budgets() {
        let request = invocation();
        assert!(validate_scoped_invocation(&request).is_ok());
        assert!(validate_capability_descriptor(&capability()).is_ok());
        assert!(validate_invocation_against_descriptor(&request,&capability(),1000.0).is_ok());
        assert_eq!(advance_invocation_budget(&request,1000.0).unwrap()["remainingCalls"],1);
        assert_eq!(consume_invocation_retry(&request,1000.0).unwrap()["remainingRetries"],0);
        assert_eq!(check_invocation_budget(&request,2000000000000.0),
            Err(ContractError("INVOCATION_DEADLINE_EXCEEDED")));
        let mut unsafe_request = request.clone();
        unsafe_request["audienceAppId"] = json!("attacker");
        assert_eq!(validate_scoped_invocation(&unsafe_request),
            Err(ContractError("INVALID_INVOCATION_IDENTITY_OR_AUDIENCE")));
        let mut cyclic = request;
        cyclic["activeAppChain"] = json!(["kone","aiko"]);
        assert_eq!(check_invocation_budget(&cyclic,1000.0),Err(ContractError("INVOCATION_CYCLE")));
        let mut secret = capability();
        secret["token"] = json!("no");
        assert_eq!(validate_capability_descriptor(&secret),Err(ContractError("INVALID_STATIC_CAPABILITY")));
    }
}
