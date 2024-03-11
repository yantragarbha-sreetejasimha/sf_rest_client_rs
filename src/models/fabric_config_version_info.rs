/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FabricConfigVersionInfo : Information about a Service Fabric config version.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricConfigVersionInfo {
    /// The config version of Service Fabric.
    #[serde(rename = "ConfigVersion")]
    config_version: Option<String>,
}

impl Default for FabricConfigVersionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricConfigVersionInfo {
    /// Information about a Service Fabric config version.
    pub fn new() -> FabricConfigVersionInfo {
        FabricConfigVersionInfo {
            config_version: None,
        }
    }

    pub fn set_config_version(&mut self, config_version: String) {
        self.config_version = Some(config_version);
    }

    pub fn with_config_version(
        mut self,
        config_version: String,
    ) -> FabricConfigVersionInfo {
        self.config_version = Some(config_version);
        self
    }

    pub fn config_version(&self) -> Option<&String> {
        self.config_version.as_ref()
    }

    pub fn reset_config_version(&mut self) {
        self.config_version = None;
    }
}
