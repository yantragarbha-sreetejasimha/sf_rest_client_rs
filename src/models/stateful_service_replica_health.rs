/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatefulServiceReplicaHealth : Represents the health of the stateful service replica. Contains the replica aggregated health state, the health events and the unhealthy evaluations.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulServiceReplicaHealth {
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "ServiceKind")]
    service_kind: Option<::models::ServiceKind>,
    /// Id of the partition to which this replica belongs.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// The HealthState representing the aggregated health state of the entity computed by Health Manager. The health evaluation of the entity reflects all events reported on the entity and its children (if any). The aggregation is done by applying the desired health policy.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// The list of health events reported on the entity.
    #[serde(rename = "HealthEvents")]
    health_events: Option<Vec<::models::HealthEvent>>,
    /// The unhealthy evaluations that show why the current aggregated health state was returned by Health Manager.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
    /// Shows the health statistics for all children types of the queried entity.
    #[serde(rename = "HealthStatistics")]
    health_statistics: Option<::models::HealthStatistics>,
    /// Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id.
    #[serde(rename = "ReplicaId")]
    replica_id: Option<::models::ReplicaId>,
}

impl Default for StatefulServiceReplicaHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulServiceReplicaHealth {
    /// Represents the health of the stateful service replica. Contains the replica aggregated health state, the health events and the unhealthy evaluations.
    pub fn new() -> StatefulServiceReplicaHealth {
        StatefulServiceReplicaHealth {
            service_kind: None,
            partition_id: None,
            aggregated_health_state: None,
            health_events: None,
            unhealthy_evaluations: None,
            health_statistics: None,
            replica_id: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = Some(service_kind);
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatefulServiceReplicaHealth {
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
    ) -> StatefulServiceReplicaHealth {
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
    ) -> StatefulServiceReplicaHealth {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_health_events(
        &mut self,
        health_events: Vec<::models::HealthEvent>,
    ) {
        self.health_events = Some(health_events);
    }

    pub fn with_health_events(
        mut self,
        health_events: Vec<::models::HealthEvent>,
    ) -> StatefulServiceReplicaHealth {
        self.health_events = Some(health_events);
        self
    }

    pub fn health_events(&self) -> Option<&Vec<::models::HealthEvent>> {
        self.health_events.as_ref()
    }

    pub fn reset_health_events(&mut self) {
        self.health_events = None;
    }

    pub fn set_unhealthy_evaluations(
        &mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
    }

    pub fn with_unhealthy_evaluations(
        mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) -> StatefulServiceReplicaHealth {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
        self
    }

    pub fn unhealthy_evaluations(
        &self,
    ) -> Option<&::models::UnhealthyEvaluations> {
        self.unhealthy_evaluations.as_ref()
    }

    pub fn reset_unhealthy_evaluations(&mut self) {
        self.unhealthy_evaluations = None;
    }

    pub fn set_health_statistics(
        &mut self,
        health_statistics: ::models::HealthStatistics,
    ) {
        self.health_statistics = Some(health_statistics);
    }

    pub fn with_health_statistics(
        mut self,
        health_statistics: ::models::HealthStatistics,
    ) -> StatefulServiceReplicaHealth {
        self.health_statistics = Some(health_statistics);
        self
    }

    pub fn health_statistics(&self) -> Option<&::models::HealthStatistics> {
        self.health_statistics.as_ref()
    }

    pub fn reset_health_statistics(&mut self) {
        self.health_statistics = None;
    }

    pub fn set_replica_id(&mut self, replica_id: ::models::ReplicaId) {
        self.replica_id = Some(replica_id);
    }

    pub fn with_replica_id(
        mut self,
        replica_id: ::models::ReplicaId,
    ) -> StatefulServiceReplicaHealth {
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
