/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosRemoveReplicaFaultCompletedEvent : Chaos Remove Replica Fault Completed event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosRemoveReplicaFaultCompletedEvent {
    /// The kind of FabricEvent.
    #[serde(rename = "Kind")]
    kind: ::models::FabricEventKind,
    /// The identifier for the FabricEvent instance.
    #[serde(rename = "EventInstanceId")]
    event_instance_id: String,
    /// The time event was logged.
    #[serde(rename = "TimeStamp")]
    time_stamp: String,
    /// Shows there is existing related events available.
    #[serde(rename = "HasCorrelatedEvents")]
    has_correlated_events: Option<bool>,
    /// An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different.
    #[serde(rename = "PartitionId")]
    partition_id: ::models::PartitionId,
    /// Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id.
    #[serde(rename = "ReplicaId")]
    replica_id: ::models::ReplicaIdInteger,
    /// Id of fault group.
    #[serde(rename = "FaultGroupId")]
    fault_group_id: String,
    /// Id of fault.
    #[serde(rename = "FaultId")]
    fault_id: String,
    /// Service name.
    #[serde(rename = "ServiceUri")]
    service_uri: String,
}

impl ChaosRemoveReplicaFaultCompletedEvent {
    /// Chaos Remove Replica Fault Completed event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        partition_id: ::models::PartitionId,
        replica_id: ::models::ReplicaIdInteger,
        fault_group_id: String,
        fault_id: String,
        service_uri: String,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        ChaosRemoveReplicaFaultCompletedEvent {
            kind,
            event_instance_id,
            time_stamp,
            has_correlated_events: None,
            partition_id,
            replica_id,
            fault_group_id,
            fault_id,
            service_uri,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::FabricEventKind {
        &self.kind
    }

    pub fn set_event_instance_id(&mut self, event_instance_id: String) {
        self.event_instance_id = event_instance_id;
    }

    pub fn with_event_instance_id(
        mut self,
        event_instance_id: String,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    pub fn with_time_stamp(
        mut self,
        time_stamp: String,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.time_stamp = time_stamp;
        self
    }

    pub fn time_stamp(&self) -> &String {
        &self.time_stamp
    }

    pub fn set_has_correlated_events(&mut self, has_correlated_events: bool) {
        self.has_correlated_events = Some(has_correlated_events);
    }

    pub fn with_has_correlated_events(
        mut self,
        has_correlated_events: bool,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = partition_id;
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.partition_id = partition_id;
        self
    }

    pub fn partition_id(&self) -> &::models::PartitionId {
        &self.partition_id
    }

    pub fn set_replica_id(&mut self, replica_id: ::models::ReplicaIdInteger) {
        self.replica_id = replica_id;
    }

    pub fn with_replica_id(
        mut self,
        replica_id: ::models::ReplicaIdInteger,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.replica_id = replica_id;
        self
    }

    pub fn replica_id(&self) -> &::models::ReplicaIdInteger {
        &self.replica_id
    }

    pub fn set_fault_group_id(&mut self, fault_group_id: String) {
        self.fault_group_id = fault_group_id;
    }

    pub fn with_fault_group_id(
        mut self,
        fault_group_id: String,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.fault_group_id = fault_group_id;
        self
    }

    pub fn fault_group_id(&self) -> &String {
        &self.fault_group_id
    }

    pub fn set_fault_id(&mut self, fault_id: String) {
        self.fault_id = fault_id;
    }

    pub fn with_fault_id(
        mut self,
        fault_id: String,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.fault_id = fault_id;
        self
    }

    pub fn fault_id(&self) -> &String {
        &self.fault_id
    }

    pub fn set_service_uri(&mut self, service_uri: String) {
        self.service_uri = service_uri;
    }

    pub fn with_service_uri(
        mut self,
        service_uri: String,
    ) -> ChaosRemoveReplicaFaultCompletedEvent {
        self.service_uri = service_uri;
        self
    }

    pub fn service_uri(&self) -> &String {
        &self.service_uri
    }
}
