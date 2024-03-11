/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupSuspensionInfo : Describes the backup suspension details.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupSuspensionInfo {
    /// Indicates whether periodic backup is suspended at this level or not.
    #[serde(rename = "IsSuspended")]
    is_suspended: Option<bool>,
    /// Specifies the scope at which the backup suspension was applied.
    #[serde(rename = "SuspensionInheritedFrom")]
    suspension_inherited_from: Option<::models::BackupSuspensionScope>,
}

impl Default for BackupSuspensionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupSuspensionInfo {
    /// Describes the backup suspension details.
    pub fn new() -> BackupSuspensionInfo {
        BackupSuspensionInfo {
            is_suspended: None,
            suspension_inherited_from: None,
        }
    }

    pub fn set_is_suspended(&mut self, is_suspended: bool) {
        self.is_suspended = Some(is_suspended);
    }

    pub fn with_is_suspended(
        mut self,
        is_suspended: bool,
    ) -> BackupSuspensionInfo {
        self.is_suspended = Some(is_suspended);
        self
    }

    pub fn is_suspended(&self) -> Option<&bool> {
        self.is_suspended.as_ref()
    }

    pub fn reset_is_suspended(&mut self) {
        self.is_suspended = None;
    }

    pub fn set_suspension_inherited_from(
        &mut self,
        suspension_inherited_from: ::models::BackupSuspensionScope,
    ) {
        self.suspension_inherited_from = Some(suspension_inherited_from);
    }

    pub fn with_suspension_inherited_from(
        mut self,
        suspension_inherited_from: ::models::BackupSuspensionScope,
    ) -> BackupSuspensionInfo {
        self.suspension_inherited_from = Some(suspension_inherited_from);
        self
    }

    pub fn suspension_inherited_from(
        &self,
    ) -> Option<&::models::BackupSuspensionScope> {
        self.suspension_inherited_from.as_ref()
    }

    pub fn reset_suspension_inherited_from(&mut self) {
        self.suspension_inherited_from = None;
    }
}
