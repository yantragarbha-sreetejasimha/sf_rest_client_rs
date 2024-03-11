/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeState : The state of the upgrade domain.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeState {}

impl Default for UpgradeState {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeState {
    /// The state of the upgrade domain.
    pub fn new() -> UpgradeState {
        UpgradeState {}
    }
}

// TODO enum
// List of UpgradeState
//const (
//
//
//
//)
