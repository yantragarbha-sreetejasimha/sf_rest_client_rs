/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionScheme : Enumerates the ways that a service can be partitioned.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionScheme {}

impl Default for PartitionScheme {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionScheme {
    /// Enumerates the ways that a service can be partitioned.
    pub fn new() -> PartitionScheme {
        PartitionScheme {}
    }
}

// TODO enum
// List of PartitionScheme
//const (
//
//
//
//)
