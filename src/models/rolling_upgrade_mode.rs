/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RollingUpgradeMode : The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RollingUpgradeMode {}

impl Default for RollingUpgradeMode {
    fn default() -> Self {
        Self::new()
    }
}

impl RollingUpgradeMode {
    /// The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored.
    pub fn new() -> RollingUpgradeMode {
        RollingUpgradeMode {}
    }
}

// TODO enum
// List of RollingUpgradeMode
//const (
//
//
//
//)
