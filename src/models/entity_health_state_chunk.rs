/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EntityHealthStateChunk : A base type for the health state chunk of various entities in the cluster. It contains the aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityHealthStateChunk {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
}

impl Default for EntityHealthStateChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityHealthStateChunk {
    /// A base type for the health state chunk of various entities in the cluster. It contains the aggregated health state.
    pub fn new() -> EntityHealthStateChunk {
        EntityHealthStateChunk { health_state: None }
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> EntityHealthStateChunk {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }
}
