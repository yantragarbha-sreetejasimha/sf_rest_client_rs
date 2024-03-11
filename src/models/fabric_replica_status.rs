/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FabricReplicaStatus : Specifies the status of the replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricReplicaStatus {}

impl Default for FabricReplicaStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricReplicaStatus {
    /// Specifies the status of the replica.
    pub fn new() -> FabricReplicaStatus {
        FabricReplicaStatus {}
    }
}

// TODO enum
// List of FabricReplicaStatus
//const (
//
//
//
//)
