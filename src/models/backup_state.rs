/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupState : Represents the current state of the partition backup operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupState {}

impl Default for BackupState {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupState {
    /// Represents the current state of the partition backup operation.
    pub fn new() -> BackupState {
        BackupState {}
    }
}

// TODO enum
// List of BackupState
//const (
//
//
//
//)
