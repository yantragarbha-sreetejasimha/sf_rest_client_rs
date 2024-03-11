/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceHealthStateFilter : Defines matching criteria to determine whether a service should be included as a child of an application in the cluster health chunk. The services are only returned if the parent application matches a filter specified in the cluster health chunk query description. One filter can match zero, one or multiple services, depending on its properties.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceHealthStateFilter {
    /// The name of the service that matches the filter. The filter is applied only to the specified service, if it exists. If the service doesn't exist, no service is returned in the cluster health chunk based on this filter. If the service exists, it is included as the application's child if the health state matches the other filter properties. If not specified, all services that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter.
    #[serde(rename = "ServiceNameFilter")]
    service_name_filter: Option<String>,
    /// The filter for the health state of the services. It allows selecting services if they match the desired health states. The possible values are integer value of one of the following health states. Only services that match the filter are returned. All services are used to evaluate the cluster aggregated health state. If not specified, default value is None, unless the service name is specified. If the filter has default value and service name is specified, the matching service is returned. The state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise 'OR' operator. For example, if the provided value is 6, it matches services with HealthState value of OK (2) and Warning (4).  - Default - Default value. Matches any HealthState. The value is zero. - None - Filter that doesn't match any HealthState value. Used in order to return no results on a given collection of states. The value is 1. - Ok - Filter that matches input with HealthState value Ok. The value is 2. - Warning - Filter that matches input with HealthState value Warning. The value is 4. - Error - Filter that matches input with HealthState value Error. The value is 8. - All - Filter that matches input with any HealthState value. The value is 65535.
    #[serde(rename = "HealthStateFilter")]
    health_state_filter: Option<i32>,
    /// Defines a list of filters that specify which partitions to be included in the returned cluster health chunk as children of the service. The partitions are returned only if the parent service matches a filter. If the list is empty, no partitions are returned. All the partitions are used to evaluate the parent service aggregated health state, regardless of the input filters. The service filter may specify multiple partition filters. For example, it can specify a filter to return all partitions with health state Error and another filter to always include a partition identified by its partition ID.
    #[serde(rename = "PartitionFilters")]
    partition_filters: Option<Vec<::models::PartitionHealthStateFilter>>,
}

impl Default for ServiceHealthStateFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceHealthStateFilter {
    /// Defines matching criteria to determine whether a service should be included as a child of an application in the cluster health chunk. The services are only returned if the parent application matches a filter specified in the cluster health chunk query description. One filter can match zero, one or multiple services, depending on its properties.
    pub fn new() -> ServiceHealthStateFilter {
        ServiceHealthStateFilter {
            service_name_filter: None,
            health_state_filter: None,
            partition_filters: None,
        }
    }

    pub fn set_service_name_filter(&mut self, service_name_filter: String) {
        self.service_name_filter = Some(service_name_filter);
    }

    pub fn with_service_name_filter(
        mut self,
        service_name_filter: String,
    ) -> ServiceHealthStateFilter {
        self.service_name_filter = Some(service_name_filter);
        self
    }

    pub fn service_name_filter(&self) -> Option<&String> {
        self.service_name_filter.as_ref()
    }

    pub fn reset_service_name_filter(&mut self) {
        self.service_name_filter = None;
    }

    pub fn set_health_state_filter(&mut self, health_state_filter: i32) {
        self.health_state_filter = Some(health_state_filter);
    }

    pub fn with_health_state_filter(
        mut self,
        health_state_filter: i32,
    ) -> ServiceHealthStateFilter {
        self.health_state_filter = Some(health_state_filter);
        self
    }

    pub fn health_state_filter(&self) -> Option<&i32> {
        self.health_state_filter.as_ref()
    }

    pub fn reset_health_state_filter(&mut self) {
        self.health_state_filter = None;
    }

    pub fn set_partition_filters(
        &mut self,
        partition_filters: Vec<::models::PartitionHealthStateFilter>,
    ) {
        self.partition_filters = Some(partition_filters);
    }

    pub fn with_partition_filters(
        mut self,
        partition_filters: Vec<::models::PartitionHealthStateFilter>,
    ) -> ServiceHealthStateFilter {
        self.partition_filters = Some(partition_filters);
        self
    }

    pub fn partition_filters(
        &self,
    ) -> Option<&Vec<::models::PartitionHealthStateFilter>> {
        self.partition_filters.as_ref()
    }

    pub fn reset_partition_filters(&mut self) {
        self.partition_filters = None;
    }
}
