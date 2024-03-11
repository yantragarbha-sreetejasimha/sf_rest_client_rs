/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupSuspensionScope : Specifies the scope at which the backup suspension was applied.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupSuspensionScope {}

impl Default for BackupSuspensionScope {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupSuspensionScope {
    /// Specifies the scope at which the backup suspension was applied.
    pub fn new() -> BackupSuspensionScope {
        BackupSuspensionScope {}
    }
}

// TODO enum
// List of BackupSuspensionScope
//const (
//
//
//
//)
