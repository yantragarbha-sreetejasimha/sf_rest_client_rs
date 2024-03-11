/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionPrimaryMoveAnalysisEvent : Partition Primary Move Analysis event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionPrimaryMoveAnalysisEvent {
    /// Metadata about an Analysis Event.
    #[serde(rename = "Metadata")]
    metadata: Option<::models::AnalysisEventMetadata>,
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
    /// Time when the move was completed.
    #[serde(rename = "WhenMoveCompleted")]
    when_move_completed: String,
    /// The name of a Service Fabric node.
    #[serde(rename = "PreviousNode")]
    previous_node: ::models::NodeName,
    /// The name of a Service Fabric node.
    #[serde(rename = "CurrentNode")]
    current_node: ::models::NodeName,
    /// Move reason.
    #[serde(rename = "MoveReason")]
    move_reason: String,
    /// Relevant traces.
    #[serde(rename = "RelevantTraces")]
    relevant_traces: String,
}

impl PartitionPrimaryMoveAnalysisEvent {
    /// Partition Primary Move Analysis event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        when_move_completed: String,
        previous_node: ::models::NodeName,
        current_node: ::models::NodeName,
        move_reason: String,
        relevant_traces: String,
    ) -> PartitionPrimaryMoveAnalysisEvent {
        PartitionPrimaryMoveAnalysisEvent {
            metadata: None,
            partition_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            when_move_completed,
            previous_node,
            current_node,
            move_reason,
            relevant_traces,
        }
    }

    pub fn set_metadata(&mut self, metadata: ::models::AnalysisEventMetadata) {
        self.metadata = Some(metadata);
    }

    pub fn with_metadata(
        mut self,
        metadata: ::models::AnalysisEventMetadata,
    ) -> PartitionPrimaryMoveAnalysisEvent {
        self.metadata = Some(metadata);
        self
    }

    pub fn metadata(&self) -> Option<&::models::AnalysisEventMetadata> {
        self.metadata.as_ref()
    }

    pub fn reset_metadata(&mut self) {
        self.metadata = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> PartitionPrimaryMoveAnalysisEvent {
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
    ) -> PartitionPrimaryMoveAnalysisEvent {
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
    ) -> PartitionPrimaryMoveAnalysisEvent {
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
    ) -> PartitionPrimaryMoveAnalysisEvent {
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
    ) -> PartitionPrimaryMoveAnalysisEvent {
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
    ) -> PartitionPrimaryMoveAnalysisEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_when_move_completed(&mut self, when_move_completed: String) {
        self.when_move_completed = when_move_completed;
    }

    pub fn with_when_move_completed(
        mut self,
        when_move_completed: String,
    ) -> PartitionPrimaryMoveAnalysisEvent {
        self.when_move_completed = when_move_completed;
        self
    }

    pub fn when_move_completed(&self) -> &String {
        &self.when_move_completed
    }

    pub fn set_previous_node(&mut self, previous_node: ::models::NodeName) {
        self.previous_node = previous_node;
    }

    pub fn with_previous_node(
        mut self,
        previous_node: ::models::NodeName,
    ) -> PartitionPrimaryMoveAnalysisEvent {
        self.previous_node = previous_node;
        self
    }

    pub fn previous_node(&self) -> &::models::NodeName {
        &self.previous_node
    }

    pub fn set_current_node(&mut self, current_node: ::models::NodeName) {
        self.current_node = current_node;
    }

    pub fn with_current_node(
        mut self,
        current_node: ::models::NodeName,
    ) -> PartitionPrimaryMoveAnalysisEvent {
        self.current_node = current_node;
        self
    }

    pub fn current_node(&self) -> &::models::NodeName {
        &self.current_node
    }

    pub fn set_move_reason(&mut self, move_reason: String) {
        self.move_reason = move_reason;
    }

    pub fn with_move_reason(
        mut self,
        move_reason: String,
    ) -> PartitionPrimaryMoveAnalysisEvent {
        self.move_reason = move_reason;
        self
    }

    pub fn move_reason(&self) -> &String {
        &self.move_reason
    }

    pub fn set_relevant_traces(&mut self, relevant_traces: String) {
        self.relevant_traces = relevant_traces;
    }

    pub fn with_relevant_traces(
        mut self,
        relevant_traces: String,
    ) -> PartitionPrimaryMoveAnalysisEvent {
        self.relevant_traces = relevant_traces;
        self
    }

    pub fn relevant_traces(&self) -> &String {
        &self.relevant_traces
    }
}
