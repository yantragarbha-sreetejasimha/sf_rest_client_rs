/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationHealthPolicy : Defines a health policy used to evaluate the health of an application or one of its children entities.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationHealthPolicy {
    /// Indicates whether warnings are treated with the same severity as errors.
    #[serde(rename = "ConsiderWarningAsError")]
    consider_warning_as_error: Option<bool>,
    /// The maximum allowed percentage of unhealthy deployed applications. Allowed values are Byte values from zero to 100. The percentage represents the maximum tolerated percentage of deployed applications that can be unhealthy before the application is considered in error. This is calculated by dividing the number of unhealthy deployed applications over the number of nodes where the application is currently deployed on in the cluster. The computation rounds up to tolerate one failure on small numbers of nodes. Default percentage is zero.
    #[serde(rename = "MaxPercentUnhealthyDeployedApplications")]
    max_percent_unhealthy_deployed_applications: Option<i32>,
    /// The health policy used by default to evaluate the health of a service type.
    #[serde(rename = "DefaultServiceTypeHealthPolicy")]
    default_service_type_health_policy:
        Option<::models::ServiceTypeHealthPolicy>,
    /// The map with service type health policy per service type name. The map is empty by default.
    #[serde(rename = "ServiceTypeHealthPolicyMap")]
    service_type_health_policy_map:
        Option<::models::ServiceTypeHealthPolicyMap>,
}

impl Default for ApplicationHealthPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHealthPolicy {
    /// Defines a health policy used to evaluate the health of an application or one of its children entities.
    pub fn new() -> ApplicationHealthPolicy {
        ApplicationHealthPolicy {
            consider_warning_as_error: None,
            max_percent_unhealthy_deployed_applications: None,
            default_service_type_health_policy: None,
            service_type_health_policy_map: None,
        }
    }

    pub fn set_consider_warning_as_error(
        &mut self,
        consider_warning_as_error: bool,
    ) {
        self.consider_warning_as_error = Some(consider_warning_as_error);
    }

    pub fn with_consider_warning_as_error(
        mut self,
        consider_warning_as_error: bool,
    ) -> ApplicationHealthPolicy {
        self.consider_warning_as_error = Some(consider_warning_as_error);
        self
    }

    pub fn consider_warning_as_error(&self) -> Option<&bool> {
        self.consider_warning_as_error.as_ref()
    }

    pub fn reset_consider_warning_as_error(&mut self) {
        self.consider_warning_as_error = None;
    }

    pub fn set_max_percent_unhealthy_deployed_applications(
        &mut self,
        max_percent_unhealthy_deployed_applications: i32,
    ) {
        self.max_percent_unhealthy_deployed_applications =
            Some(max_percent_unhealthy_deployed_applications);
    }

    pub fn with_max_percent_unhealthy_deployed_applications(
        mut self,
        max_percent_unhealthy_deployed_applications: i32,
    ) -> ApplicationHealthPolicy {
        self.max_percent_unhealthy_deployed_applications =
            Some(max_percent_unhealthy_deployed_applications);
        self
    }

    pub fn max_percent_unhealthy_deployed_applications(&self) -> Option<&i32> {
        self.max_percent_unhealthy_deployed_applications.as_ref()
    }

    pub fn reset_max_percent_unhealthy_deployed_applications(&mut self) {
        self.max_percent_unhealthy_deployed_applications = None;
    }

    pub fn set_default_service_type_health_policy(
        &mut self,
        default_service_type_health_policy: ::models::ServiceTypeHealthPolicy,
    ) {
        self.default_service_type_health_policy =
            Some(default_service_type_health_policy);
    }

    pub fn with_default_service_type_health_policy(
        mut self,
        default_service_type_health_policy: ::models::ServiceTypeHealthPolicy,
    ) -> ApplicationHealthPolicy {
        self.default_service_type_health_policy =
            Some(default_service_type_health_policy);
        self
    }

    pub fn default_service_type_health_policy(
        &self,
    ) -> Option<&::models::ServiceTypeHealthPolicy> {
        self.default_service_type_health_policy.as_ref()
    }

    pub fn reset_default_service_type_health_policy(&mut self) {
        self.default_service_type_health_policy = None;
    }

    pub fn set_service_type_health_policy_map(
        &mut self,
        service_type_health_policy_map: ::models::ServiceTypeHealthPolicyMap,
    ) {
        self.service_type_health_policy_map =
            Some(service_type_health_policy_map);
    }

    pub fn with_service_type_health_policy_map(
        mut self,
        service_type_health_policy_map: ::models::ServiceTypeHealthPolicyMap,
    ) -> ApplicationHealthPolicy {
        self.service_type_health_policy_map =
            Some(service_type_health_policy_map);
        self
    }

    pub fn service_type_health_policy_map(
        &self,
    ) -> Option<&::models::ServiceTypeHealthPolicyMap> {
        self.service_type_health_policy_map.as_ref()
    }

    pub fn reset_service_type_health_policy_map(&mut self) {
        self.service_type_health_policy_map = None;
    }
}