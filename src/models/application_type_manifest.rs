/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeManifest : Contains the manifest describing an application type registered in a Service Fabric cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeManifest {
    /// The XML manifest as a string.
    #[serde(rename = "Manifest")]
    manifest: Option<String>,
}

impl Default for ApplicationTypeManifest {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationTypeManifest {
    /// Contains the manifest describing an application type registered in a Service Fabric cluster.
    pub fn new() -> ApplicationTypeManifest {
        ApplicationTypeManifest { manifest: None }
    }

    pub fn set_manifest(&mut self, manifest: String) {
        self.manifest = Some(manifest);
    }

    pub fn with_manifest(
        mut self,
        manifest: String,
    ) -> ApplicationTypeManifest {
        self.manifest = Some(manifest);
        self
    }

    pub fn manifest(&self) -> Option<&String> {
        self.manifest.as_ref()
    }

    pub fn reset_manifest(&mut self) {
        self.manifest = None;
    }
}