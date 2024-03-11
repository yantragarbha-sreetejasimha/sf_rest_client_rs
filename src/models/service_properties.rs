/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceProperties : Describes properties of a service resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceProperties {
    /// User readable description of the service.
    #[serde(rename = "description")]
    description: Option<String>,
    /// The number of replicas of the service to create. Defaults to 1 if not specified.
    #[serde(rename = "replicaCount")]
    replica_count: Option<i32>,
    /// The execution policy of the service
    #[serde(rename = "executionPolicy")]
    execution_policy: Option<::models::ExecutionPolicy>,
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
    /// The service identity list.
    #[serde(rename = "identityRefs")]
    identity_refs: Option<Vec<::models::ServiceIdentity>>,
    /// Dns name of the service.
    #[serde(rename = "dnsName")]
    dns_name: Option<String>,
}

impl Default for ServiceProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceProperties {
    /// Describes properties of a service resource.
    pub fn new() -> ServiceProperties {
        ServiceProperties {
            description: None,
            replica_count: None,
            execution_policy: None,
            auto_scaling_policies: None,
            status: None,
            status_details: None,
            health_state: None,
            unhealthy_evaluation: None,
            identity_refs: None,
            dns_name: None,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> ServiceProperties {
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
    ) -> ServiceProperties {
        self.replica_count = Some(replica_count);
        self
    }

    pub fn replica_count(&self) -> Option<&i32> {
        self.replica_count.as_ref()
    }

    pub fn reset_replica_count(&mut self) {
        self.replica_count = None;
    }

    pub fn set_execution_policy(
        &mut self,
        execution_policy: ::models::ExecutionPolicy,
    ) {
        self.execution_policy = Some(execution_policy);
    }

    pub fn with_execution_policy(
        mut self,
        execution_policy: ::models::ExecutionPolicy,
    ) -> ServiceProperties {
        self.execution_policy = Some(execution_policy);
        self
    }

    pub fn execution_policy(&self) -> Option<&::models::ExecutionPolicy> {
        self.execution_policy.as_ref()
    }

    pub fn reset_execution_policy(&mut self) {
        self.execution_policy = None;
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
    ) -> ServiceProperties {
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
    ) -> ServiceProperties {
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
    ) -> ServiceProperties {
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
    ) -> ServiceProperties {
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
    ) -> ServiceProperties {
        self.unhealthy_evaluation = Some(unhealthy_evaluation);
        self
    }

    pub fn unhealthy_evaluation(&self) -> Option<&String> {
        self.unhealthy_evaluation.as_ref()
    }

    pub fn reset_unhealthy_evaluation(&mut self) {
        self.unhealthy_evaluation = None;
    }

    pub fn set_identity_refs(
        &mut self,
        identity_refs: Vec<::models::ServiceIdentity>,
    ) {
        self.identity_refs = Some(identity_refs);
    }

    pub fn with_identity_refs(
        mut self,
        identity_refs: Vec<::models::ServiceIdentity>,
    ) -> ServiceProperties {
        self.identity_refs = Some(identity_refs);
        self
    }

    pub fn identity_refs(&self) -> Option<&Vec<::models::ServiceIdentity>> {
        self.identity_refs.as_ref()
    }

    pub fn reset_identity_refs(&mut self) {
        self.identity_refs = None;
    }

    pub fn set_dns_name(&mut self, dns_name: String) {
        self.dns_name = Some(dns_name);
    }

    pub fn with_dns_name(mut self, dns_name: String) -> ServiceProperties {
        self.dns_name = Some(dns_name);
        self
    }

    pub fn dns_name(&self) -> Option<&String> {
        self.dns_name.as_ref()
    }

    pub fn reset_dns_name(&mut self) {
        self.dns_name = None;
    }
}
