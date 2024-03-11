/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupEntityKind : The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupEntityKind {}

impl Default for BackupEntityKind {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupEntityKind {
    /// The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled.
    pub fn new() -> BackupEntityKind {
        BackupEntityKind {}
    }
}

// TODO enum
// List of BackupEntityKind
//const (
//
//
//
//)
