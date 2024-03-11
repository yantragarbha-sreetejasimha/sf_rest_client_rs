/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterFabricCodeVersionString : The ServiceFabric code version of the cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFabricCodeVersionString {}

impl Default for ClusterFabricCodeVersionString {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterFabricCodeVersionString {
    /// The ServiceFabric code version of the cluster.
    pub fn new() -> ClusterFabricCodeVersionString {
        ClusterFabricCodeVersionString {}
    }
}
