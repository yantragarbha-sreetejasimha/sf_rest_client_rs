/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeTypeName : The node type name as defined in the cluster manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTypeName {}

impl Default for NodeTypeName {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeTypeName {
    /// The node type name as defined in the cluster manifest.
    pub fn new() -> NodeTypeName {
        NodeTypeName {}
    }
}
