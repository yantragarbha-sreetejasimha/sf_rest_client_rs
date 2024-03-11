/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeManifest : Contains the manifest describing a service type registered as part of an application in a Service Fabric cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeManifest {
    /// The XML manifest as a string.
    #[serde(rename = "Manifest")]
    manifest: Option<String>,
}

impl Default for ServiceTypeManifest {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeManifest {
    /// Contains the manifest describing a service type registered as part of an application in a Service Fabric cluster.
    pub fn new() -> ServiceTypeManifest {
        ServiceTypeManifest { manifest: None }
    }

    pub fn set_manifest(&mut self, manifest: String) {
        self.manifest = Some(manifest);
    }

    pub fn with_manifest(mut self, manifest: String) -> ServiceTypeManifest {
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
