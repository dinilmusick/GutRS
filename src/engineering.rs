//! Wire-safe engineering contracts from GutTS engineering-types.ts.
//! These types convey data and declared grants; they never imply permission.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Identifier = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderKind {
    Github,
    Gitlab,
    Bitbucket,
    Git,
    Local,
    Container,
    Webcontainer,
    RemoteSandbox,
    Ci,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryRef {
    pub id: Identifier,
    pub provider: ProviderKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceRef {
    pub id: Identifier,
    pub root: String,
    pub repositories: Vec<RepositoryRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<Identifier>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkPolicy {
    None,
    Restricted,
    Full,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_mb: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionRequest {
    pub workspace_id: Identifier,
    pub command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<NetworkPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ExecutionResources>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionStatus {
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResult {
    pub execution_id: Identifier,
    pub status: ExecutionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub artifact_ids: Vec<Identifier>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckStatus {
    Passed,
    Failed,
    Skipped,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationCheck {
    pub id: Identifier,
    pub name: String,
    pub status: CheckStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<Identifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationReportStatus {
    Passed,
    Failed,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationReport {
    pub id: Identifier,
    pub status: VerificationReportStatus,
    pub checks: Vec<VerificationCheck>,
    pub artifact_ids: Vec<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactRef {
    pub id: Identifier,
    pub digest: String,
    #[serde(rename = "type")]
    pub artifact_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_execution_id: Option<Identifier>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineeringCapability {
    #[serde(rename = "repository.read")]
    RepositoryRead,
    #[serde(rename = "repository.write")]
    RepositoryWrite,
    #[serde(rename = "git.write")]
    GitWrite,
    #[serde(rename = "workspace.create")]
    WorkspaceCreate,
    #[serde(rename = "execution.run")]
    ExecutionRun,
    #[serde(rename = "browser.run")]
    BrowserRun,
    #[serde(rename = "verification.run")]
    VerificationRun,
    #[serde(rename = "artifact.write")]
    ArtifactWrite,
    #[serde(rename = "package.publish")]
    PackagePublish,
    #[serde(rename = "deployment.write")]
    DeploymentWrite,
    #[serde(rename = "promotion.write")]
    PromotionWrite,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityGrant {
    pub capability: EngineeringCapability,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineeringOperationContext {
    pub task_id: Identifier,
    pub trace_id: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Identifier>,
    pub grants: Vec<CapabilityGrant>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceRecord {
    pub operation_id: Identifier,
    pub actor_id: Identifier,
    pub task_id: Identifier,
    pub trace_id: Identifier,
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<RepositoryRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Identifier>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn upstream_wire_shapes_round_trip_without_unrecognized_capabilities() {
        let request = json!({
            "workspaceId":"workspace-1","command":"cargo test","cwd":"/repo",
            "network":"restricted","resources":{"cpu":2,"memoryMb":1024},
            "timeoutMs":30000
        });
        let typed: ExecutionRequest = serde_json::from_value(request.clone()).unwrap();
        assert_eq!(typed.network, Some(NetworkPolicy::Restricted));
        assert_eq!(serde_json::to_value(typed).unwrap(), request);
        let context = json!({"taskId":"task","traceId":"trace","grants":[
            {"capability":"repository.read","resource":"dinilmusick/GutRS"},
            {"capability":"verification.run"}]});
        let typed: EngineeringOperationContext = serde_json::from_value(context.clone()).unwrap();
        assert_eq!(serde_json::to_value(typed).unwrap(), context);
        assert!(
            serde_json::from_value::<EngineeringOperationContext>(json!({
                "taskId":"task","traceId":"trace","grants":[{"capability":"admin.superuser"}]
            }))
            .is_err()
        );
    }
}
