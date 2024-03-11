/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UnprovisionFabricDescription : Describes the parameters for unprovisioning a cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnprovisionFabricDescription {
    /// The cluster code package version.
    #[serde(rename = "CodeVersion")]
    code_version: Option<String>,
    /// The cluster manifest version.
    #[serde(rename = "ConfigVersion")]
    config_version: Option<String>,
}

impl Default for UnprovisionFabricDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl UnprovisionFabricDescription {
    /// Describes the parameters for unprovisioning a cluster.
    pub fn new() -> UnprovisionFabricDescription {
        UnprovisionFabricDescription {
            code_version: None,
            config_version: None,
        }
    }

    pub fn set_code_version(&mut self, code_version: String) {
        self.code_version = Some(code_version);
    }

    pub fn with_code_version(
        mut self,
        code_version: String,
    ) -> UnprovisionFabricDescription {
        self.code_version = Some(code_version);
        self
    }

    pub fn code_version(&self) -> Option<&String> {
        self.code_version.as_ref()
    }

    pub fn reset_code_version(&mut self) {
        self.code_version = None;
    }

    pub fn set_config_version(&mut self, config_version: String) {
        self.config_version = Some(config_version);
    }

    pub fn with_config_version(
        mut self,
        config_version: String,
    ) -> UnprovisionFabricDescription {
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
