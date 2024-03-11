/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeDeactivationTaskList : List of tasks representing the deactivation operation on the node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDeactivationTaskList {}

impl Default for NodeDeactivationTaskList {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeDeactivationTaskList {
    /// List of tasks representing the deactivation operation on the node.
    pub fn new() -> NodeDeactivationTaskList {
        NodeDeactivationTaskList {}
    }
}
