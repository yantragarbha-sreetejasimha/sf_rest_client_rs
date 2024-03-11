/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicaStatus : The status of a replica of a service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaStatus {}

impl Default for ReplicaStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplicaStatus {
    /// The status of a replica of a service.
    pub fn new() -> ReplicaStatus {
        ReplicaStatus {}
    }
}

// TODO enum
// List of ReplicaStatus
//const (
//
//
//
//)
