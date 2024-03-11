/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupPartitionDescription : Describes the parameters for triggering partition's backup.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupPartitionDescription {
    /// Specifies the details of the backup storage where to save the backup.
    #[serde(rename = "BackupStorage")]
    backup_storage: Option<::models::BackupStorageDescription>,
}

impl Default for BackupPartitionDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupPartitionDescription {
    /// Describes the parameters for triggering partition's backup.
    pub fn new() -> BackupPartitionDescription {
        BackupPartitionDescription {
            backup_storage: None,
        }
    }

    pub fn set_backup_storage(
        &mut self,
        backup_storage: ::models::BackupStorageDescription,
    ) {
        self.backup_storage = Some(backup_storage);
    }

    pub fn with_backup_storage(
        mut self,
        backup_storage: ::models::BackupStorageDescription,
    ) -> BackupPartitionDescription {
        self.backup_storage = Some(backup_storage);
        self
    }

    pub fn backup_storage(
        &self,
    ) -> Option<&::models::BackupStorageDescription> {
        self.backup_storage.as_ref()
    }

    pub fn reset_backup_storage(&mut self) {
        self.backup_storage = None;
    }
}