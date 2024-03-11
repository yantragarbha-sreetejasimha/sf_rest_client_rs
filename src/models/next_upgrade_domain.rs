/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NextUpgradeDomain : The name of the next upgrade domain to be processed.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NextUpgradeDomain {}

impl Default for NextUpgradeDomain {
    fn default() -> Self {
        Self::new()
    }
}

impl NextUpgradeDomain {
    /// The name of the next upgrade domain to be processed.
    pub fn new() -> NextUpgradeDomain {
        NextUpgradeDomain {}
    }
}
