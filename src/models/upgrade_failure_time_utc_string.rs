/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeFailureTimeUtcString : The failure time of the upgrade in UTC.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeFailureTimeUtcString {}

impl Default for UpgradeFailureTimeUtcString {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeFailureTimeUtcString {
    /// The failure time of the upgrade in UTC.
    pub fn new() -> UpgradeFailureTimeUtcString {
        UpgradeFailureTimeUtcString {}
    }
}
