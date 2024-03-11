/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationHealthStateFilter : Defines matching criteria to determine whether a deployed application should be included as a child of an application in the cluster health chunk. The deployed applications are only returned if the parent application matches a filter specified in the cluster health chunk query description. One filter can match zero, one or multiple deployed applications, depending on its properties.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationHealthStateFilter {
    /// The name of the node where the application is deployed in order to match the filter. If specified, the filter is applied only to the application deployed on the specified node. If the application is not deployed on the node with the specified name, no deployed application is returned in the cluster health chunk based on this filter. Otherwise, the deployed application is included in the cluster health chunk if it respects the other filter properties. If not specified, all deployed applications that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter.
    #[serde(rename = "NodeNameFilter")]
    node_name_filter: Option<String>,
    /// The filter for the health state of the deployed applications. It allows selecting deployed applications if they match the desired health states. The possible values are integer value of one of the following health states. Only deployed applications that match the filter are returned. All deployed applications are used to evaluate the cluster aggregated health state. If not specified, default value is None, unless the node name is specified. If the filter has default value and node name is specified, the matching deployed application is returned. The state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator. For example, if the provided value is 6, it matches deployed applications with HealthState value of OK (2) and Warning (4).  - Default - Default value. Matches any HealthState. The value is zero. - None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1. - Ok - Filter that matches input with HealthState value Ok. The value is 2. - Warning - Filter that matches input with HealthState value Warning. The value is 4. - Error - Filter that matches input with HealthState value Error. The value is 8. - All - Filter that matches input with any HealthState value. The value is 65535.
    #[serde(rename = "HealthStateFilter")]
    health_state_filter: Option<i32>,
    /// Defines a list of filters that specify which deployed service packages to be included in the returned cluster health chunk as children of the parent deployed application. The deployed service packages are returned only if the parent deployed application matches a filter. If the list is empty, no deployed service packages are returned. All the deployed service packages are used to evaluate the parent deployed application aggregated health state, regardless of the input filters. The deployed application filter may specify multiple deployed service package filters. For example, it can specify a filter to return all deployed service packages with health state Error and another filter to always include a deployed service package on a node.
    #[serde(rename = "DeployedServicePackageFilters")]
    deployed_service_package_filters:
        Option<Vec<::models::DeployedServicePackageHealthStateFilter>>,
}

impl Default for DeployedApplicationHealthStateFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedApplicationHealthStateFilter {
    /// Defines matching criteria to determine whether a deployed application should be included as a child of an application in the cluster health chunk. The deployed applications are only returned if the parent application matches a filter specified in the cluster health chunk query description. One filter can match zero, one or multiple deployed applications, depending on its properties.
    pub fn new() -> DeployedApplicationHealthStateFilter {
        DeployedApplicationHealthStateFilter {
            node_name_filter: None,
            health_state_filter: None,
            deployed_service_package_filters: None,
        }
    }

    pub fn set_node_name_filter(&mut self, node_name_filter: String) {
        self.node_name_filter = Some(node_name_filter);
    }

    pub fn with_node_name_filter(
        mut self,
        node_name_filter: String,
    ) -> DeployedApplicationHealthStateFilter {
        self.node_name_filter = Some(node_name_filter);
        self
    }

    pub fn node_name_filter(&self) -> Option<&String> {
        self.node_name_filter.as_ref()
    }

    pub fn reset_node_name_filter(&mut self) {
        self.node_name_filter = None;
    }

    pub fn set_health_state_filter(&mut self, health_state_filter: i32) {
        self.health_state_filter = Some(health_state_filter);
    }

    pub fn with_health_state_filter(
        mut self,
        health_state_filter: i32,
    ) -> DeployedApplicationHealthStateFilter {
        self.health_state_filter = Some(health_state_filter);
        self
    }

    pub fn health_state_filter(&self) -> Option<&i32> {
        self.health_state_filter.as_ref()
    }

    pub fn reset_health_state_filter(&mut self) {
        self.health_state_filter = None;
    }

    pub fn set_deployed_service_package_filters(
        &mut self,
        deployed_service_package_filters: Vec<
            ::models::DeployedServicePackageHealthStateFilter,
        >,
    ) {
        self.deployed_service_package_filters =
            Some(deployed_service_package_filters);
    }

    pub fn with_deployed_service_package_filters(
        mut self,
        deployed_service_package_filters: Vec<
            ::models::DeployedServicePackageHealthStateFilter,
        >,
    ) -> DeployedApplicationHealthStateFilter {
        self.deployed_service_package_filters =
            Some(deployed_service_package_filters);
        self
    }

    pub fn deployed_service_package_filters(
        &self,
    ) -> Option<&Vec<::models::DeployedServicePackageHealthStateFilter>> {
        self.deployed_service_package_filters.as_ref()
    }

    pub fn reset_deployed_service_package_filters(&mut self) {
        self.deployed_service_package_filters = None;
    }
}
