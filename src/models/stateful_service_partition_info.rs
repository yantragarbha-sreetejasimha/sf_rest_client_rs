/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatefulServicePartitionInfo : Information about a partition of a stateful Service Fabric service..

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulServicePartitionInfo {
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The status of the service fabric service partition.
    #[serde(rename = "PartitionStatus")]
    partition_status: Option<::models::ServicePartitionStatus>,
    /// Information about the partition identity, partitioning scheme and keys supported by it.
    #[serde(rename = "PartitionInformation")]
    partition_information: Option<::models::PartitionInformation>,
    /// The target replica set size as a number.
    #[serde(rename = "TargetReplicaSetSize")]
    target_replica_set_size: Option<i64>,
    /// The minimum replica set size as a number.
    #[serde(rename = "MinReplicaSetSize")]
    min_replica_set_size: Option<i64>,
    /// The auxiliary replica count as a number. To use Auxiliary replicas the following must be true, AuxiliaryReplicaCount < (TargetReplicaSetSize+1)/2 and TargetReplicaSetSize >=3.
    #[serde(rename = "AuxiliaryReplicaCount")]
    auxiliary_replica_count: Option<i64>,
    /// The duration for which this partition was in quorum loss. If the partition is currently in quorum loss, it returns the duration since it has been in that state. This field is using ISO8601 format for specifying the duration.
    #[serde(rename = "LastQuorumLossDuration")]
    last_quorum_loss_duration: Option<String>,
    /// An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica.
    #[serde(rename = "PrimaryEpoch")]
    primary_epoch: Option<::models::Epoch>,
}

impl StatefulServicePartitionInfo {
    /// Information about a partition of a stateful Service Fabric service..
    pub fn new(
        service_kind: ::models::ServiceKind,
    ) -> StatefulServicePartitionInfo {
        StatefulServicePartitionInfo {
            service_kind,
            health_state: None,
            partition_status: None,
            partition_information: None,
            target_replica_set_size: None,
            min_replica_set_size: None,
            auxiliary_replica_count: None,
            last_quorum_loss_duration: None,
            primary_epoch: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatefulServicePartitionInfo {
        self.service_kind = service_kind;
        self
    }

    pub fn service_kind(&self) -> &::models::ServiceKind {
        &self.service_kind
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> StatefulServicePartitionInfo {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_partition_status(
        &mut self,
        partition_status: ::models::ServicePartitionStatus,
    ) {
        self.partition_status = Some(partition_status);
    }

    pub fn with_partition_status(
        mut self,
        partition_status: ::models::ServicePartitionStatus,
    ) -> StatefulServicePartitionInfo {
        self.partition_status = Some(partition_status);
        self
    }

    pub fn partition_status(
        &self,
    ) -> Option<&::models::ServicePartitionStatus> {
        self.partition_status.as_ref()
    }

    pub fn reset_partition_status(&mut self) {
        self.partition_status = None;
    }

    pub fn set_partition_information(
        &mut self,
        partition_information: ::models::PartitionInformation,
    ) {
        self.partition_information = Some(partition_information);
    }

    pub fn with_partition_information(
        mut self,
        partition_information: ::models::PartitionInformation,
    ) -> StatefulServicePartitionInfo {
        self.partition_information = Some(partition_information);
        self
    }

    pub fn partition_information(
        &self,
    ) -> Option<&::models::PartitionInformation> {
        self.partition_information.as_ref()
    }

    pub fn reset_partition_information(&mut self) {
        self.partition_information = None;
    }

    pub fn set_target_replica_set_size(
        &mut self,
        target_replica_set_size: i64,
    ) {
        self.target_replica_set_size = Some(target_replica_set_size);
    }

    pub fn with_target_replica_set_size(
        mut self,
        target_replica_set_size: i64,
    ) -> StatefulServicePartitionInfo {
        self.target_replica_set_size = Some(target_replica_set_size);
        self
    }

    pub fn target_replica_set_size(&self) -> Option<&i64> {
        self.target_replica_set_size.as_ref()
    }

    pub fn reset_target_replica_set_size(&mut self) {
        self.target_replica_set_size = None;
    }

    pub fn set_min_replica_set_size(&mut self, min_replica_set_size: i64) {
        self.min_replica_set_size = Some(min_replica_set_size);
    }

    pub fn with_min_replica_set_size(
        mut self,
        min_replica_set_size: i64,
    ) -> StatefulServicePartitionInfo {
        self.min_replica_set_size = Some(min_replica_set_size);
        self
    }

    pub fn min_replica_set_size(&self) -> Option<&i64> {
        self.min_replica_set_size.as_ref()
    }

    pub fn reset_min_replica_set_size(&mut self) {
        self.min_replica_set_size = None;
    }

    pub fn set_auxiliary_replica_count(
        &mut self,
        auxiliary_replica_count: i64,
    ) {
        self.auxiliary_replica_count = Some(auxiliary_replica_count);
    }

    pub fn with_auxiliary_replica_count(
        mut self,
        auxiliary_replica_count: i64,
    ) -> StatefulServicePartitionInfo {
        self.auxiliary_replica_count = Some(auxiliary_replica_count);
        self
    }

    pub fn auxiliary_replica_count(&self) -> Option<&i64> {
        self.auxiliary_replica_count.as_ref()
    }

    pub fn reset_auxiliary_replica_count(&mut self) {
        self.auxiliary_replica_count = None;
    }

    pub fn set_last_quorum_loss_duration(
        &mut self,
        last_quorum_loss_duration: String,
    ) {
        self.last_quorum_loss_duration = Some(last_quorum_loss_duration);
    }

    pub fn with_last_quorum_loss_duration(
        mut self,
        last_quorum_loss_duration: String,
    ) -> StatefulServicePartitionInfo {
        self.last_quorum_loss_duration = Some(last_quorum_loss_duration);
        self
    }

    pub fn last_quorum_loss_duration(&self) -> Option<&String> {
        self.last_quorum_loss_duration.as_ref()
    }

    pub fn reset_last_quorum_loss_duration(&mut self) {
        self.last_quorum_loss_duration = None;
    }

    pub fn set_primary_epoch(&mut self, primary_epoch: ::models::Epoch) {
        self.primary_epoch = Some(primary_epoch);
    }

    pub fn with_primary_epoch(
        mut self,
        primary_epoch: ::models::Epoch,
    ) -> StatefulServicePartitionInfo {
        self.primary_epoch = Some(primary_epoch);
        self
    }

    pub fn primary_epoch(&self) -> Option<&::models::Epoch> {
        self.primary_epoch.as_ref()
    }

    pub fn reset_primary_epoch(&mut self) {
        self.primary_epoch = None;
    }
}
