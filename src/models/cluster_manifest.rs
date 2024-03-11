/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterManifest : Information about the cluster manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterManifest {
    /// The contents of the cluster manifest file.
    #[serde(rename = "Manifest")]
    manifest: Option<String>,
}

impl Default for ClusterManifest {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterManifest {
    /// Information about the cluster manifest.
    pub fn new() -> ClusterManifest {
        ClusterManifest { manifest: None }
    }

    pub fn set_manifest(&mut self, manifest: String) {
        self.manifest = Some(manifest);
    }

    pub fn with_manifest(mut self, manifest: String) -> ClusterManifest {
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
