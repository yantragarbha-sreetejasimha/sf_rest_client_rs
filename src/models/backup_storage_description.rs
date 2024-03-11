/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupStorageDescription : Describes the parameters for the backup storage.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupStorageDescription {
    /// The kind of backup storage, where backups are saved.
    #[serde(rename = "StorageKind")]
    storage_kind: ::models::BackupStorageKind,
    /// Friendly name for this backup storage.
    #[serde(rename = "FriendlyName")]
    friendly_name: Option<String>,
}

impl BackupStorageDescription {
    /// Describes the parameters for the backup storage.
    pub fn new(
        storage_kind: ::models::BackupStorageKind,
    ) -> BackupStorageDescription {
        BackupStorageDescription {
            storage_kind,
            friendly_name: None,
        }
    }

    pub fn set_storage_kind(
        &mut self,
        storage_kind: ::models::BackupStorageKind,
    ) {
        self.storage_kind = storage_kind;
    }

    pub fn with_storage_kind(
        mut self,
        storage_kind: ::models::BackupStorageKind,
    ) -> BackupStorageDescription {
        self.storage_kind = storage_kind;
        self
    }

    pub fn storage_kind(&self) -> &::models::BackupStorageKind {
        &self.storage_kind
    }

    pub fn set_friendly_name(&mut self, friendly_name: String) {
        self.friendly_name = Some(friendly_name);
    }

    pub fn with_friendly_name(
        mut self,
        friendly_name: String,
    ) -> BackupStorageDescription {
        self.friendly_name = Some(friendly_name);
        self
    }

    pub fn friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    pub fn reset_friendly_name(&mut self) {
        self.friendly_name = None;
    }
}