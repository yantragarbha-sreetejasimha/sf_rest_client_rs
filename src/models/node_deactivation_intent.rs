/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeDeactivationIntent : The intent or the reason for deactivating the node. Following are the possible values for it.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDeactivationIntent {}

impl Default for NodeDeactivationIntent {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeDeactivationIntent {
    /// The intent or the reason for deactivating the node. Following are the possible values for it.
    pub fn new() -> NodeDeactivationIntent {
        NodeDeactivationIntent {}
    }
}

// TODO enum
// List of NodeDeactivationIntent
//const (
//
//
//
//)
