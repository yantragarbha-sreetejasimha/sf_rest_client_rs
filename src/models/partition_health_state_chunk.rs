/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionHealthStateChunk : Represents the health state chunk of a partition, which contains the partition ID, its aggregated health state and any replicas that respect the filters in the cluster health chunk query description.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionHealthStateChunk {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The Id of the partition.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// The list of replica health state chunks belonging to the partition that respect the filters in the cluster health chunk query description.
    #[serde(rename = "ReplicaHealthStateChunks")]
    replica_health_state_chunks: Option<::models::ReplicaHealthStateChunkList>,
}

impl Default for PartitionHealthStateChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionHealthStateChunk {
    /// Represents the health state chunk of a partition, which contains the partition ID, its aggregated health state and any replicas that respect the filters in the cluster health chunk query description.
    pub fn new() -> PartitionHealthStateChunk {
        PartitionHealthStateChunk {
            health_state: None,
            partition_id: None,
            replica_health_state_chunks: None,
        }
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> PartitionHealthStateChunk {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> PartitionHealthStateChunk {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_replica_health_state_chunks(
        &mut self,
        replica_health_state_chunks: ::models::ReplicaHealthStateChunkList,
    ) {
        self.replica_health_state_chunks = Some(replica_health_state_chunks);
    }

    pub fn with_replica_health_state_chunks(
        mut self,
        replica_health_state_chunks: ::models::ReplicaHealthStateChunkList,
    ) -> PartitionHealthStateChunk {
        self.replica_health_state_chunks = Some(replica_health_state_chunks);
        self
    }

    pub fn replica_health_state_chunks(
        &self,
    ) -> Option<&::models::ReplicaHealthStateChunkList> {
        self.replica_health_state_chunks.as_ref()
    }

    pub fn reset_replica_health_state_chunks(&mut self) {
        self.replica_health_state_chunks = None;
    }
}
