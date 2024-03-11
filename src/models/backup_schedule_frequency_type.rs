/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupScheduleFrequencyType : Describes the frequency with which to run the time based backup schedule.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupScheduleFrequencyType {}

impl Default for BackupScheduleFrequencyType {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupScheduleFrequencyType {
    /// Describes the frequency with which to run the time based backup schedule.
    pub fn new() -> BackupScheduleFrequencyType {
        BackupScheduleFrequencyType {}
    }
}

// TODO enum
// List of BackupScheduleFrequencyType
//const (
//
//
//
//)