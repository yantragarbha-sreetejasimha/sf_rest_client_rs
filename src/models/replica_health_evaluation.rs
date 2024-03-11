/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicaHealthEvaluation : Represents health evaluation for a replica, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state is either Error or Warning.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaHealthEvaluation {
    /// The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::HealthEvaluationKind,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Description of the health evaluation, which represents a summary of the evaluation process.
    #[serde(rename = "Description")]
    description: Option<String>,
    /// Id of the partition to which the replica belongs.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// Id of a stateful service replica or a stateless service instance. This ID is used in the queries that apply to both stateful and stateless services. It is used by Service Fabric to uniquely identify a replica of a partition of a stateful service or an instance of a stateless service partition. It is unique within a partition and does not change for the lifetime of the replica or the instance. If a stateful replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the ID. If a stateless instance is failed over on the same or different node it will get a different value for the ID.
    #[serde(rename = "ReplicaOrInstanceId")]
    replica_or_instance_id: Option<::models::ReplicaOrInstanceId>,
    /// List of unhealthy evaluations that led to the current aggregated health state of the replica. The types of the unhealthy evaluations can be EventHealthEvaluation.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
}

impl ReplicaHealthEvaluation {
    /// Represents health evaluation for a replica, containing information about the data and the algorithm used by health store to evaluate health. The evaluation is returned only when the aggregated health state is either Error or Warning.
    pub fn new(
        kind: ::models::HealthEvaluationKind,
    ) -> ReplicaHealthEvaluation {
        ReplicaHealthEvaluation {
            kind,
            aggregated_health_state: None,
            description: None,
            partition_id: None,
            replica_or_instance_id: None,
            unhealthy_evaluations: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::HealthEvaluationKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::HealthEvaluationKind,
    ) -> ReplicaHealthEvaluation {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::HealthEvaluationKind {
        &self.kind
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
    ) -> ReplicaHealthEvaluation {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> ReplicaHealthEvaluation {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> ReplicaHealthEvaluation {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_replica_or_instance_id(
        &mut self,
        replica_or_instance_id: ::models::ReplicaOrInstanceId,
    ) {
        self.replica_or_instance_id = Some(replica_or_instance_id);
    }

    pub fn with_replica_or_instance_id(
        mut self,
        replica_or_instance_id: ::models::ReplicaOrInstanceId,
    ) -> ReplicaHealthEvaluation {
        self.replica_or_instance_id = Some(replica_or_instance_id);
        self
    }

    pub fn replica_or_instance_id(
        &self,
    ) -> Option<&::models::ReplicaOrInstanceId> {
        self.replica_or_instance_id.as_ref()
    }

    pub fn reset_replica_or_instance_id(&mut self) {
        self.replica_or_instance_id = None;
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
    ) -> ReplicaHealthEvaluation {
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
}
