/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LoadedPartitionInformationResult : Represents partition information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadedPartitionInformationResult {
    /// Name of the service this partition belongs to.
    #[serde(rename = "ServiceName")]
    service_name: String,
    /// Id of the partition.
    #[serde(rename = "PartitionId")]
    partition_id: ::models::PartitionId,
    /// Name of the metric for which this information is provided.
    #[serde(rename = "MetricName")]
    metric_name: String,
    /// Load for metric.
    #[serde(rename = "Load")]
    load: i64,
}

impl LoadedPartitionInformationResult {
    /// Represents partition information.
    pub fn new(
        service_name: String,
        partition_id: ::models::PartitionId,
        metric_name: String,
        load: i64,
    ) -> LoadedPartitionInformationResult {
        LoadedPartitionInformationResult {
            service_name,
            partition_id,
            metric_name,
            load,
        }
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = service_name;
    }

    pub fn with_service_name(
        mut self,
        service_name: String,
    ) -> LoadedPartitionInformationResult {
        self.service_name = service_name;
        self
    }

    pub fn service_name(&self) -> &String {
        &self.service_name
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = partition_id;
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> LoadedPartitionInformationResult {
        self.partition_id = partition_id;
        self
    }

    pub fn partition_id(&self) -> &::models::PartitionId {
        &self.partition_id
    }

    pub fn set_metric_name(&mut self, metric_name: String) {
        self.metric_name = metric_name;
    }

    pub fn with_metric_name(
        mut self,
        metric_name: String,
    ) -> LoadedPartitionInformationResult {
        self.metric_name = metric_name;
        self
    }

    pub fn metric_name(&self) -> &String {
        &self.metric_name
    }

    pub fn set_load(&mut self, load: i64) {
        self.load = load;
    }

    pub fn with_load(mut self, load: i64) -> LoadedPartitionInformationResult {
        self.load = load;
        self
    }

    pub fn load(&self) -> &i64 {
        &self.load
    }
}
