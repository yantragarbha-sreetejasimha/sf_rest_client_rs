/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeName : The name of a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeName {}

impl Default for NodeName {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeName {
    /// The name of a Service Fabric node.
    pub fn new() -> NodeName {
        NodeName {}
    }
}
