/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceResourceProperties : This type describes properties of a service resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceResourceProperties {
    /// The operation system required by the code in service.
    #[serde(rename = "osType")]
    os_type: ::models::OperatingSystemType,
    /// Describes the set of code packages that forms the service. A code package describes the container and the properties for running it. All the code packages are started together on the same host and share the same context (network, process etc.).
    #[serde(rename = "codePackages")]
    code_packages: Vec<::models::ContainerCodePackageProperties>,
    /// The names of the private networks that this service needs to be part of.
    #[serde(rename = "networkRefs")]
    network_refs: Option<Vec<::models::NetworkRef>>,
    /// Reference to sinks in DiagnosticsDescription.
    #[serde(rename = "diagnostics")]
    diagnostics: Option<::models::DiagnosticsRef>,
    /// User readable description of the service.
    #[serde(rename = "description")]
    description: Option<String>,
    /// The number of replicas of the service to create. Defaults to 1 if not specified.
    #[serde(rename = "replicaCount")]
    replica_count: Option<i32>,
    /// Auto scaling policies
    #[serde(rename = "autoScalingPolicies")]
    auto_scaling_policies: Option<Vec<::models::AutoScalingPolicy>>,
    /// Status of the service.
    #[serde(rename = "status")]
    status: Option<::models::ResourceStatus>,
    /// Gives additional information about the current status of the service.
    #[serde(rename = "statusDetails")]
    status_details: Option<String>,
    /// Describes the health state of an application resource.
    #[serde(rename = "healthState")]
    health_state: Option<::models::HealthState>,
    /// When the service's health state is not 'Ok', this additional details from service fabric Health Manager for the user to know why the service is marked unhealthy.
    #[serde(rename = "unhealthyEvaluation")]
    unhealthy_evaluation: Option<String>,
}

impl ServiceResourceProperties {
    /// This type describes properties of a service resource.
    pub fn new(
        os_type: ::models::OperatingSystemType,
        code_packages: Vec<::models::ContainerCodePackageProperties>,
    ) -> ServiceResourceProperties {
        ServiceResourceProperties {
            os_type,
            code_packages,
            network_refs: None,
            diagnostics: None,
            description: None,
            replica_count: None,
            auto_scaling_policies: None,
            status: None,
            status_details: None,
            health_state: None,
            unhealthy_evaluation: None,
        }
    }

    pub fn set_os_type(&mut self, os_type: ::models::OperatingSystemType) {
        self.os_type = os_type;
    }

    pub fn with_os_type(
        mut self,
        os_type: ::models::OperatingSystemType,
    ) -> ServiceResourceProperties {
        self.os_type = os_type;
        self
    }

    pub fn os_type(&self) -> &::models::OperatingSystemType {
        &self.os_type
    }

    pub fn set_code_packages(
        &mut self,
        code_packages: Vec<::models::ContainerCodePackageProperties>,
    ) {
        self.code_packages = code_packages;
    }

    pub fn with_code_packages(
        mut self,
        code_packages: Vec<::models::ContainerCodePackageProperties>,
    ) -> ServiceResourceProperties {
        self.code_packages = code_packages;
        self
    }

    pub fn code_packages(
        &self,
    ) -> &Vec<::models::ContainerCodePackageProperties> {
        &self.code_packages
    }

    pub fn set_network_refs(
        &mut self,
        network_refs: Vec<::models::NetworkRef>,
    ) {
        self.network_refs = Some(network_refs);
    }

    pub fn with_network_refs(
        mut self,
        network_refs: Vec<::models::NetworkRef>,
    ) -> ServiceResourceProperties {
        self.network_refs = Some(network_refs);
        self
    }

    pub fn network_refs(&self) -> Option<&Vec<::models::NetworkRef>> {
        self.network_refs.as_ref()
    }

    pub fn reset_network_refs(&mut self) {
        self.network_refs = None;
    }

    pub fn set_diagnostics(&mut self, diagnostics: ::models::DiagnosticsRef) {
        self.diagnostics = Some(diagnostics);
    }

    pub fn with_diagnostics(
        mut self,
        diagnostics: ::models::DiagnosticsRef,
    ) -> ServiceResourceProperties {
        self.diagnostics = Some(diagnostics);
        self
    }

    pub fn diagnostics(&self) -> Option<&::models::DiagnosticsRef> {
        self.diagnostics.as_ref()
    }

    pub fn reset_diagnostics(&mut self) {
        self.diagnostics = None;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> ServiceResourceProperties {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_replica_count(&mut self, replica_count: i32) {
        self.replica_count = Some(replica_count);
    }

    pub fn with_replica_count(
        mut self,
        replica_count: i32,
    ) -> ServiceResourceProperties {
        self.replica_count = Some(replica_count);
        self
    }

    pub fn replica_count(&self) -> Option<&i32> {
        self.replica_count.as_ref()
    }

    pub fn reset_replica_count(&mut self) {
        self.replica_count = None;
    }

    pub fn set_auto_scaling_policies(
        &mut self,
        auto_scaling_policies: Vec<::models::AutoScalingPolicy>,
    ) {
        self.auto_scaling_policies = Some(auto_scaling_policies);
    }

    pub fn with_auto_scaling_policies(
        mut self,
        auto_scaling_policies: Vec<::models::AutoScalingPolicy>,
    ) -> ServiceResourceProperties {
        self.auto_scaling_policies = Some(auto_scaling_policies);
        self
    }

    pub fn auto_scaling_policies(
        &self,
    ) -> Option<&Vec<::models::AutoScalingPolicy>> {
        self.auto_scaling_policies.as_ref()
    }

    pub fn reset_auto_scaling_policies(&mut self) {
        self.auto_scaling_policies = None;
    }

    pub fn set_status(&mut self, status: ::models::ResourceStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ResourceStatus,
    ) -> ServiceResourceProperties {
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
    ) -> ServiceResourceProperties {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> ServiceResourceProperties {
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
    ) -> ServiceResourceProperties {
        self.unhealthy_evaluation = Some(unhealthy_evaluation);
        self
    }

    pub fn unhealthy_evaluation(&self) -> Option<&String> {
        self.unhealthy_evaluation.as_ref()
    }

    pub fn reset_unhealthy_evaluation(&mut self) {
        self.unhealthy_evaluation = None;
    }
}
