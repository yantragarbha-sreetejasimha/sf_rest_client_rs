/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeHealthStateChunk : Represents the health state chunk of a node, which contains the node name and its aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHealthStateChunk {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: Option<::models::NodeName>,
}

impl Default for NodeHealthStateChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeHealthStateChunk {
    /// Represents the health state chunk of a node, which contains the node name and its aggregated health state.
    pub fn new() -> NodeHealthStateChunk {
        NodeHealthStateChunk {
            health_state: None,
            node_name: None,
        }
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> NodeHealthStateChunk {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> NodeHealthStateChunk {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&::models::NodeName> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
    }
}
