/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SingletonPartitionSchemeDescription : Describes the partition scheme of a singleton-partitioned, or non-partitioned service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingletonPartitionSchemeDescription {
    /// Specifies how the service is partitioned.
    #[serde(rename = "PartitionScheme")]
    partition_scheme: ::models::PartitionScheme,
}

impl SingletonPartitionSchemeDescription {
    /// Describes the partition scheme of a singleton-partitioned, or non-partitioned service.
    pub fn new(
        partition_scheme: ::models::PartitionScheme,
    ) -> SingletonPartitionSchemeDescription {
        SingletonPartitionSchemeDescription { partition_scheme }
    }

    pub fn set_partition_scheme(
        &mut self,
        partition_scheme: ::models::PartitionScheme,
    ) {
        self.partition_scheme = partition_scheme;
    }

    pub fn with_partition_scheme(
        mut self,
        partition_scheme: ::models::PartitionScheme,
    ) -> SingletonPartitionSchemeDescription {
        self.partition_scheme = partition_scheme;
        self
    }

    pub fn partition_scheme(&self) -> &::models::PartitionScheme {
        &self.partition_scheme
    }
}
