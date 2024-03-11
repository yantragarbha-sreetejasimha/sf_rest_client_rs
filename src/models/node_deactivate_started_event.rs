/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeDeactivateStartedEvent : Node Deactivate Started event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDeactivateStartedEvent {
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: Option<::models::NodeName>,
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
    /// Id of Node instance.
    #[serde(rename = "NodeInstance")]
    node_instance: i64,
    /// Batch Id.
    #[serde(rename = "BatchId")]
    batch_id: String,
    /// Describes deactivate intent.
    #[serde(rename = "DeactivateIntent")]
    deactivate_intent: String,
}

impl NodeDeactivateStartedEvent {
    /// Node Deactivate Started event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        node_instance: i64,
        batch_id: String,
        deactivate_intent: String,
    ) -> NodeDeactivateStartedEvent {
        NodeDeactivateStartedEvent {
            node_name: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            node_instance,
            batch_id,
            deactivate_intent,
        }
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> NodeDeactivateStartedEvent {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&::models::NodeName> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> NodeDeactivateStartedEvent {
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
    ) -> NodeDeactivateStartedEvent {
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
    ) -> NodeDeactivateStartedEvent {
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
    ) -> NodeDeactivateStartedEvent {
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
    ) -> NodeDeactivateStartedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_node_instance(&mut self, node_instance: i64) {
        self.node_instance = node_instance;
    }

    pub fn with_node_instance(
        mut self,
        node_instance: i64,
    ) -> NodeDeactivateStartedEvent {
        self.node_instance = node_instance;
        self
    }

    pub fn node_instance(&self) -> &i64 {
        &self.node_instance
    }

    pub fn set_batch_id(&mut self, batch_id: String) {
        self.batch_id = batch_id;
    }

    pub fn with_batch_id(
        mut self,
        batch_id: String,
    ) -> NodeDeactivateStartedEvent {
        self.batch_id = batch_id;
        self
    }

    pub fn batch_id(&self) -> &String {
        &self.batch_id
    }

    pub fn set_deactivate_intent(&mut self, deactivate_intent: String) {
        self.deactivate_intent = deactivate_intent;
    }

    pub fn with_deactivate_intent(
        mut self,
        deactivate_intent: String,
    ) -> NodeDeactivateStartedEvent {
        self.deactivate_intent = deactivate_intent;
        self
    }

    pub fn deactivate_intent(&self) -> &String {
        &self.deactivate_intent
    }
}
