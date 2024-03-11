/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RestorePartitionDescription : Specifies the parameters needed to trigger a restore of a specific partition.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RestorePartitionDescription {
    /// Unique backup ID.
    #[serde(rename = "BackupId")]
    backup_id: String,
    /// Location of the backup relative to the backup storage specified/ configured.
    #[serde(rename = "BackupLocation")]
    backup_location: String,
    /// Location of the backup from where the partition will be restored.
    #[serde(rename = "BackupStorage")]
    backup_storage: Option<::models::BackupStorageDescription>,
}

impl RestorePartitionDescription {
    /// Specifies the parameters needed to trigger a restore of a specific partition.
    pub fn new(
        backup_id: String,
        backup_location: String,
    ) -> RestorePartitionDescription {
        RestorePartitionDescription {
            backup_id,
            backup_location,
            backup_storage: None,
        }
    }

    pub fn set_backup_id(&mut self, backup_id: String) {
        self.backup_id = backup_id;
    }

    pub fn with_backup_id(
        mut self,
        backup_id: String,
    ) -> RestorePartitionDescription {
        self.backup_id = backup_id;
        self
    }

    pub fn backup_id(&self) -> &String {
        &self.backup_id
    }

    pub fn set_backup_location(&mut self, backup_location: String) {
        self.backup_location = backup_location;
    }

    pub fn with_backup_location(
        mut self,
        backup_location: String,
    ) -> RestorePartitionDescription {
        self.backup_location = backup_location;
        self
    }

    pub fn backup_location(&self) -> &String {
        &self.backup_location
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
    ) -> RestorePartitionDescription {
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
