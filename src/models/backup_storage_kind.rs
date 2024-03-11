/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupStorageKind : The kind of backup storage, where backups are saved.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupStorageKind {}

impl Default for BackupStorageKind {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupStorageKind {
    /// The kind of backup storage, where backups are saved.
    pub fn new() -> BackupStorageKind {
        BackupStorageKind {}
    }
}

// TODO enum
// List of BackupStorageKind
//const (
//
//
//
//)
