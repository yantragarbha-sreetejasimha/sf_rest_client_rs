/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationHealthPolicies : Defines the application health policy map used to evaluate the health of an application or one of its children entities.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationHealthPolicies {
    /// The wrapper that contains the map with application health policies used to evaluate specific applications in the cluster.
    #[serde(rename = "ApplicationHealthPolicyMap")]
    application_health_policy_map: Option<::models::ApplicationHealthPolicyMap>,
}

impl Default for ApplicationHealthPolicies {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHealthPolicies {
    /// Defines the application health policy map used to evaluate the health of an application or one of its children entities.
    pub fn new() -> ApplicationHealthPolicies {
        ApplicationHealthPolicies {
            application_health_policy_map: None,
        }
    }

    pub fn set_application_health_policy_map(
        &mut self,
        application_health_policy_map: ::models::ApplicationHealthPolicyMap,
    ) {
        self.application_health_policy_map =
            Some(application_health_policy_map);
    }

    pub fn with_application_health_policy_map(
        mut self,
        application_health_policy_map: ::models::ApplicationHealthPolicyMap,
    ) -> ApplicationHealthPolicies {
        self.application_health_policy_map =
            Some(application_health_policy_map);
        self
    }

    pub fn application_health_policy_map(
        &self,
    ) -> Option<&::models::ApplicationHealthPolicyMap> {
        self.application_health_policy_map.as_ref()
    }

    pub fn reset_application_health_policy_map(&mut self) {
        self.application_health_policy_map = None;
    }
}
