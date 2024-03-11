/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeInfo : Information about a node in Service Fabric cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// The name of a Service Fabric node.
    #[serde(rename = "Name")]
    name: Option<::models::NodeName>,
    /// The IP address or fully qualified domain name of the node.
    #[serde(rename = "IpAddressOrFQDN")]
    ip_address_or_fqdn: Option<String>,
    /// The type of the node.
    #[serde(rename = "Type")]
    _type: Option<String>,
    /// The version of Service Fabric binaries that the node is running.
    #[serde(rename = "CodeVersion")]
    code_version: Option<String>,
    /// The version of Service Fabric cluster manifest that the node is using.
    #[serde(rename = "ConfigVersion")]
    config_version: Option<String>,
    /// The status of the node.
    #[serde(rename = "NodeStatus")]
    node_status: Option<::models::NodeStatus>,
    /// Time in seconds since the node has been in NodeStatus Up. Value zero indicates that the node is not Up.
    #[serde(rename = "NodeUpTimeInSeconds")]
    node_up_time_in_seconds: Option<String>,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// Indicates if the node is a seed node or not. Returns true if the node is a seed node, otherwise false. A quorum of seed nodes are required for proper operation of Service Fabric cluster.
    #[serde(rename = "IsSeedNode")]
    is_seed_node: Option<bool>,
    /// The upgrade domain of the node.
    #[serde(rename = "UpgradeDomain")]
    upgrade_domain: Option<String>,
    /// The fault domain of the node.
    #[serde(rename = "FaultDomain")]
    fault_domain: Option<String>,
    /// An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name.
    #[serde(rename = "Id")]
    id: Option<::models::NodeId>,
    /// The ID representing the node instance. While the ID of the node is deterministically generated from the node name and remains same across restarts, the InstanceId changes every time node restarts.
    #[serde(rename = "InstanceId")]
    instance_id: Option<String>,
    /// Information about the node deactivation. This information is valid for a node that is undergoing deactivation or has already been deactivated.
    #[serde(rename = "NodeDeactivationInfo")]
    node_deactivation_info: Option<::models::NodeDeactivationInfo>,
    /// Indicates if the node is stopped by calling stop node API or not. Returns true if the node is stopped, otherwise false.
    #[serde(rename = "IsStopped")]
    is_stopped: Option<bool>,
    /// Time in seconds since the node has been in NodeStatus Down. Value zero indicates node is not NodeStatus Down.
    #[serde(rename = "NodeDownTimeInSeconds")]
    node_down_time_in_seconds: Option<String>,
    /// Date time in UTC when the node came up. If the node has never been up then this value will be zero date time.
    #[serde(rename = "NodeUpAt")]
    node_up_at: Option<String>,
    /// Date time in UTC when the node went down. If node has never been down then this value will be zero date time.
    #[serde(rename = "NodeDownAt")]
    node_down_at: Option<String>,
}

impl Default for NodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeInfo {
    /// Information about a node in Service Fabric cluster.
    pub fn new() -> NodeInfo {
        NodeInfo {
            name: None,
            ip_address_or_fqdn: None,
            _type: None,
            code_version: None,
            config_version: None,
            node_status: None,
            node_up_time_in_seconds: None,
            health_state: None,
            is_seed_node: None,
            upgrade_domain: None,
            fault_domain: None,
            id: None,
            instance_id: None,
            node_deactivation_info: None,
            is_stopped: None,
            node_down_time_in_seconds: None,
            node_up_at: None,
            node_down_at: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::NodeName) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: ::models::NodeName) -> NodeInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::NodeName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_ip_address_or_fqdn(&mut self, ip_address_or_fqdn: String) {
        self.ip_address_or_fqdn = Some(ip_address_or_fqdn);
    }

    pub fn with_ip_address_or_fqdn(
        mut self,
        ip_address_or_fqdn: String,
    ) -> NodeInfo {
        self.ip_address_or_fqdn = Some(ip_address_or_fqdn);
        self
    }

    pub fn ip_address_or_fqdn(&self) -> Option<&String> {
        self.ip_address_or_fqdn.as_ref()
    }

    pub fn reset_ip_address_or_fqdn(&mut self) {
        self.ip_address_or_fqdn = None;
    }

    pub fn set_type(&mut self, _type: String) {
        self._type = Some(_type);
    }

    pub fn with_type(mut self, _type: String) -> NodeInfo {
        self._type = Some(_type);
        self
    }

    pub fn _type(&self) -> Option<&String> {
        self._type.as_ref()
    }

    pub fn reset_type(&mut self) {
        self._type = None;
    }

    pub fn set_code_version(&mut self, code_version: String) {
        self.code_version = Some(code_version);
    }

    pub fn with_code_version(mut self, code_version: String) -> NodeInfo {
        self.code_version = Some(code_version);
        self
    }

    pub fn code_version(&self) -> Option<&String> {
        self.code_version.as_ref()
    }

    pub fn reset_code_version(&mut self) {
        self.code_version = None;
    }

    pub fn set_config_version(&mut self, config_version: String) {
        self.config_version = Some(config_version);
    }

    pub fn with_config_version(mut self, config_version: String) -> NodeInfo {
        self.config_version = Some(config_version);
        self
    }

    pub fn config_version(&self) -> Option<&String> {
        self.config_version.as_ref()
    }

    pub fn reset_config_version(&mut self) {
        self.config_version = None;
    }

    pub fn set_node_status(&mut self, node_status: ::models::NodeStatus) {
        self.node_status = Some(node_status);
    }

    pub fn with_node_status(
        mut self,
        node_status: ::models::NodeStatus,
    ) -> NodeInfo {
        self.node_status = Some(node_status);
        self
    }

