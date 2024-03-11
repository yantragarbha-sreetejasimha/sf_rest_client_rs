/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeSortOrder : Defines the order in which an upgrade proceeds through the cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeSortOrder {}

impl Default for UpgradeSortOrder {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeSortOrder {
    /// Defines the order in which an upgrade proceeds through the cluster.
    pub fn new() -> UpgradeSortOrder {
        UpgradeSortOrder {}
    }
}

// TODO enum
// List of UpgradeSortOrder
//const (
//
//
//
//)
