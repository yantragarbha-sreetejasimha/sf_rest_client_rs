/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EnableBackupDescription : Specifies the parameters needed to enable periodic backup.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnableBackupDescription {
    /// Name of the backup policy to be used for enabling periodic backups.
    #[serde(rename = "BackupPolicyName")]
    backup_policy_name: String,
}

impl EnableBackupDescription {
    /// Specifies the parameters needed to enable periodic backup.
    pub fn new(backup_policy_name: String) -> EnableBackupDescription {
        EnableBackupDescription { backup_policy_name }
    }

    pub fn set_backup_policy_name(&mut self, backup_policy_name: String) {
        self.backup_policy_name = backup_policy_name;
    }

    pub fn with_backup_policy_name(
        mut self,
        backup_policy_name: String,
    ) -> EnableBackupDescription {
        self.backup_policy_name = backup_policy_name;
        self
    }

    pub fn backup_policy_name(&self) -> &String {
        &self.backup_policy_name
    }
}
