/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceHealthStateChunk : Represents the health state chunk of a service, which contains the service name, its aggregated health state and any partitions that respect the filters in the cluster health chunk query description.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceHealthStateChunk {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The name of the service whose health state chunk is provided in this object.
    #[serde(rename = "ServiceName")]
    service_name: Option<::models::ServiceName>,
    /// The list of partition health state chunks belonging to the service that respect the filters in the cluster health chunk query description.
    #[serde(rename = "PartitionHealthStateChunks")]
    partition_health_state_chunks:
        Option<::models::PartitionHealthStateChunkList>,
}

impl Default for ServiceHealthStateChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceHealthStateChunk {
    /// Represents the health state chunk of a service, which contains the service name, its aggregated health state and any partitions that respect the filters in the cluster health chunk query description.
    pub fn new() -> ServiceHealthStateChunk {
        ServiceHealthStateChunk {
            health_state: None,
            service_name: None,
            partition_health_state_chunks: None,
        }
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> ServiceHealthStateChunk {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_service_name(&mut self, service_name: ::models::ServiceName) {
        self.service_name = Some(service_name);
    }

    pub fn with_service_name(
        mut self,
        service_name: ::models::ServiceName,
    ) -> ServiceHealthStateChunk {
        self.service_name = Some(service_name);
        self
    }

    pub fn service_name(&self) -> Option<&::models::ServiceName> {
        self.service_name.as_ref()
    }

    pub fn reset_service_name(&mut self) {
        self.service_name = None;
    }

    pub fn set_partition_health_state_chunks(
        &mut self,
        partition_health_state_chunks: ::models::PartitionHealthStateChunkList,
    ) {
        self.partition_health_state_chunks =
            Some(partition_health_state_chunks);
    }

    pub fn with_partition_health_state_chunks(
        mut self,
        partition_health_state_chunks: ::models::PartitionHealthStateChunkList,
    ) -> ServiceHealthStateChunk {
        self.partition_health_state_chunks =
            Some(partition_health_state_chunks);
        self
    }

    pub fn partition_health_state_chunks(
        &self,
    ) -> Option<&::models::PartitionHealthStateChunkList> {
        self.partition_health_state_chunks.as_ref()
    }

    pub fn reset_partition_health_state_chunks(&mut self) {
        self.partition_health_state_chunks = None;
    }
}
