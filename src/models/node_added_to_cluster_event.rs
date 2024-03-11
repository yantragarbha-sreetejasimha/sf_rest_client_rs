/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeAddedToClusterEvent : Node Added event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeAddedToClusterEvent {
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
    /// Id of Node.
    #[serde(rename = "NodeId")]
    node_id: String,
    /// Id of Node instance.
    #[serde(rename = "NodeInstance")]
    node_instance: i64,
    /// Type of Node.
    #[serde(rename = "NodeType")]
    node_type: String,
    /// Fabric version.
    #[serde(rename = "FabricVersion")]
    fabric_version: String,
    /// IP address or FQDN.
    #[serde(rename = "IpAddressOrFQDN")]
    ip_address_or_fqdn: String,
    /// Capacities.
    #[serde(rename = "NodeCapacities")]
    node_capacities: String,
}

impl NodeAddedToClusterEvent {
    /// Node Added event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        node_id: String,
        node_instance: i64,
        node_type: String,
        fabric_version: String,
        ip_address_or_fqdn: String,
        node_capacities: String,
    ) -> NodeAddedToClusterEvent {
        NodeAddedToClusterEvent {
            node_name: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            node_id,
            node_instance,
            node_type,
            fabric_version,
            ip_address_or_fqdn,
            node_capacities,
        }
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> NodeAddedToClusterEvent {
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
    ) -> NodeAddedToClusterEvent {
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
    ) -> NodeAddedToClusterEvent {
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
    ) -> NodeAddedToClusterEvent {
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
    ) -> NodeAddedToClusterEvent {
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
    ) -> NodeAddedToClusterEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_node_id(&mut self, node_id: String) {
        self.node_id = node_id;
    }

    pub fn with_node_id(mut self, node_id: String) -> NodeAddedToClusterEvent {
        self.node_id = node_id;
        self
    }

    pub fn node_id(&self) -> &String {
        &self.node_id
    }

    pub fn set_node_instance(&mut self, node_instance: i64) {
        self.node_instance = node_instance;
    }

    pub fn with_node_instance(
        mut self,
        node_instance: i64,
    ) -> NodeAddedToClusterEvent {
        self.node_instance = node_instance;
        self
    }

    pub fn node_instance(&self) -> &i64 {
        &self.node_instance
    }

    pub fn set_node_type(&mut self, node_type: String) {
        self.node_type = node_type;
    }

    pub fn with_node_type(
        mut self,
        node_type: String,
    ) -> NodeAddedToClusterEvent {
        self.node_type = node_type;
        self
    }

    pub fn node_type(&self) -> &String {
        &self.node_type
    }

    pub fn set_fabric_version(&mut self, fabric_version: String) {
        self.fabric_version = fabric_version;
    }

    pub fn with_fabric_version(
        mut self,
        fabric_version: String,
    ) -> NodeAddedToClusterEvent {
        self.fabric_version = fabric_version;
        self
    }

    pub fn fabric_version(&self) -> &String {
        &self.fabric_version
    }

    pub fn set_ip_address_or_fqdn(&mut self, ip_address_or_fqdn: String) {
        self.ip_address_or_fqdn = ip_address_or_fqdn;
    }

    pub fn with_ip_address_or_fqdn(
        mut self,
        ip_address_or_fqdn: String,
    ) -> NodeAddedToClusterEvent {
        self.ip_address_or_fqdn = ip_address_or_fqdn;
        self
    }

    pub fn ip_address_or_fqdn(&self) -> &String {
        &self.ip_address_or_fqdn
    }

    pub fn set_node_capacities(&mut self, node_capacities: String) {
        self.node_capacities = node_capacities;
    }

    pub fn with_node_capacities(
        mut self,
        node_capacities: String,
    ) -> NodeAddedToClusterEvent {
        self.node_capacities = node_capacities;
        self
    }

    pub fn node_capacities(&self) -> &String {
        &self.node_capacities
    }
}
