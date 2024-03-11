/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TimeList : Represents the list of exact time during the day in ISO8601 format. Like '19:00:00' will represent '7PM' during the day. Date specified along with time will be ignored.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeList {}

impl Default for TimeList {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeList {
    /// Represents the list of exact time during the day in ISO8601 format. Like '19:00:00' will represent '7PM' during the day. Date specified along with time will be ignored.
    pub fn new() -> TimeList {
        TimeList {}
    }
}