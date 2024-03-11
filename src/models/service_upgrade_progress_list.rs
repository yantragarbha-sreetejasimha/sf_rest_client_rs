/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceUpgradeProgressList : List of service upgrade progresses.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceUpgradeProgressList {}

impl Default for ServiceUpgradeProgressList {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceUpgradeProgressList {
    /// List of service upgrade progresses.
    pub fn new() -> ServiceUpgradeProgressList {
        ServiceUpgradeProgressList {}
    }
}