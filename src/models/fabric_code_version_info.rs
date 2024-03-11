/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FabricCodeVersionInfo : Information about a Service Fabric code version.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricCodeVersionInfo {
    /// The product version of Service Fabric.
    #[serde(rename = "CodeVersion")]
    code_version: Option<String>,
}

impl Default for FabricCodeVersionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricCodeVersionInfo {
    /// Information about a Service Fabric code version.
    pub fn new() -> FabricCodeVersionInfo {
        FabricCodeVersionInfo { code_version: None }
    }

    pub fn set_code_version(&mut self, code_version: String) {
        self.code_version = Some(code_version);
    }

    pub fn with_code_version(
        mut self,
        code_version: String,
    ) -> FabricCodeVersionInfo {
        self.code_version = Some(code_version);
        self
    }

    pub fn code_version(&self) -> Option<&String> {
        self.code_version.as_ref()
    }

    pub fn reset_code_version(&mut self) {
        self.code_version = None;
    }
}
