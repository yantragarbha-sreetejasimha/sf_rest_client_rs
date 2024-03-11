/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterVersion : The cluster version.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterVersion {
    /// The Service Fabric cluster runtime version.
    #[serde(rename = "Version")]
    version: Option<String>,
}

impl Default for ClusterVersion {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterVersion {
    /// The cluster version.
    pub fn new() -> ClusterVersion {
        ClusterVersion { version: None }
    }

    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn with_version(mut self, version: String) -> ClusterVersion {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn reset_version(&mut self) {
        self.version = None;
    }
}
