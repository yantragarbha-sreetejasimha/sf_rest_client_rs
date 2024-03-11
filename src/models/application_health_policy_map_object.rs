/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationHealthPolicyMapObject : Represents the map of application health policies for a ServiceFabric cluster upgrade

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationHealthPolicyMapObject {
    /// Defines a map that contains specific application health policies for different applications. Each entry specifies as key the application name and as value an ApplicationHealthPolicy used to evaluate the application health. If an application is not specified in the map, the application health evaluation uses the ApplicationHealthPolicy found in its application manifest or the default application health policy (if no health policy is defined in the manifest). The map is empty by default.
    #[serde(rename = "ApplicationHealthPolicyMap")]
    application_health_policy_map: Option<::models::ApplicationHealthPolicyMap>,
}

impl Default for ApplicationHealthPolicyMapObject {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHealthPolicyMapObject {
    /// Represents the map of application health policies for a ServiceFabric cluster upgrade
    pub fn new() -> ApplicationHealthPolicyMapObject {
        ApplicationHealthPolicyMapObject {
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
    ) -> ApplicationHealthPolicyMapObject {
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
