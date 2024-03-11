/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupScheduleKind : The kind of backup schedule, time based or frequency based.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupScheduleKind {}

impl Default for BackupScheduleKind {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupScheduleKind {
    /// The kind of backup schedule, time based or frequency based.
    pub fn new() -> BackupScheduleKind {
        BackupScheduleKind {}
    }
}

// TODO enum
// List of BackupScheduleKind
//const (
//
//
//
//)