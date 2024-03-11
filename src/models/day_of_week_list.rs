/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DayOfWeekList : List of days of a week when to trigger the periodic backup. This is valid only when the backup schedule frequency type is weekly.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DayOfWeekList {}

impl Default for DayOfWeekList {
    fn default() -> Self {
        Self::new()
    }
}

impl DayOfWeekList {
    /// List of days of a week when to trigger the periodic backup. This is valid only when the backup schedule frequency type is weekly.
    pub fn new() -> DayOfWeekList {
        DayOfWeekList {}
    }
}
