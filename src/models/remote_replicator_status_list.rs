/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RemoteReplicatorStatusList : List of remote replicator status

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteReplicatorStatusList {}

impl Default for RemoteReplicatorStatusList {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteReplicatorStatusList {
    /// List of remote replicator status
    pub fn new() -> RemoteReplicatorStatusList {
        RemoteReplicatorStatusList {}
    }
}
