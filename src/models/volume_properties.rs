/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// VolumeProperties : Describes properties of a volume resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeProperties {
    /// User readable description of the volume.
    #[serde(rename = "description")]
    description: Option<String>,
    /// Status of the volume.
    #[serde(rename = "status")]
    status: Option<::models::ResourceStatus>,
    /// Gives additional information about the current status of the volume.
    #[serde(rename = "statusDetails")]
    status_details: Option<String>,
    /// Provider of the volume.
    #[serde(rename = "provider")]
    provider: ::models::VolumeProvider,
    /// This type describes a volume provided by an Azure Files file share.
    #[serde(rename = "azureFileParameters")]
    azure_file_parameters: Option<::models::VolumeProviderParametersAzureFile>,
}

impl VolumeProperties {
    /// Describes properties of a volume resource.
    pub fn new(provider: ::models::VolumeProvider) -> VolumeProperties {
        VolumeProperties {
            description: None,
            status: None,
            status_details: None,
            provider,
            azure_file_parameters: None,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(mut self, description: String) -> VolumeProperties {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_status(&mut self, status: ::models::ResourceStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ResourceStatus,
    ) -> VolumeProperties {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::ResourceStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_status_details(&mut self, status_details: String) {
        self.status_details = Some(status_details);
    }

    pub fn with_status_details(
        mut self,
        status_details: String,
    ) -> VolumeProperties {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }

    pub fn set_provider(&mut self, provider: ::models::VolumeProvider) {
        self.provider = provider;
    }

    pub fn with_provider(
        mut self,
        provider: ::models::VolumeProvider,
    ) -> VolumeProperties {
        self.provider = provider;
        self
    }

    pub fn provider(&self) -> &::models::VolumeProvider {
        &self.provider
    }

    pub fn set_azure_file_parameters(
        &mut self,
        azure_file_parameters: ::models::VolumeProviderParametersAzureFile,
    ) {
        self.azure_file_parameters = Some(azure_file_parameters);
    }

    pub fn with_azure_file_parameters(
        mut self,
        azure_file_parameters: ::models::VolumeProviderParametersAzureFile,
    ) -> VolumeProperties {
        self.azure_file_parameters = Some(azure_file_parameters);
        self
    }

    pub fn azure_file_parameters(
        &self,
    ) -> Option<&::models::VolumeProviderParametersAzureFile> {
        self.azure_file_parameters.as_ref()
    }

    pub fn reset_azure_file_parameters(&mut self) {
        self.azure_file_parameters = None;
    }
}