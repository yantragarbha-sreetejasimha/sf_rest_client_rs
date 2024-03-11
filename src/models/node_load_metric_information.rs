/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeLoadMetricInformation : Represents data structure that contains load information for a certain metric on a node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeLoadMetricInformation {
    /// Name of the metric for which this load information is provided.
    #[serde(rename = "Name")]
    name: Option<String>,
    /// Total capacity on the node for this metric.
    #[serde(rename = "NodeCapacity")]
    node_capacity: Option<String>,
    /// Current load on the node for this metric.
    #[serde(rename = "NodeLoad")]
    node_load: Option<String>,
    /// The remaining capacity on the node for this metric.
    #[serde(rename = "NodeRemainingCapacity")]
    node_remaining_capacity: Option<String>,
    /// Indicates if there is a capacity violation for this metric on the node.
    #[serde(rename = "IsCapacityViolation")]
    is_capacity_violation: Option<bool>,
    /// The value that indicates the reserved capacity for this metric on the node.
    #[serde(rename = "NodeBufferedCapacity")]
    node_buffered_capacity: Option<String>,
    /// The remaining reserved capacity for this metric on the node.
    #[serde(rename = "NodeRemainingBufferedCapacity")]
    node_remaining_buffered_capacity: Option<String>,
}

impl Default for NodeLoadMetricInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeLoadMetricInformation {
    /// Represents data structure that contains load information for a certain metric on a node.
    pub fn new() -> NodeLoadMetricInformation {
        NodeLoadMetricInformation {
            name: None,
            node_capacity: None,
            node_load: None,
            node_remaining_capacity: None,
            is_capacity_violation: None,
            node_buffered_capacity: None,
            node_remaining_buffered_capacity: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> NodeLoadMetricInformation {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_node_capacity(&mut self, node_capacity: String) {
        self.node_capacity = Some(node_capacity);
    }

    pub fn with_node_capacity(
        mut self,
        node_capacity: String,
    ) -> NodeLoadMetricInformation {
        self.node_capacity = Some(node_capacity);
        self
    }

    pub fn node_capacity(&self) -> Option<&String> {
        self.node_capacity.as_ref()
    }

    pub fn reset_node_capacity(&mut self) {
        self.node_capacity = None;
    }

    pub fn set_node_load(&mut self, node_load: String) {
        self.node_load = Some(node_load);
    }

    pub fn with_node_load(
        mut self,
        node_load: String,
    ) -> NodeLoadMetricInformation {
        self.node_load = Some(node_load);
        self
    }

    pub fn node_load(&self) -> Option<&String> {
        self.node_load.as_ref()
    }

    pub fn reset_node_load(&mut self) {
        self.node_load = None;
    }

    pub fn set_node_remaining_capacity(
        &mut self,
        node_remaining_capacity: String,
    ) {
        self.node_remaining_capacity = Some(node_remaining_capacity);
    }

    pub fn with_node_remaining_capacity(
        mut self,
        node_remaining_capacity: String,
    ) -> NodeLoadMetricInformation {
        self.node_remaining_capacity = Some(node_remaining_capacity);
        self
    }

    pub fn node_remaining_capacity(&self) -> Option<&String> {
        self.node_remaining_capacity.as_ref()
    }

    pub fn reset_node_remaining_capacity(&mut self) {
        self.node_remaining_capacity = None;
    }

    pub fn set_is_capacity_violation(&mut self, is_capacity_violation: bool) {
        self.is_capacity_violation = Some(is_capacity_violation);
    }

    pub fn with_is_capacity_violation(
        mut self,
        is_capacity_violation: bool,
    ) -> NodeLoadMetricInformation {
        self.is_capacity_violation = Some(is_capacity_violation);
        self
    }

    pub fn is_capacity_violation(&self) -> Option<&bool> {
        self.is_capacity_violation.as_ref()
    }

    pub fn reset_is_capacity_violation(&mut self) {
        self.is_capacity_violation = None;
    }

    pub fn set_node_buffered_capacity(
        &mut self,
        node_buffered_capacity: String,
    ) {
        self.node_buffered_capacity = Some(node_buffered_capacity);
    }

    pub fn with_node_buffered_capacity(
        mut self,
        node_buffered_capacity: String,
    ) -> NodeLoadMetricInformation {
        self.node_buffered_capacity = Some(node_buffered_capacity);
        self
    }

    pub fn node_buffered_capacity(&self) -> Option<&String> {
        self.node_buffered_capacity.as_ref()
    }

    pub fn reset_node_buffered_capacity(&mut self) {
        self.node_buffered_capacity = None;
    }

    pub fn set_node_remaining_buffered_capacity(
        &mut self,
        node_remaining_buffered_capacity: String,
    ) {
        self.node_remaining_buffered_capacity =
            Some(node_remaining_buffered_capacity);
    }

    pub fn with_node_remaining_buffered_capacity(
        mut self,
        node_remaining_buffered_capacity: String,
    ) -> NodeLoadMetricInformation {
        self.node_remaining_buffered_capacity =
            Some(node_remaining_buffered_capacity);
        self
    }

    pub fn node_remaining_buffered_capacity(&self) -> Option<&String> {
        self.node_remaining_buffered_capacity.as_ref()
    }

    pub fn reset_node_remaining_buffered_capacity(&mut self) {
        self.node_remaining_buffered_capacity = None;
    }
}
