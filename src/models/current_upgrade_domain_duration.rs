/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CurrentUpgradeDomainDuration : The estimated amount of time spent processing current Upgrade Domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUpgradeDomainDuration {}

impl Default for CurrentUpgradeDomainDuration {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentUpgradeDomainDuration {
    /// The estimated amount of time spent processing current Upgrade Domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds.
    pub fn new() -> CurrentUpgradeDomainDuration {
        CurrentUpgradeDomainDuration {}
    }
}
