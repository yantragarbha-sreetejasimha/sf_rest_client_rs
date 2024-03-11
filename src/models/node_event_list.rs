/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeEventList : A list of NodeEvent objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeEventList {}

impl Default for NodeEventList {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeEventList {
    /// A list of NodeEvent objects.
    pub fn new() -> NodeEventList {
        NodeEventList {}
    }
}
