/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DsmsAzureBlobBackupStorageDescription : Describes the parameters for Dsms Azure blob store used for storing and enumerating backups.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DsmsAzureBlobBackupStorageDescription {
    /// The kind of backup storage, where backups are saved.
    #[serde(rename = "StorageKind")]
    storage_kind: ::models::BackupStorageKind,
    /// Friendly name for this backup storage.
    #[serde(rename = "FriendlyName")]
    friendly_name: Option<String>,
    /// The source location of the storage credentials to connect to the Dsms Azure blob store.
    #[serde(rename = "StorageCredentialsSourceLocation")]
    storage_credentials_source_location: String,
    /// The name of the container in the blob store to store and enumerate backups from.
    #[serde(rename = "ContainerName")]
    container_name: String,
}

impl DsmsAzureBlobBackupStorageDescription {
    /// Describes the parameters for Dsms Azure blob store used for storing and enumerating backups.
    pub fn new(
        storage_kind: ::models::BackupStorageKind,
        storage_credentials_source_location: String,
        container_name: String,
    ) -> DsmsAzureBlobBackupStorageDescription {
        DsmsAzureBlobBackupStorageDescription {
            storage_kind,
            friendly_name: None,
            storage_credentials_source_location,
            container_name,
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
    ) -> DsmsAzureBlobBackupStorageDescription {
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
    ) -> DsmsAzureBlobBackupStorageDescription {
        self.friendly_name = Some(friendly_name);
        self
    }

    pub fn friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    pub fn reset_friendly_name(&mut self) {
        self.friendly_name = None;
    }

    pub fn set_storage_credentials_source_location(
        &mut self,
        storage_credentials_source_location: String,
    ) {
        self.storage_credentials_source_location =
            storage_credentials_source_location;
    }

    pub fn with_storage_credentials_source_location(
        mut self,
        storage_credentials_source_location: String,
    ) -> DsmsAzureBlobBackupStorageDescription {
        self.storage_credentials_source_location =
            storage_credentials_source_location;
        self
    }

    pub fn storage_credentials_source_location(&self) -> &String {
        &self.storage_credentials_source_location
    }

    pub fn set_container_name(&mut self, container_name: String) {
        self.container_name = container_name;
    }

    pub fn with_container_name(
        mut self,
        container_name: String,
    ) -> DsmsAzureBlobBackupStorageDescription {
        self.container_name = container_name;
        self
    }

    pub fn container_name(&self) -> &String {
        &self.container_name
    }
}