/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationResourceUpgradeState : The state of the application resource upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationResourceUpgradeState {}

impl Default for ApplicationResourceUpgradeState {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationResourceUpgradeState {
    /// The state of the application resource upgrade.
    pub fn new() -> ApplicationResourceUpgradeState {
        ApplicationResourceUpgradeState {}
    }
}

// TODO enum
// List of ApplicationResourceUpgradeState
//const (
//
//
//
//)
