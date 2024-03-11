/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeResult : Contains information about a node that was targeted by a user-induced operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResult {
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: Option<::models::NodeName>,
    /// The node instance id.
    #[serde(rename = "NodeInstanceId")]
    node_instance_id: Option<String>,
}

impl Default for NodeResult {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeResult {
    /// Contains information about a node that was targeted by a user-induced operation.
    pub fn new() -> NodeResult {
        NodeResult {
            node_name: None,
            node_instance_id: None,
        }
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> NodeResult {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&::models::NodeName> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
    }

    pub fn set_node_instance_id(&mut self, node_instance_id: String) {
        self.node_instance_id = Some(node_instance_id);
    }

    pub fn with_node_instance_id(
        mut self,
        node_instance_id: String,
    ) -> NodeResult {
        self.node_instance_id = Some(node_instance_id);
        self
    }

    pub fn node_instance_id(&self) -> Option<&String> {
        self.node_instance_id.as_ref()
    }

    pub fn reset_node_instance_id(&mut self) {
        self.node_instance_id = None;
    }
}
