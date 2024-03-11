/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk : Describes parameters for creating application-scoped volumes provided by Service Fabric Volume Disks

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
    /// Specifies the application-scoped volume kind.
    #[serde(rename = "kind")]
    kind: ::models::ApplicationScopedVolumeKind,
    /// User readable description of the volume.
    #[serde(rename = "description")]
    description: Option<String>,
    /// Volume size
    #[serde(rename = "sizeDisk")]
    size_disk: String,
}

impl ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
    /// Describes parameters for creating application-scoped volumes provided by Service Fabric Volume Disks
    pub fn new(
        kind: ::models::ApplicationScopedVolumeKind,
        size_disk: String,
    ) -> ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
        ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
            kind,
            description: None,
            size_disk,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ApplicationScopedVolumeKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ApplicationScopedVolumeKind,
    ) -> ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ApplicationScopedVolumeKind {
        &self.kind
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_size_disk(&mut self, size_disk: String) {
        self.size_disk = size_disk;
    }

    pub fn with_size_disk(
        mut self,
        size_disk: String,
    ) -> ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
        self.size_disk = size_disk;
        self
    }

    pub fn size_disk(&self) -> &String {
        &self.size_disk
    }
}
