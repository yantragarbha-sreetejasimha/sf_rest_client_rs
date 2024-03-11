/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UniformInt64RangePartitionSchemeDescription : Describes a partitioning scheme where an integer range is allocated evenly across a number of partitions.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UniformInt64RangePartitionSchemeDescription {
    /// Specifies how the service is partitioned.
    #[serde(rename = "PartitionScheme")]
    partition_scheme: ::models::PartitionScheme,
    /// The number of partitions.
    #[serde(rename = "Count")]
    count: i32,
    /// String indicating the lower bound of the partition key range that should be split between the partitions.
    #[serde(rename = "LowKey")]
    low_key: String,
    /// String indicating the upper bound of the partition key range that should be split between the partitions.
    #[serde(rename = "HighKey")]
    high_key: String,
}

impl UniformInt64RangePartitionSchemeDescription {
    /// Describes a partitioning scheme where an integer range is allocated evenly across a number of partitions.
    pub fn new(
        partition_scheme: ::models::PartitionScheme,
        count: i32,
        low_key: String,
        high_key: String,
    ) -> UniformInt64RangePartitionSchemeDescription {
        UniformInt64RangePartitionSchemeDescription {
            partition_scheme,
            count,
            low_key,
            high_key,
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
    ) -> UniformInt64RangePartitionSchemeDescription {
        self.partition_scheme = partition_scheme;
        self
    }

    pub fn partition_scheme(&self) -> &::models::PartitionScheme {
        &self.partition_scheme
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    pub fn with_count(
        mut self,
        count: i32,
    ) -> UniformInt64RangePartitionSchemeDescription {
        self.count = count;
        self
    }

    pub fn count(&self) -> &i32 {
        &self.count
    }

    pub fn set_low_key(&mut self, low_key: String) {
        self.low_key = low_key;
    }

    pub fn with_low_key(
        mut self,
        low_key: String,
    ) -> UniformInt64RangePartitionSchemeDescription {
        self.low_key = low_key;
        self
    }

    pub fn low_key(&self) -> &String {
        &self.low_key
    }

    pub fn set_high_key(&mut self, high_key: String) {
        self.high_key = high_key;
    }

    pub fn with_high_key(
        mut self,
        high_key: String,
    ) -> UniformInt64RangePartitionSchemeDescription {
        self.high_key = high_key;
        self
    }

    pub fn high_key(&self) -> &String {
        &self.high_key
    }
}