    pub fn node_status(&self) -> Option<&::models::NodeStatus> {
        self.node_status.as_ref()
    }

    pub fn reset_node_status(&mut self) {
        self.node_status = None;
    }

    pub fn set_node_up_time_in_seconds(
        &mut self,
        node_up_time_in_seconds: String,
    ) {
        self.node_up_time_in_seconds = Some(node_up_time_in_seconds);
    }

    pub fn with_node_up_time_in_seconds(
        mut self,
        node_up_time_in_seconds: String,
    ) -> NodeInfo {
        self.node_up_time_in_seconds = Some(node_up_time_in_seconds);
        self
    }

    pub fn node_up_time_in_seconds(&self) -> Option<&String> {
        self.node_up_time_in_seconds.as_ref()
    }

    pub fn reset_node_up_time_in_seconds(&mut self) {
        self.node_up_time_in_seconds = None;
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> NodeInfo {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_is_seed_node(&mut self, is_seed_node: bool) {
        self.is_seed_node = Some(is_seed_node);
    }

    pub fn with_is_seed_node(mut self, is_seed_node: bool) -> NodeInfo {
        self.is_seed_node = Some(is_seed_node);
        self
    }

    pub fn is_seed_node(&self) -> Option<&bool> {
        self.is_seed_node.as_ref()
    }

    pub fn reset_is_seed_node(&mut self) {
        self.is_seed_node = None;
    }

    pub fn set_upgrade_domain(&mut self, upgrade_domain: String) {
        self.upgrade_domain = Some(upgrade_domain);
    }

    pub fn with_upgrade_domain(mut self, upgrade_domain: String) -> NodeInfo {
        self.upgrade_domain = Some(upgrade_domain);
        self
    }

    pub fn upgrade_domain(&self) -> Option<&String> {
        self.upgrade_domain.as_ref()
    }

    pub fn reset_upgrade_domain(&mut self) {
        self.upgrade_domain = None;
    }

    pub fn set_fault_domain(&mut self, fault_domain: String) {
        self.fault_domain = Some(fault_domain);
    }

    pub fn with_fault_domain(mut self, fault_domain: String) -> NodeInfo {
        self.fault_domain = Some(fault_domain);
        self
    }

    pub fn fault_domain(&self) -> Option<&String> {
        self.fault_domain.as_ref()
    }

    pub fn reset_fault_domain(&mut self) {
        self.fault_domain = None;
    }

    pub fn set_id(&mut self, id: ::models::NodeId) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: ::models::NodeId) -> NodeInfo {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&::models::NodeId> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_instance_id(&mut self, instance_id: String) {
        self.instance_id = Some(instance_id);
    }

    pub fn with_instance_id(mut self, instance_id: String) -> NodeInfo {
        self.instance_id = Some(instance_id);
        self
    }

    pub fn instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    pub fn reset_instance_id(&mut self) {
        self.instance_id = None;
    }

    pub fn set_node_deactivation_info(
        &mut self,
        node_deactivation_info: ::models::NodeDeactivationInfo,
    ) {
        self.node_deactivation_info = Some(node_deactivation_info);
    }

    pub fn with_node_deactivation_info(
        mut self,
        node_deactivation_info: ::models::NodeDeactivationInfo,
    ) -> NodeInfo {
        self.node_deactivation_info = Some(node_deactivation_info);
        self
    }

    pub fn node_deactivation_info(
        &self,
    ) -> Option<&::models::NodeDeactivationInfo> {
        self.node_deactivation_info.as_ref()
    }

    pub fn reset_node_deactivation_info(&mut self) {
        self.node_deactivation_info = None;
    }

    pub fn set_is_stopped(&mut self, is_stopped: bool) {
        self.is_stopped = Some(is_stopped);
    }

    pub fn with_is_stopped(mut self, is_stopped: bool) -> NodeInfo {
        self.is_stopped = Some(is_stopped);
        self
    }

    pub fn is_stopped(&self) -> Option<&bool> {
        self.is_stopped.as_ref()
    }

    pub fn reset_is_stopped(&mut self) {
        self.is_stopped = None;
    }

    pub fn set_node_down_time_in_seconds(
        &mut self,
        node_down_time_in_seconds: String,
    ) {
        self.node_down_time_in_seconds = Some(node_down_time_in_seconds);
    }

    pub fn with_node_down_time_in_seconds(
        mut self,
        node_down_time_in_seconds: String,
    ) -> NodeInfo {
        self.node_down_time_in_seconds = Some(node_down_time_in_seconds);
        self
    }

    pub fn node_down_time_in_seconds(&self) -> Option<&String> {
        self.node_down_time_in_seconds.as_ref()
    }

    pub fn reset_node_down_time_in_seconds(&mut self) {
        self.node_down_time_in_seconds = None;
    }

    pub fn set_node_up_at(&mut self, node_up_at: String) {
        self.node_up_at = Some(node_up_at);
    }

    pub fn with_node_up_at(mut self, node_up_at: String) -> NodeInfo {
        self.node_up_at = Some(node_up_at);
        self
    }

    pub fn node_up_at(&self) -> Option<&String> {
        self.node_up_at.as_ref()
    }

    pub fn reset_node_up_at(&mut self) {
        self.node_up_at = None;
    }

    pub fn set_node_down_at(&mut self, node_down_at: String) {
        self.node_down_at = Some(node_down_at);
    }

    pub fn with_node_down_at(mut self, node_down_at: String) -> NodeInfo {
        self.node_down_at = Some(node_down_at);
        self
    }

    pub fn node_down_at(&self) -> Option<&String> {
        self.node_down_at.as_ref()
    }

    pub fn reset_node_down_at(&mut self) {
        self.node_down_at = None;
    }
}
