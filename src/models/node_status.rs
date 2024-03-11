/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeStatus : The status of the node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatus {}

impl Default for NodeStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeStatus {
    /// The status of the node.
    pub fn new() -> NodeStatus {
        NodeStatus {}
    }
}

// TODO enum
// List of NodeStatus
//const (
//
//
//
//)