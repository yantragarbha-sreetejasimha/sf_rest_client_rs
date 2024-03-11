/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairTaskList : A list of repair tasks.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairTaskList {}

impl Default for RepairTaskList {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairTaskList {
    /// A list of repair tasks.
    pub fn new() -> RepairTaskList {
        RepairTaskList {}
    }
}
