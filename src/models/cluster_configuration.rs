/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterConfiguration : Information about the standalone cluster configuration.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfiguration {
    /// The contents of the cluster configuration file.
    #[serde(rename = "ClusterConfiguration")]
    cluster_configuration: Option<String>,
}

impl Default for ClusterConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterConfiguration {
    /// Information about the standalone cluster configuration.
    pub fn new() -> ClusterConfiguration {
        ClusterConfiguration {
            cluster_configuration: None,
        }
    }

    pub fn set_cluster_configuration(&mut self, cluster_configuration: String) {
        self.cluster_configuration = Some(cluster_configuration);
    }

    pub fn with_cluster_configuration(
        mut self,
        cluster_configuration: String,
    ) -> ClusterConfiguration {
        self.cluster_configuration = Some(cluster_configuration);
        self
    }

    pub fn cluster_configuration(&self) -> Option<&String> {
        self.cluster_configuration.as_ref()
    }

    pub fn reset_cluster_configuration(&mut self) {
        self.cluster_configuration = None;
    }
}
