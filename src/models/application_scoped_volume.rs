/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationScopedVolume : Describes a volume whose lifetime is scoped to the application's lifetime.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationScopedVolume {
    /// Name of the volume being referenced.
    #[serde(rename = "name")]
    name: String,
    /// The flag indicating whether the volume is read only. Default is 'false'.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// The path within the container at which the volume should be mounted. Only valid path characters are allowed.
    #[serde(rename = "destinationPath")]
    destination_path: String,
    /// Describes parameters for creating application-scoped volumes.
    #[serde(rename = "creationParameters")]
    creation_parameters: ::models::ApplicationScopedVolumeCreationParameters,
}

impl ApplicationScopedVolume {
    /// Describes a volume whose lifetime is scoped to the application's lifetime.
    pub fn new(
        name: String,
        destination_path: String,
        creation_parameters: ::models::ApplicationScopedVolumeCreationParameters,
    ) -> ApplicationScopedVolume {
        ApplicationScopedVolume {
            name,
            read_only: None,
            destination_path,
            creation_parameters,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> ApplicationScopedVolume {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = Some(read_only);
    }

    pub fn with_read_only(
        mut self,
        read_only: bool,
    ) -> ApplicationScopedVolume {
        self.read_only = Some(read_only);
        self
    }

    pub fn read_only(&self) -> Option<&bool> {
        self.read_only.as_ref()
    }

    pub fn reset_read_only(&mut self) {
        self.read_only = None;
    }

    pub fn set_destination_path(&mut self, destination_path: String) {
        self.destination_path = destination_path;
    }

    pub fn with_destination_path(
        mut self,
        destination_path: String,
    ) -> ApplicationScopedVolume {
        self.destination_path = destination_path;
        self
    }

    pub fn destination_path(&self) -> &String {
        &self.destination_path
    }

    pub fn set_creation_parameters(
        &mut self,
        creation_parameters: ::models::ApplicationScopedVolumeCreationParameters,
    ) {
        self.creation_parameters = creation_parameters;
    }

    pub fn with_creation_parameters(
        mut self,
        creation_parameters: ::models::ApplicationScopedVolumeCreationParameters,
    ) -> ApplicationScopedVolume {
        self.creation_parameters = creation_parameters;
        self
    }

    pub fn creation_parameters(
        &self,
    ) -> &::models::ApplicationScopedVolumeCreationParameters {
        &self.creation_parameters
    }
}
