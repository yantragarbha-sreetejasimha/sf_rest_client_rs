/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeDomainName : The name of the upgrade domain

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeDomainName {}

impl Default for UpgradeDomainName {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeDomainName {
    /// The name of the upgrade domain
    pub fn new() -> UpgradeDomainName {
        UpgradeDomainName {}
    }
}
