/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeDownEvent : Node Down event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDownEvent {
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
    /// Time when Node was last up.
    #[serde(rename = "LastNodeUpAt")]
    last_node_up_at: String,
}

impl NodeDownEvent {
    /// Node Down event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        node_instance: i64,
        last_node_up_at: String,
    ) -> NodeDownEvent {
        NodeDownEvent {
            node_name: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            node_instance,
            last_node_up_at,
        }
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> NodeDownEvent {
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
    ) -> NodeDownEvent {
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
    ) -> NodeDownEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn with_category(mut self, category: String) -> NodeDownEvent {
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

    pub fn with_time_stamp(mut self, time_stamp: String) -> NodeDownEvent {
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
    ) -> NodeDownEvent {
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

    pub fn with_node_instance(mut self, node_instance: i64) -> NodeDownEvent {
        self.node_instance = node_instance;
        self
    }

    pub fn node_instance(&self) -> &i64 {
        &self.node_instance
    }

    pub fn set_last_node_up_at(&mut self, last_node_up_at: String) {
        self.last_node_up_at = last_node_up_at;
    }

    pub fn with_last_node_up_at(
        mut self,
        last_node_up_at: String,
    ) -> NodeDownEvent {
        self.last_node_up_at = last_node_up_at;
        self
    }

    pub fn last_node_up_at(&self) -> &String {
        &self.last_node_up_at
    }
}
