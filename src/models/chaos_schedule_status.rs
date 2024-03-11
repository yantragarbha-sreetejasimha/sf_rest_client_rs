/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosScheduleStatus : Current status of the schedule.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosScheduleStatus {}

impl Default for ChaosScheduleStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosScheduleStatus {
    /// Current status of the schedule.
    pub fn new() -> ChaosScheduleStatus {
        ChaosScheduleStatus {}
    }
}

// TODO enum
// List of ChaosScheduleStatus
//const (
//
//
//
//)
