/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeHealthState : Represents the health state of a node, which contains the node identifier and its aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHealthState {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// The name of a Service Fabric node.
    #[serde(rename = "Name")]
    name: Option<::models::NodeName>,
    /// An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name.
    #[serde(rename = "Id")]
    id: Option<::models::NodeId>,
}

impl Default for NodeHealthState {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeHealthState {
    /// Represents the health state of a node, which contains the node identifier and its aggregated health state.
    pub fn new() -> NodeHealthState {
        NodeHealthState {
            aggregated_health_state: None,
            name: None,
            id: None,
        }
    }

    pub fn set_aggregated_health_state(
        &mut self,
        aggregated_health_state: ::models::HealthState,
    ) {
        self.aggregated_health_state = Some(aggregated_health_state);
    }

    pub fn with_aggregated_health_state(
        mut self,
        aggregated_health_state: ::models::HealthState,
    ) -> NodeHealthState {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_name(&mut self, name: ::models::NodeName) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: ::models::NodeName) -> NodeHealthState {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::NodeName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_id(&mut self, id: ::models::NodeId) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: ::models::NodeId) -> NodeHealthState {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&::models::NodeId> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }
}
