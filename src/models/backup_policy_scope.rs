/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupPolicyScope : Specifies the scope at which the backup policy is applied.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupPolicyScope {}

impl Default for BackupPolicyScope {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupPolicyScope {
    /// Specifies the scope at which the backup policy is applied.
    pub fn new() -> BackupPolicyScope {
        BackupPolicyScope {}
    }
}

// TODO enum
// List of BackupPolicyScope
//const (
//
//
//
//)
