/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionEventList : A list of PartitionEvent objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionEventList {}

impl Default for PartitionEventList {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionEventList {
    /// A list of PartitionEvent objects.
    pub fn new() -> PartitionEventList {
        PartitionEventList {}
    }
}
