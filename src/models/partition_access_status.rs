/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionAccessStatus : Specifies the access status of the partition.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionAccessStatus {}

impl Default for PartitionAccessStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionAccessStatus {
    /// Specifies the access status of the partition.
    pub fn new() -> PartitionAccessStatus {
        PartitionAccessStatus {}
    }
}

// TODO enum
// List of PartitionAccessStatus
//const (
//
//
//
//)
