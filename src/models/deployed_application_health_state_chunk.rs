/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationHealthStateChunk : Represents the health state chunk of a deployed application, which contains the node where the application is deployed, the aggregated health state and any deployed service packages that respect the chunk query description filters.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationHealthStateChunk {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The name of node where the application is deployed.
    #[serde(rename = "NodeName")]
    node_name: Option<String>,
    /// The list of deployed service package health state chunks belonging to the deployed application that respect the filters in the cluster health chunk query description.
    #[serde(rename = "DeployedServicePackageHealthStateChunks")]
    deployed_service_package_health_state_chunks:
        Option<::models::DeployedServicePackageHealthStateChunkList>,
}

impl Default for DeployedApplicationHealthStateChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedApplicationHealthStateChunk {
    /// Represents the health state chunk of a deployed application, which contains the node where the application is deployed, the aggregated health state and any deployed service packages that respect the chunk query description filters.
    pub fn new() -> DeployedApplicationHealthStateChunk {
        DeployedApplicationHealthStateChunk {
            health_state: None,
            node_name: None,
            deployed_service_package_health_state_chunks: None,
        }
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> DeployedApplicationHealthStateChunk {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_node_name(&mut self, node_name: String) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: String,
    ) -> DeployedApplicationHealthStateChunk {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&String> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
    }

    pub fn set_deployed_service_package_health_state_chunks(
        &mut self,
        deployed_service_package_health_state_chunks: ::models::DeployedServicePackageHealthStateChunkList,
    ) {
        self.deployed_service_package_health_state_chunks =
            Some(deployed_service_package_health_state_chunks);
    }

    pub fn with_deployed_service_package_health_state_chunks(
        mut self,
        deployed_service_package_health_state_chunks: ::models::DeployedServicePackageHealthStateChunkList,
    ) -> DeployedApplicationHealthStateChunk {
        self.deployed_service_package_health_state_chunks =
            Some(deployed_service_package_health_state_chunks);
        self
    }

    pub fn deployed_service_package_health_state_chunks(
        &self,
    ) -> Option<&::models::DeployedServicePackageHealthStateChunkList> {
        self.deployed_service_package_health_state_chunks.as_ref()
    }

    pub fn reset_deployed_service_package_health_state_chunks(&mut self) {
        self.deployed_service_package_health_state_chunks = None;
    }
}
