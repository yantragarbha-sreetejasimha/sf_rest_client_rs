/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupConfigurationInfo : Describes the backup configuration information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupConfigurationInfo {
    /// The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled.
    #[serde(rename = "Kind")]
    kind: ::models::BackupEntityKind,
    /// The name of the backup policy which is applicable to this Service Fabric application or service or partition.
    #[serde(rename = "PolicyName")]
    policy_name: Option<String>,
    /// Specifies the scope at which the backup policy is applied.
    #[serde(rename = "PolicyInheritedFrom")]
    policy_inherited_from: Option<::models::BackupPolicyScope>,
    /// Describes the backup suspension details.
    #[serde(rename = "SuspensionInfo")]
    suspension_info: Option<::models::BackupSuspensionInfo>,
}

impl BackupConfigurationInfo {
    /// Describes the backup configuration information.
    pub fn new(kind: ::models::BackupEntityKind) -> BackupConfigurationInfo {
        BackupConfigurationInfo {
            kind,
            policy_name: None,
            policy_inherited_from: None,
            suspension_info: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::BackupEntityKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::BackupEntityKind,
    ) -> BackupConfigurationInfo {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::BackupEntityKind {
        &self.kind
    }

    pub fn set_policy_name(&mut self, policy_name: String) {
        self.policy_name = Some(policy_name);
    }

    pub fn with_policy_name(
        mut self,
        policy_name: String,
    ) -> BackupConfigurationInfo {
        self.policy_name = Some(policy_name);
        self
    }

    pub fn policy_name(&self) -> Option<&String> {
        self.policy_name.as_ref()
    }

    pub fn reset_policy_name(&mut self) {
        self.policy_name = None;
    }

    pub fn set_policy_inherited_from(
        &mut self,
        policy_inherited_from: ::models::BackupPolicyScope,
    ) {
        self.policy_inherited_from = Some(policy_inherited_from);
    }

    pub fn with_policy_inherited_from(
        mut self,
        policy_inherited_from: ::models::BackupPolicyScope,
    ) -> BackupConfigurationInfo {
        self.policy_inherited_from = Some(policy_inherited_from);
        self
    }

    pub fn policy_inherited_from(
        &self,
    ) -> Option<&::models::BackupPolicyScope> {
        self.policy_inherited_from.as_ref()
    }

    pub fn reset_policy_inherited_from(&mut self) {
        self.policy_inherited_from = None;
    }

    pub fn set_suspension_info(
        &mut self,
        suspension_info: ::models::BackupSuspensionInfo,
    ) {
        self.suspension_info = Some(suspension_info);
    }

    pub fn with_suspension_info(
        mut self,
        suspension_info: ::models::BackupSuspensionInfo,
    ) -> BackupConfigurationInfo {
        self.suspension_info = Some(suspension_info);
        self
    }

    pub fn suspension_info(&self) -> Option<&::models::BackupSuspensionInfo> {
        self.suspension_info.as_ref()
    }

    pub fn reset_suspension_info(&mut self) {
        self.suspension_info = None;
    }
}
