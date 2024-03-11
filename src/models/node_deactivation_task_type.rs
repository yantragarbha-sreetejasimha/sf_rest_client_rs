/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeDeactivationTaskType : The type of the task that performed the node deactivation. Following are the possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDeactivationTaskType {}

impl Default for NodeDeactivationTaskType {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeDeactivationTaskType {
    /// The type of the task that performed the node deactivation. Following are the possible values.
    pub fn new() -> NodeDeactivationTaskType {
        NodeDeactivationTaskType {}
    }
}

// TODO enum
// List of NodeDeactivationTaskType
//const (
//
//
//
//)
