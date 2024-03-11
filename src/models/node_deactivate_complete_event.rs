/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeDeactivateCompleteEvent : Node Deactivate Complete event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDeactivateCompleteEvent {
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
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: ::models::NodeName,
    /// Id of Node instance.
    #[serde(rename = "NodeInstance")]
    node_instance: i64,
    /// Describes deactivate intent.
    #[serde(rename = "EffectiveDeactivateIntent")]
    effective_deactivate_intent: String,
    /// Batch Ids.
    #[serde(rename = "BatchIdsWithDeactivateIntent")]
    batch_ids_with_deactivate_intent: String,
    /// Start time.
    #[serde(rename = "StartTime")]
    start_time: String,
}

impl NodeDeactivateCompleteEvent {
    /// Node Deactivate Complete event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        node_name: ::models::NodeName,
        node_instance: i64,
        effective_deactivate_intent: String,
        batch_ids_with_deactivate_intent: String,
        start_time: String,
    ) -> NodeDeactivateCompleteEvent {
        NodeDeactivateCompleteEvent {
            kind,
            event_instance_id,
            time_stamp,
            has_correlated_events: None,
            node_name,
            node_instance,
            effective_deactivate_intent,
            batch_ids_with_deactivate_intent,
            start_time,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> NodeDeactivateCompleteEvent {
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
    ) -> NodeDeactivateCompleteEvent {
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
    ) -> NodeDeactivateCompleteEvent {
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
    ) -> NodeDeactivateCompleteEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = node_name;
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> NodeDeactivateCompleteEvent {
        self.node_name = node_name;
        self
    }

    pub fn node_name(&self) -> &::models::NodeName {
        &self.node_name
    }

    pub fn set_node_instance(&mut self, node_instance: i64) {
        self.node_instance = node_instance;
    }

    pub fn with_node_instance(
        mut self,
        node_instance: i64,
    ) -> NodeDeactivateCompleteEvent {
        self.node_instance = node_instance;
        self
    }

    pub fn node_instance(&self) -> &i64 {
        &self.node_instance
    }

    pub fn set_effective_deactivate_intent(
        &mut self,
        effective_deactivate_intent: String,
    ) {
        self.effective_deactivate_intent = effective_deactivate_intent;
    }

    pub fn with_effective_deactivate_intent(
        mut self,
        effective_deactivate_intent: String,
    ) -> NodeDeactivateCompleteEvent {
        self.effective_deactivate_intent = effective_deactivate_intent;
        self
    }

    pub fn effective_deactivate_intent(&self) -> &String {
        &self.effective_deactivate_intent
    }

    pub fn set_batch_ids_with_deactivate_intent(
        &mut self,
        batch_ids_with_deactivate_intent: String,
    ) {
        self.batch_ids_with_deactivate_intent =
            batch_ids_with_deactivate_intent;
    }

    pub fn with_batch_ids_with_deactivate_intent(
        mut self,
        batch_ids_with_deactivate_intent: String,
    ) -> NodeDeactivateCompleteEvent {
        self.batch_ids_with_deactivate_intent =
            batch_ids_with_deactivate_intent;
        self
    }

    pub fn batch_ids_with_deactivate_intent(&self) -> &String {
        &self.batch_ids_with_deactivate_intent
    }

    pub fn set_start_time(&mut self, start_time: String) {
        self.start_time = start_time;
    }

    pub fn with_start_time(
        mut self,
        start_time: String,
    ) -> NodeDeactivateCompleteEvent {
        self.start_time = start_time;
        self
    }

    pub fn start_time(&self) -> &String {
        &self.start_time
    }
}