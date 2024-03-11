/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ComposeDeploymentStatusInfo : Information about a Service Fabric compose deployment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComposeDeploymentStatusInfo {
    /// The name of the deployment.
    #[serde(rename = "Name")]
    name: Option<::models::DeploymentName>,
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "ApplicationName")]
    application_name: Option<::models::ApplicationName>,
    /// The status of the compose deployment.
    #[serde(rename = "Status")]
    status: Option<::models::ComposeDeploymentStatus>,
    /// The status details of compose deployment including failure message.
    #[serde(rename = "StatusDetails")]
    status_details: Option<String>,
}

impl Default for ComposeDeploymentStatusInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ComposeDeploymentStatusInfo {
    /// Information about a Service Fabric compose deployment.
    pub fn new() -> ComposeDeploymentStatusInfo {
        ComposeDeploymentStatusInfo {
            name: None,
            application_name: None,
            status: None,
            status_details: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::DeploymentName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::DeploymentName,
    ) -> ComposeDeploymentStatusInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::DeploymentName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_application_name(
        &mut self,
        application_name: ::models::ApplicationName,
    ) {
        self.application_name = Some(application_name);
    }

    pub fn with_application_name(
        mut self,
        application_name: ::models::ApplicationName,
    ) -> ComposeDeploymentStatusInfo {
        self.application_name = Some(application_name);
        self
    }

    pub fn application_name(&self) -> Option<&::models::ApplicationName> {
        self.application_name.as_ref()
    }

    pub fn reset_application_name(&mut self) {
        self.application_name = None;
    }

    pub fn set_status(&mut self, status: ::models::ComposeDeploymentStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ComposeDeploymentStatus,
    ) -> ComposeDeploymentStatusInfo {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::ComposeDeploymentStatus> {
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
    ) -> ComposeDeploymentStatusInfo {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }
}
