/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatefulServiceReplicaHealthState : Represents the health state of the stateful service replica, which contains the replica ID and the aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulServiceReplicaHealthState {
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "ServiceKind")]
    service_kind: Option<::models::ServiceKind>,
    /// The ID of the partition to which this replica belongs.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id.
    #[serde(rename = "ReplicaId")]
    replica_id: Option<::models::ReplicaId>,
}

impl Default for StatefulServiceReplicaHealthState {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulServiceReplicaHealthState {
    /// Represents the health state of the stateful service replica, which contains the replica ID and the aggregated health state.
    pub fn new() -> StatefulServiceReplicaHealthState {
        StatefulServiceReplicaHealthState {
            service_kind: None,
            partition_id: None,
            aggregated_health_state: None,
            replica_id: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = Some(service_kind);
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatefulServiceReplicaHealthState {
        self.service_kind = Some(service_kind);
        self
    }

    pub fn service_kind(&self) -> Option<&::models::ServiceKind> {
        self.service_kind.as_ref()
    }

    pub fn reset_service_kind(&mut self) {
        self.service_kind = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> StatefulServiceReplicaHealthState {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_aggregated_health_state(
        &mut self,
        aggregated_health_state: ::models::HealthState,
    ) {
        self.aggregated_health_state = Some(aggregated_health_state);
    }

    pub fn with_aggregated_health_state(
        mut self,
        aggregated_health_state: ::models::HealthState,
    ) -> StatefulServiceReplicaHealthState {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_replica_id(&mut self, replica_id: ::models::ReplicaId) {
        self.replica_id = Some(replica_id);
    }

    pub fn with_replica_id(
        mut self,
        replica_id: ::models::ReplicaId,
    ) -> StatefulServiceReplicaHealthState {
        self.replica_id = Some(replica_id);
        self
    }

    pub fn replica_id(&self) -> Option<&::models::ReplicaId> {
        self.replica_id.as_ref()
    }

    pub fn reset_replica_id(&mut self) {
        self.replica_id = None;
    }
}
