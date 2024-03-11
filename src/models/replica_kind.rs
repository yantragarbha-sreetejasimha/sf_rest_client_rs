/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicaKind : The role of a replica of a stateful service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaKind {}

impl Default for ReplicaKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplicaKind {
    /// The role of a replica of a stateful service.
    pub fn new() -> ReplicaKind {
        ReplicaKind {}
    }
}

// TODO enum
// List of ReplicaKind
//const (
//
//
//
//)
