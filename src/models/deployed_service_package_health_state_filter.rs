/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServicePackageHealthStateFilter : Defines matching criteria to determine whether a deployed service package should be included as a child of a deployed application in the cluster health chunk. The deployed service packages are only returned if the parent entities match a filter specified in the cluster health chunk query description. The parent deployed application and its parent application must be included in the cluster health chunk. One filter can match zero, one or multiple deployed service packages, depending on its properties.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServicePackageHealthStateFilter {
    /// The name of the service manifest which identifies the deployed service packages that matches the filter. If specified, the filter is applied only to the specified deployed service packages, if any. If no deployed service packages with specified manifest name exist, nothing is returned in the cluster health chunk based on this filter. If any deployed service package exists, they are included in the cluster health chunk if it respects the other filter properties. If not specified, all deployed service packages that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter.
    #[serde(rename = "ServiceManifestNameFilter")]
    service_manifest_name_filter: Option<String>,
    /// The activation ID of a deployed service package that matches the filter. If not specified, the filter applies to all deployed service packages that match the other parameters. If specified, the filter matches only the deployed service package with the specified activation ID.
    #[serde(rename = "ServicePackageActivationIdFilter")]
    service_package_activation_id_filter: Option<String>,
    /// The filter for the health state of the deployed service packages. It allows selecting deployed service packages if they match the desired health states. The possible values are integer value of one of the following health states. Only deployed service packages that match the filter are returned. All deployed service packages are used to evaluate the parent deployed application aggregated health state. If not specified, default value is None, unless the deployed service package ID is specified. If the filter has default value and deployed service package ID is specified, the matching deployed service package is returned. The state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator. For example, if the provided value is 6, it matches deployed service packages with HealthState value of OK (2) and Warning (4).  - Default - Default value. Matches any HealthState. The value is zero. - None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1. - Ok - Filter that matches input with HealthState value Ok. The value is 2. - Warning - Filter that matches input with HealthState value Warning. The value is 4. - Error - Filter that matches input with HealthState value Error. The value is 8. - All - Filter that matches input with any HealthState value. The value is 65535.
    #[serde(rename = "HealthStateFilter")]
    health_state_filter: Option<i32>,
}

impl Default for DeployedServicePackageHealthStateFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedServicePackageHealthStateFilter {
    /// Defines matching criteria to determine whether a deployed service package should be included as a child of a deployed application in the cluster health chunk. The deployed service packages are only returned if the parent entities match a filter specified in the cluster health chunk query description. The parent deployed application and its parent application must be included in the cluster health chunk. One filter can match zero, one or multiple deployed service packages, depending on its properties.
    pub fn new() -> DeployedServicePackageHealthStateFilter {
        DeployedServicePackageHealthStateFilter {
            service_manifest_name_filter: None,
            service_package_activation_id_filter: None,
            health_state_filter: None,
        }
    }

    pub fn set_service_manifest_name_filter(
        &mut self,
        service_manifest_name_filter: String,
    ) {
        self.service_manifest_name_filter = Some(service_manifest_name_filter);
    }

    pub fn with_service_manifest_name_filter(
        mut self,
        service_manifest_name_filter: String,
    ) -> DeployedServicePackageHealthStateFilter {
        self.service_manifest_name_filter = Some(service_manifest_name_filter);
        self
    }

    pub fn service_manifest_name_filter(&self) -> Option<&String> {
        self.service_manifest_name_filter.as_ref()
    }

    pub fn reset_service_manifest_name_filter(&mut self) {
        self.service_manifest_name_filter = None;
    }

    pub fn set_service_package_activation_id_filter(
        &mut self,
        service_package_activation_id_filter: String,
    ) {
        self.service_package_activation_id_filter =
            Some(service_package_activation_id_filter);
    }

    pub fn with_service_package_activation_id_filter(
        mut self,
        service_package_activation_id_filter: String,
    ) -> DeployedServicePackageHealthStateFilter {
        self.service_package_activation_id_filter =
            Some(service_package_activation_id_filter);
        self
    }

    pub fn service_package_activation_id_filter(&self) -> Option<&String> {
        self.service_package_activation_id_filter.as_ref()
    }

    pub fn reset_service_package_activation_id_filter(&mut self) {
        self.service_package_activation_id_filter = None;
    }

    pub fn set_health_state_filter(&mut self, health_state_filter: i32) {
        self.health_state_filter = Some(health_state_filter);
    }

    pub fn with_health_state_filter(
        mut self,
        health_state_filter: i32,
    ) -> DeployedServicePackageHealthStateFilter {
        self.health_state_filter = Some(health_state_filter);
        self
    }

    pub fn health_state_filter(&self) -> Option<&i32> {
        self.health_state_filter.as_ref()
    }

    pub fn reset_health_state_filter(&mut self) {
        self.health_state_filter = None;
    }
}
