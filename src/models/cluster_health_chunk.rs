/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterHealthChunk : Represents the health chunk of the cluster. Contains the cluster aggregated health state, and the cluster entities that respect the input filter.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterHealthChunk {
    /// The HealthState representing the aggregated health state of the cluster computed by Health Manager. The health evaluation of the entity reflects all events reported on the entity and its children (if any). The aggregation is done by applying the desired cluster health policy and the application health policies.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The list of node health state chunks in the cluster that respect the filters in the cluster health chunk query description.
    #[serde(rename = "NodeHealthStateChunks")]
    node_health_state_chunks: Option<::models::NodeHealthStateChunkList>,
    /// The list of application health state chunks in the cluster that respect the filters in the cluster health chunk query description.
    #[serde(rename = "ApplicationHealthStateChunks")]
    application_health_state_chunks:
        Option<::models::ApplicationHealthStateChunkList>,
}

impl Default for ClusterHealthChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterHealthChunk {
    /// Represents the health chunk of the cluster. Contains the cluster aggregated health state, and the cluster entities that respect the input filter.
    pub fn new() -> ClusterHealthChunk {
        ClusterHealthChunk {
            health_state: None,
            node_health_state_chunks: None,
            application_health_state_chunks: None,
        }
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> ClusterHealthChunk {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_node_health_state_chunks(
        &mut self,
        node_health_state_chunks: ::models::NodeHealthStateChunkList,
    ) {
        self.node_health_state_chunks = Some(node_health_state_chunks);
    }

    pub fn with_node_health_state_chunks(
        mut self,
        node_health_state_chunks: ::models::NodeHealthStateChunkList,
    ) -> ClusterHealthChunk {
        self.node_health_state_chunks = Some(node_health_state_chunks);
        self
    }

    pub fn node_health_state_chunks(
        &self,
    ) -> Option<&::models::NodeHealthStateChunkList> {
        self.node_health_state_chunks.as_ref()
    }

    pub fn reset_node_health_state_chunks(&mut self) {
        self.node_health_state_chunks = None;
    }

    pub fn set_application_health_state_chunks(
        &mut self,
        application_health_state_chunks: ::models::ApplicationHealthStateChunkList,
    ) {
        self.application_health_state_chunks =
            Some(application_health_state_chunks);
    }

    pub fn with_application_health_state_chunks(
        mut self,
        application_health_state_chunks: ::models::ApplicationHealthStateChunkList,
    ) -> ClusterHealthChunk {
        self.application_health_state_chunks =
            Some(application_health_state_chunks);
        self
    }

    pub fn application_health_state_chunks(
        &self,
    ) -> Option<&::models::ApplicationHealthStateChunkList> {
        self.application_health_state_chunks.as_ref()
    }

    pub fn reset_application_health_state_chunks(&mut self) {
        self.application_health_state_chunks = None;
    }
}
