/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CurrentUpgradeDomainName : The name of the upgrade domain. Not applicable to node-by-node upgrades.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUpgradeDomainName {}

impl Default for CurrentUpgradeDomainName {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentUpgradeDomainName {
    /// The name of the upgrade domain. Not applicable to node-by-node upgrades.
    pub fn new() -> CurrentUpgradeDomainName {
        CurrentUpgradeDomainName {}
    }
}
