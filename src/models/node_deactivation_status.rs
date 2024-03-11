/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeDeactivationStatus : The status of node deactivation operation. Following are the possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDeactivationStatus {}

impl Default for NodeDeactivationStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeDeactivationStatus {
    /// The status of node deactivation operation. Following are the possible values.
    pub fn new() -> NodeDeactivationStatus {
        NodeDeactivationStatus {}
    }
}

// TODO enum
// List of NodeDeactivationStatus
//const (
//
//
//
//)
