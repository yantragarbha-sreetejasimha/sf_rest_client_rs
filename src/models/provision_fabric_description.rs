/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ProvisionFabricDescription : Describes the parameters for provisioning a cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionFabricDescription {
    /// The cluster code package file path.
    #[serde(rename = "CodeFilePath")]
    code_file_path: Option<String>,
    /// The cluster manifest file path.
    #[serde(rename = "ClusterManifestFilePath")]
    cluster_manifest_file_path: Option<String>,
}

impl Default for ProvisionFabricDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvisionFabricDescription {
    /// Describes the parameters for provisioning a cluster.
    pub fn new() -> ProvisionFabricDescription {
        ProvisionFabricDescription {
            code_file_path: None,
            cluster_manifest_file_path: None,
        }
    }

    pub fn set_code_file_path(&mut self, code_file_path: String) {
        self.code_file_path = Some(code_file_path);
    }

    pub fn with_code_file_path(
        mut self,
        code_file_path: String,
    ) -> ProvisionFabricDescription {
        self.code_file_path = Some(code_file_path);
        self
    }

    pub fn code_file_path(&self) -> Option<&String> {
        self.code_file_path.as_ref()
    }

    pub fn reset_code_file_path(&mut self) {
        self.code_file_path = None;
    }

    pub fn set_cluster_manifest_file_path(
        &mut self,
        cluster_manifest_file_path: String,
    ) {
        self.cluster_manifest_file_path = Some(cluster_manifest_file_path);
    }

    pub fn with_cluster_manifest_file_path(
        mut self,
        cluster_manifest_file_path: String,
    ) -> ProvisionFabricDescription {
        self.cluster_manifest_file_path = Some(cluster_manifest_file_path);
        self
    }

    pub fn cluster_manifest_file_path(&self) -> Option<&String> {
        self.cluster_manifest_file_path.as_ref()
    }

    pub fn reset_cluster_manifest_file_path(&mut self) {
        self.cluster_manifest_file_path = None;
    }
}
