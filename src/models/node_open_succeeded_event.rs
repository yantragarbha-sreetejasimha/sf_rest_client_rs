/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeOpenSucceededEvent : Node Opened Succeeded event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeOpenSucceededEvent {
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
    /// Id of Node.
    #[serde(rename = "NodeId")]
    node_id: String,
    /// Upgrade domain of Node.
    #[serde(rename = "UpgradeDomain")]
    upgrade_domain: String,
    /// Fault domain of Node.
    #[serde(rename = "FaultDomain")]
    fault_domain: String,
    /// IP address or FQDN.
    #[serde(rename = "IpAddressOrFQDN")]
    ip_address_or_fqdn: String,
    /// Name of Host.
    #[serde(rename = "Hostname")]
    hostname: String,
    /// Indicates if it is seed node.
    #[serde(rename = "IsSeedNode")]
    is_seed_node: bool,
    /// Version of Node.
    #[serde(rename = "NodeVersion")]
    node_version: String,
}

impl NodeOpenSucceededEvent {
    /// Node Opened Succeeded event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        node_instance: i64,
        node_id: String,
        upgrade_domain: String,
        fault_domain: String,
        ip_address_or_fqdn: String,
        hostname: String,
        is_seed_node: bool,
        node_version: String,
    ) -> NodeOpenSucceededEvent {
        NodeOpenSucceededEvent {
            node_name: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            node_instance,
            node_id,
            upgrade_domain,
            fault_domain,
            ip_address_or_fqdn,
            hostname,
            is_seed_node,
            node_version,
        }
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> NodeOpenSucceededEvent {
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
    ) -> NodeOpenSucceededEvent {
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
    ) -> NodeOpenSucceededEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn with_category(mut self, category: String) -> NodeOpenSucceededEvent {
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
    ) -> NodeOpenSucceededEvent {
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
    ) -> NodeOpenSucceededEvent {
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
    ) -> NodeOpenSucceededEvent {
        self.node_instance = node_instance;
        self
    }

    pub fn node_instance(&self) -> &i64 {
        &self.node_instance
    }

    pub fn set_node_id(&mut self, node_id: String) {
        self.node_id = node_id;
    }

    pub fn with_node_id(mut self, node_id: String) -> NodeOpenSucceededEvent {
        self.node_id = node_id;
        self
    }

    pub fn node_id(&self) -> &String {
        &self.node_id
    }

    pub fn set_upgrade_domain(&mut self, upgrade_domain: String) {
        self.upgrade_domain = upgrade_domain;
    }

    pub fn with_upgrade_domain(
        mut self,
        upgrade_domain: String,
    ) -> NodeOpenSucceededEvent {
        self.upgrade_domain = upgrade_domain;
        self
    }

    pub fn upgrade_domain(&self) -> &String {
        &self.upgrade_domain
    }

    pub fn set_fault_domain(&mut self, fault_domain: String) {
        self.fault_domain = fault_domain;
    }

    pub fn with_fault_domain(
        mut self,
        fault_domain: String,
    ) -> NodeOpenSucceededEvent {
        self.fault_domain = fault_domain;
        self
    }

    pub fn fault_domain(&self) -> &String {
        &self.fault_domain
    }

    pub fn set_ip_address_or_fqdn(&mut self, ip_address_or_fqdn: String) {
        self.ip_address_or_fqdn = ip_address_or_fqdn;
    }

    pub fn with_ip_address_or_fqdn(
        mut self,
        ip_address_or_fqdn: String,
    ) -> NodeOpenSucceededEvent {
        self.ip_address_or_fqdn = ip_address_or_fqdn;
        self
    }

    pub fn ip_address_or_fqdn(&self) -> &String {
        &self.ip_address_or_fqdn
    }

    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = hostname;
    }

    pub fn with_hostname(mut self, hostname: String) -> NodeOpenSucceededEvent {
        self.hostname = hostname;
        self
    }

    pub fn hostname(&self) -> &String {
        &self.hostname
    }

    pub fn set_is_seed_node(&mut self, is_seed_node: bool) {
        self.is_seed_node = is_seed_node;
    }

    pub fn with_is_seed_node(
        mut self,
        is_seed_node: bool,
    ) -> NodeOpenSucceededEvent {
        self.is_seed_node = is_seed_node;
        self
    }

    pub fn is_seed_node(&self) -> &bool {
        &self.is_seed_node
    }

    pub fn set_node_version(&mut self, node_version: String) {
        self.node_version = node_version;
    }

    pub fn with_node_version(
        mut self,
        node_version: String,
    ) -> NodeOpenSucceededEvent {
        self.node_version = node_version;
        self
    }

    pub fn node_version(&self) -> &String {
        &self.node_version
    }
}
