/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterEventList : A list of ClusterEvent objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterEventList {}

impl Default for ClusterEventList {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterEventList {
    /// A list of ClusterEvent objects.
    pub fn new() -> ClusterEventList {
        ClusterEventList {}
    }
}
