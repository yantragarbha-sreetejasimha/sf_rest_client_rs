/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeStartTimeUtcString : The start time of the upgrade in UTC.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeStartTimeUtcString {}

impl Default for UpgradeStartTimeUtcString {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeStartTimeUtcString {
    /// The start time of the upgrade in UTC.
    pub fn new() -> UpgradeStartTimeUtcString {
        UpgradeStartTimeUtcString {}
    }
}
