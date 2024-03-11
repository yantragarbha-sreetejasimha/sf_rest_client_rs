/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationProperties : This type describes properties of an application resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationProperties {
    /// User readable description of the application.
    #[serde(rename = "description")]
    description: Option<String>,
    /// Internal use.
    #[serde(rename = "debugParams")]
    debug_params: Option<String>,
    /// describes the services in the application.
    #[serde(rename = "services")]
    services: Option<Vec<::models::ServiceResourceDescription>>,
    /// Describes the health state of an application resource.
    #[serde(rename = "healthState")]
    health_state: Option<::models::HealthState>,
    /// When the application's health state is not 'Ok', this additional details from service fabric Health Manager for the user to know why the application is marked unhealthy.
    #[serde(rename = "unhealthyEvaluation")]
    unhealthy_evaluation: Option<String>,
    /// Status of the application resource.
    #[serde(rename = "status")]
    status: Option<String>,
    /// Gives additional information about the current status of the application deployment.
    #[serde(rename = "statusDetails")]
    status_details: Option<String>,
    /// Names of the services in the application.
    #[serde(rename = "serviceNames")]
    service_names: Option<Vec<String>>,
    /// Describes the diagnostics definition and usage for an application resource.
    #[serde(rename = "diagnostics")]
    diagnostics: Option<::models::DiagnosticsDescription>,
}

impl Default for ApplicationProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationProperties {
    /// This type describes properties of an application resource.
    pub fn new() -> ApplicationProperties {
        ApplicationProperties {
            description: None,
            debug_params: None,
            services: None,
            health_state: None,
            unhealthy_evaluation: None,
            status: None,
            status_details: None,
            service_names: None,
            diagnostics: None,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> ApplicationProperties {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_debug_params(&mut self, debug_params: String) {
        self.debug_params = Some(debug_params);
    }

    pub fn with_debug_params(
        mut self,
        debug_params: String,
    ) -> ApplicationProperties {
        self.debug_params = Some(debug_params);
        self
    }

    pub fn debug_params(&self) -> Option<&String> {
        self.debug_params.as_ref()
    }

    pub fn reset_debug_params(&mut self) {
        self.debug_params = None;
    }

    pub fn set_services(
        &mut self,
        services: Vec<::models::ServiceResourceDescription>,
    ) {
        self.services = Some(services);
    }

    pub fn with_services(
        mut self,
        services: Vec<::models::ServiceResourceDescription>,
    ) -> ApplicationProperties {
        self.services = Some(services);
        self
    }

    pub fn services(
        &self,
    ) -> Option<&Vec<::models::ServiceResourceDescription>> {
        self.services.as_ref()
    }

    pub fn reset_services(&mut self) {
        self.services = None;
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> ApplicationProperties {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_unhealthy_evaluation(&mut self, unhealthy_evaluation: String) {
        self.unhealthy_evaluation = Some(unhealthy_evaluation);
    }

    pub fn with_unhealthy_evaluation(
        mut self,
        unhealthy_evaluation: String,
    ) -> ApplicationProperties {
        self.unhealthy_evaluation = Some(unhealthy_evaluation);
        self
    }

    pub fn unhealthy_evaluation(&self) -> Option<&String> {
        self.unhealthy_evaluation.as_ref()
    }

    pub fn reset_unhealthy_evaluation(&mut self) {
        self.unhealthy_evaluation = None;
    }

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn with_status(mut self, status: String) -> ApplicationProperties {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&String> {
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
    ) -> ApplicationProperties {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }

    pub fn set_service_names(&mut self, service_names: Vec<String>) {
        self.service_names = Some(service_names);
    }

    pub fn with_service_names(
        mut self,
        service_names: Vec<String>,
    ) -> ApplicationProperties {
        self.service_names = Some(service_names);
        self
    }

    pub fn service_names(&self) -> Option<&Vec<String>> {
        self.service_names.as_ref()
    }

    pub fn reset_service_names(&mut self) {
        self.service_names = None;
    }

    pub fn set_diagnostics(
        &mut self,
        diagnostics: ::models::DiagnosticsDescription,
    ) {
        self.diagnostics = Some(diagnostics);
    }

    pub fn with_diagnostics(
        mut self,
        diagnostics: ::models::DiagnosticsDescription,
    ) -> ApplicationProperties {
        self.diagnostics = Some(diagnostics);
        self
    }

    pub fn diagnostics(&self) -> Option<&::models::DiagnosticsDescription> {
        self.diagnostics.as_ref()
    }

    pub fn reset_diagnostics(&mut self) {
        self.diagnostics = None;
    }
}
