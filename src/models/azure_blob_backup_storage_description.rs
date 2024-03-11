/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AzureBlobBackupStorageDescription : Describes the parameters for Azure blob store used for storing and enumerating backups.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureBlobBackupStorageDescription {
    /// The kind of backup storage, where backups are saved.
    #[serde(rename = "StorageKind")]
    storage_kind: ::models::BackupStorageKind,
    /// Friendly name for this backup storage.
    #[serde(rename = "FriendlyName")]
    friendly_name: Option<String>,
    /// The connection string to connect to the Azure blob store.
    #[serde(rename = "ConnectionString")]
    connection_string: String,
    /// The name of the container in the blob store to store and enumerate backups from.
    #[serde(rename = "ContainerName")]
    container_name: String,
}

impl AzureBlobBackupStorageDescription {
    /// Describes the parameters for Azure blob store used for storing and enumerating backups.
    pub fn new(
        storage_kind: ::models::BackupStorageKind,
        connection_string: String,
        container_name: String,
    ) -> AzureBlobBackupStorageDescription {
        AzureBlobBackupStorageDescription {
            storage_kind,
            friendly_name: None,
            connection_string,
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
    ) -> AzureBlobBackupStorageDescription {
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
    ) -> AzureBlobBackupStorageDescription {
        self.friendly_name = Some(friendly_name);
        self
    }

    pub fn friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    pub fn reset_friendly_name(&mut self) {
        self.friendly_name = None;
    }

    pub fn set_connection_string(&mut self, connection_string: String) {
        self.connection_string = connection_string;
    }

    pub fn with_connection_string(
        mut self,
        connection_string: String,
    ) -> AzureBlobBackupStorageDescription {
        self.connection_string = connection_string;
        self
    }

    pub fn connection_string(&self) -> &String {
        &self.connection_string
    }

    pub fn set_container_name(&mut self, container_name: String) {
        self.container_name = container_name;
    }

    pub fn with_container_name(
        mut self,
        container_name: String,
    ) -> AzureBlobBackupStorageDescription {
        self.container_name = container_name;
        self
    }

    pub fn container_name(&self) -> &String {
        &self.container_name
    }
}
