/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosPartitionSecondaryMoveScheduledEvent : Chaos Move Secondary Fault Scheduled event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosPartitionSecondaryMoveScheduledEvent {
    /// An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// The kind of FabricEvent.
    #[serde(rename = "Kind")]
    kind: ::models::FabricEventKind,
    /// The identifier for the FabricEvent instance.
    #[serde(rename = "EventInstanceId")]
    event_instance_id: String,
    /// The category of event.
    #[serde(rename = "Category")]
    category: Option<String>,
    /// The time event was logged.
    #[serde(rename = "TimeStamp")]
    time_stamp: String,
    /// Shows there is existing related events available.
    #[serde(rename = "HasCorrelatedEvents")]
    has_correlated_events: Option<bool>,
    /// Id of fault group.
    #[serde(rename = "FaultGroupId")]
    fault_group_id: String,
    /// Id of fault.
    #[serde(rename = "FaultId")]
    fault_id: String,
    /// Service name.
    #[serde(rename = "ServiceName")]
    service_name: String,
    /// The name of a Service Fabric node.
    #[serde(rename = "SourceNode")]
    source_node: ::models::NodeName,
    /// The name of a Service Fabric node.
    #[serde(rename = "DestinationNode")]
    destination_node: ::models::NodeName,
    /// Indicates a forced move.
    #[serde(rename = "ForcedMove")]
    forced_move: bool,
}

impl ChaosPartitionSecondaryMoveScheduledEvent {
    /// Chaos Move Secondary Fault Scheduled event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        fault_group_id: String,
        fault_id: String,
        service_name: String,
        source_node: ::models::NodeName,
        destination_node: ::models::NodeName,
        forced_move: bool,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        ChaosPartitionSecondaryMoveScheduledEvent {
            partition_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            fault_group_id,
            fault_id,
            service_name,
            source_node,
            destination_node,
            forced_move,
        }
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
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
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn with_category(
        mut self,
        category: String,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.category = Some(category);
        self
    }

    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    pub fn reset_category(&mut self) {
        self.category = None;
    }

    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    pub fn with_time_stamp(
        mut self,
        time_stamp: String,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
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
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_fault_group_id(&mut self, fault_group_id: String) {
        self.fault_group_id = fault_group_id;
    }

    pub fn with_fault_group_id(
        mut self,
        fault_group_id: String,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
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
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.fault_id = fault_id;
        self
    }

    pub fn fault_id(&self) -> &String {
        &self.fault_id
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = service_name;
    }

    pub fn with_service_name(
        mut self,
        service_name: String,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.service_name = service_name;
        self
    }

    pub fn service_name(&self) -> &String {
        &self.service_name
    }

    pub fn set_source_node(&mut self, source_node: ::models::NodeName) {
        self.source_node = source_node;
    }

    pub fn with_source_node(
        mut self,
        source_node: ::models::NodeName,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.source_node = source_node;
        self
    }

    pub fn source_node(&self) -> &::models::NodeName {
        &self.source_node
    }

    pub fn set_destination_node(
        &mut self,
        destination_node: ::models::NodeName,
    ) {
        self.destination_node = destination_node;
    }

    pub fn with_destination_node(
        mut self,
        destination_node: ::models::NodeName,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.destination_node = destination_node;
        self
    }

    pub fn destination_node(&self) -> &::models::NodeName {
        &self.destination_node
    }

    pub fn set_forced_move(&mut self, forced_move: bool) {
        self.forced_move = forced_move;
    }

    pub fn with_forced_move(
        mut self,
        forced_move: bool,
    ) -> ChaosPartitionSecondaryMoveScheduledEvent {
        self.forced_move = forced_move;
        self
    }

    pub fn forced_move(&self) -> &bool {
        &self.forced_move
    }
}
