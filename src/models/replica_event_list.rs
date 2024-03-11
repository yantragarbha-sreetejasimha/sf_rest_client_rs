/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicaEventList : A list of ReplicaEvent objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaEventList {}

impl Default for ReplicaEventList {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplicaEventList {
    /// A list of ReplicaEvent objects.
    pub fn new() -> ReplicaEventList {
        ReplicaEventList {}
    }
}
