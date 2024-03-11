/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NamedPartitionSchemeDescription : Describes the named partition scheme of the service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedPartitionSchemeDescription {
    /// Specifies how the service is partitioned.
    #[serde(rename = "PartitionScheme")]
    partition_scheme: ::models::PartitionScheme,
    /// The number of partitions.
    #[serde(rename = "Count")]
    count: i32,
    /// Array of size specified by the ‘Count’ parameter, for the names of the partitions.
    #[serde(rename = "Names")]
    names: Vec<String>,
}

impl NamedPartitionSchemeDescription {
    /// Describes the named partition scheme of the service.
    pub fn new(
        partition_scheme: ::models::PartitionScheme,
        count: i32,
        names: Vec<String>,
    ) -> NamedPartitionSchemeDescription {
        NamedPartitionSchemeDescription {
            partition_scheme,
            count,
            names,
        }
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
    ) -> NamedPartitionSchemeDescription {
        self.partition_scheme = partition_scheme;
        self
    }

    pub fn partition_scheme(&self) -> &::models::PartitionScheme {
        &self.partition_scheme
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    pub fn with_count(mut self, count: i32) -> NamedPartitionSchemeDescription {
        self.count = count;
        self
    }

    pub fn count(&self) -> &i32 {
        &self.count
    }

    pub fn set_names(&mut self, names: Vec<String>) {
        self.names = names;
    }

    pub fn with_names(
        mut self,
        names: Vec<String>,
    ) -> NamedPartitionSchemeDescription {
        self.names = names;
        self
    }

    pub fn names(&self) -> &Vec<String> {
        &self.names
    }
}