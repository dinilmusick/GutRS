//! Strongly typed portable authentication data structures corresponding to
//! pinned GutTS OAuth, capability, invocation and branded identity contracts.
//! Parsing untrusted transports MUST use checked constructors; serde shape
//! alone is never authorization or proof of session ownership.
use crate::auth_contracts::{
    validate_capability_descriptor, validate_oauth_client_registration, validate_scoped_invocation,
    ContractError,
};
use crate::identity::valid_identity_id;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityKind {
    Application,
    Deployment,
    OauthClient,
    User,
    Workspace,
    UiSession,
}

/// IdentityId<K> from TypeScript is represented by separate checked newtypes.
/// These ensure typed identities cannot be confused by native consumers.
macro_rules! typed_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);
        impl $name {
            pub fn parse(value: &str) -> Option<Self> {
                valid_identity_id(value).then(|| Self(value.to_owned()))
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::parse(&value).ok_or_else(|| D::Error::custom("invalid role identity ID"))
            }
        }
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}
typed_id!(ApplicationId);
typed_id!(DeploymentId);
typed_id!(OAuthClientId);
typed_id!(UserId);
typed_id!(WorkspaceId);
typed_id!(UiSessionId);

/// Generic wire identity carrying its explicit kind. The caller should
/// validate kind and ID using identity::parse_identity_ref first.
pub type IdentityId = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthPlatform {
    Web,
    Ios,
    Android,
    Desktop,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthClientRegistrationV1 {
    pub schema_version: u8,
    pub app_id: ApplicationId,
    pub client_id: OAuthClientId,
    pub platform: OAuthPlatform,
    pub public_client: bool,
    pub pkce_method: String,
    pub redirect_uris: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_hosts: Option<Vec<String>>,
}
impl OAuthClientRegistrationV1 {
    pub fn checked(input: &Value) -> Result<Self, ContractError> {
        let validated = validate_oauth_client_registration(input)?;
        serde_json::from_value(validated)
            .map_err(|_| ContractError("INVALID_CLIENT_IDENTITY_OR_PKCE"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CapabilityParameterKind {
    String,
    Number,
    Integer,
    Boolean,
    Object,
    Array,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityParameterV1 {
    pub name: String,
    pub kind: CapabilityParameterKind,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityBaseV1 {
    pub schema_version: u8,
    pub app_id: ApplicationId,
    pub operation_id: String,
    pub revision: String,
    pub display_name: String,
    pub required_scopes: Vec<String>,
    pub input_parameters: Vec<CapabilityParameterV1>,
    pub output_parameters: Vec<CapabilityParameterV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaticCapabilityKind {
    Backend,
    Frontend,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FrontendPermission {
    Inspect,
    Act,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticCapabilityV1 {
    #[serde(flatten)]
    pub base: CapabilityBaseV1,
    pub kind: StaticCapabilityKind,
    pub deployment_id: DeploymentId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_permission: Option<FrontendPermission>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowPublicationV1 {
    #[serde(flatten)]
    pub base: CapabilityBaseV1,
    pub kind: String,
    pub workflow_id: String,
    pub owner_user_id: UserId,
    pub source_workspace_id: WorkspaceId,
    pub published: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CapabilityDescriptorV1 {
    Static(StaticCapabilityV1),
    Workflow(WorkflowPublicationV1),
}
impl CapabilityDescriptorV1 {
    pub fn checked(input: &Value) -> Result<Self, ContractError> {
        let value = validate_capability_descriptor(input)?;
        serde_json::from_value(value).map_err(|_| ContractError("INVALID_CAPABILITY_BASE"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationKind {
    Backend,
    Frontend,
    Workflow,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopedInvocationV1 {
    pub schema_version: u8,
    pub caller_app_id: ApplicationId,
    pub target_app_id: ApplicationId,
    pub audience_app_id: ApplicationId,
    pub principal_user_id: UserId,
    pub caller_workspace_id: WorkspaceId,
    pub target_workspace_id: WorkspaceId,
    pub operation_id: String,
    pub expected_revision: String,
    pub operation_kind: OperationKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend_session_id: Option<UiSessionId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend_permission: Option<FrontendPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend_consent_id: Option<String>,
    pub grant_id: String,
    pub trace_id: String,
    pub active_app_chain: Vec<ApplicationId>,
    pub deadline_at_ms: i64,
    pub max_depth: u8,
    pub remaining_calls: u16,
    pub remaining_retries: u8,
}
impl ScopedInvocationV1 {
    pub fn checked(input: &Value) -> Result<Self, ContractError> {
        let value = validate_scoped_invocation(input)?;
        serde_json::from_value(value)
            .map_err(|_| ContractError("INVALID_INVOCATION_IDENTITY_OR_AUDIENCE"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn distinct_branded_ids_reject_invalid_characters_and_roles() {
        assert_eq!(ApplicationId::parse("aiko").unwrap().as_str(), "aiko");
        assert!(DeploymentId::parse("https://example.test").is_none());
        assert!(serde_json::from_value::<WorkspaceId>(json!(" aiko")).is_err());
        assert_eq!(
            serde_json::to_value(IdentityKind::OauthClient).unwrap(),
            json!("oauth_client")
        );
    }
    #[test]
    fn versioned_oauth_contract_rejects_secrets_and_serializes() {
        let payload = json!({"schemaVersion":1,"appId":"aiko","clientId":"ios-client-1",
            "platform":"ios","publicClient":true,"pkceMethod":"S256",
            "verifiedHosts":["example.org"],"redirectUris":["https://example.org/cb"]});
        let parsed = OAuthClientRegistrationV1::checked(&payload).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), payload);
        let mut unsafe_payload = payload;
        unsafe_payload["clientSecret"] = json!("SECRET");
        assert!(OAuthClientRegistrationV1::checked(&unsafe_payload).is_err());
    }
    #[test]
    fn published_capability_and_invocation_check_shape_and_audience() {
        let capability = json!({
            "schemaVersion":1,"appId":"kone","operationId":"play","revision":"v1",
            "displayName":"Play","requiredScopes":[],"inputParameters":[],
            "outputParameters":[],"kind":"backend","deploymentId":"dep-1"
        });
        let actual = CapabilityDescriptorV1::checked(&capability).unwrap();
        assert_eq!(serde_json::to_value(actual).unwrap(), capability);
        let invocation = json!({
            "schemaVersion":1,"callerAppId":"aiko","targetAppId":"kone","audienceAppId":"kone",
            "principalUserId":"user-1","callerWorkspaceId":"ws-1","targetWorkspaceId":"ws-2",
            "operationId":"play","expectedRevision":"v1","operationKind":"backend",
            "grantId":"grant-1","traceId":"trace-1","activeAppChain":["aiko"],
            "deadlineAtMs":2000000000000_i64,"maxDepth":4,"remainingCalls":2,"remainingRetries":1
        });
        let parsed = ScopedInvocationV1::checked(&invocation).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), invocation);
        let mut wrong = invocation.clone();
        wrong["audienceAppId"] = json!("attacker");
        assert!(ScopedInvocationV1::checked(&wrong).is_err());
        let mut bypass = capability;
        bypass["token"] = json!("unauthorized");
        assert!(CapabilityDescriptorV1::checked(&bypass).is_err());
    }
}
