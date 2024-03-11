/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateComposeDeploymentDescription : Defines description for creating a Service Fabric compose deployment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComposeDeploymentDescription {
    /// The name of the deployment.
    #[serde(rename = "DeploymentName")]
    deployment_name: ::models::DeploymentName,
    /// The content of the compose file that describes the deployment to create.
    #[serde(rename = "ComposeFileContent")]
    compose_file_content: String,
    /// Credential information to connect to container registry.
    #[serde(rename = "RegistryCredential")]
    registry_credential: Option<::models::RegistryCredential>,
}

impl CreateComposeDeploymentDescription {
    /// Defines description for creating a Service Fabric compose deployment.
    pub fn new(
        deployment_name: ::models::DeploymentName,
        compose_file_content: String,
    ) -> CreateComposeDeploymentDescription {
        CreateComposeDeploymentDescription {
            deployment_name,
            compose_file_content,
            registry_credential: None,
        }
    }

    pub fn set_deployment_name(
        &mut self,
        deployment_name: ::models::DeploymentName,
    ) {
        self.deployment_name = deployment_name;
    }

    pub fn with_deployment_name(
        mut self,
        deployment_name: ::models::DeploymentName,
    ) -> CreateComposeDeploymentDescription {
        self.deployment_name = deployment_name;
        self
    }

    pub fn deployment_name(&self) -> &::models::DeploymentName {
        &self.deployment_name
    }

    pub fn set_compose_file_content(&mut self, compose_file_content: String) {
        self.compose_file_content = compose_file_content;
    }

    pub fn with_compose_file_content(
        mut self,
        compose_file_content: String,
    ) -> CreateComposeDeploymentDescription {
        self.compose_file_content = compose_file_content;
        self
    }

    pub fn compose_file_content(&self) -> &String {
        &self.compose_file_content
    }

    pub fn set_registry_credential(
        &mut self,
        registry_credential: ::models::RegistryCredential,
    ) {
        self.registry_credential = Some(registry_credential);
    }

    pub fn with_registry_credential(
        mut self,
        registry_credential: ::models::RegistryCredential,
    ) -> CreateComposeDeploymentDescription {
        self.registry_credential = Some(registry_credential);
        self
    }

    pub fn registry_credential(&self) -> Option<&::models::RegistryCredential> {
        self.registry_credential.as_ref()
    }

    pub fn reset_registry_credential(&mut self) {
        self.registry_credential = None;
    }
}
