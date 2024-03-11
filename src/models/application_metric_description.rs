/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationMetricDescription : Describes capacity information for a custom resource balancing metric. This can be used to limit the total consumption of this metric by the services of this application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationMetricDescription {
    /// The name of the metric.
    #[serde(rename = "Name")]
    name: Option<String>,
    /// The maximum node capacity for Service Fabric application. This is the maximum Load for an instance of this application on a single node. Even if the capacity of node is greater than this value, Service Fabric will limit the total load of services within the application on each node to this value. If set to zero, capacity for this metric is unlimited on each node. When creating a new application with application capacity defined, the product of MaximumNodes and this value must always be smaller than or equal to TotalApplicationCapacity. When updating existing application with application capacity, the product of MaximumNodes and this value must always be smaller than or equal to TotalApplicationCapacity.
    #[serde(rename = "MaximumCapacity")]
    maximum_capacity: Option<i64>,
    /// The node reservation capacity for Service Fabric application. This is the amount of load which is reserved on nodes which have instances of this application. If MinimumNodes is specified, then the product of these values will be the capacity reserved in the cluster for the application. If set to zero, no capacity is reserved for this metric. When setting application capacity or when updating application capacity; this value must be smaller than or equal to MaximumCapacity for each metric.
    #[serde(rename = "ReservationCapacity")]
    reservation_capacity: Option<i64>,
    /// The total metric capacity for Service Fabric application. This is the total metric capacity for this application in the cluster. Service Fabric will try to limit the sum of loads of services within the application to this value. When creating a new application with application capacity defined, the product of MaximumNodes and MaximumCapacity must always be smaller than or equal to this value.
    #[serde(rename = "TotalApplicationCapacity")]
    total_application_capacity: Option<i64>,
}

impl Default for ApplicationMetricDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationMetricDescription {
    /// Describes capacity information for a custom resource balancing metric. This can be used to limit the total consumption of this metric by the services of this application.
    pub fn new() -> ApplicationMetricDescription {
        ApplicationMetricDescription {
            name: None,
            maximum_capacity: None,
            reservation_capacity: None,
            total_application_capacity: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> ApplicationMetricDescription {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_maximum_capacity(&mut self, maximum_capacity: i64) {
        self.maximum_capacity = Some(maximum_capacity);
    }

    pub fn with_maximum_capacity(
        mut self,
        maximum_capacity: i64,
    ) -> ApplicationMetricDescription {
        self.maximum_capacity = Some(maximum_capacity);
        self
    }

    pub fn maximum_capacity(&self) -> Option<&i64> {
        self.maximum_capacity.as_ref()
    }

    pub fn reset_maximum_capacity(&mut self) {
        self.maximum_capacity = None;
    }

    pub fn set_reservation_capacity(&mut self, reservation_capacity: i64) {
        self.reservation_capacity = Some(reservation_capacity);
    }

    pub fn with_reservation_capacity(
        mut self,
        reservation_capacity: i64,
    ) -> ApplicationMetricDescription {
        self.reservation_capacity = Some(reservation_capacity);
        self
    }

    pub fn reservation_capacity(&self) -> Option<&i64> {
        self.reservation_capacity.as_ref()
    }

    pub fn reset_reservation_capacity(&mut self) {
        self.reservation_capacity = None;
    }

    pub fn set_total_application_capacity(
        &mut self,
        total_application_capacity: i64,
    ) {
        self.total_application_capacity = Some(total_application_capacity);
    }

    pub fn with_total_application_capacity(
        mut self,
        total_application_capacity: i64,
    ) -> ApplicationMetricDescription {
        self.total_application_capacity = Some(total_application_capacity);
        self
    }

    pub fn total_application_capacity(&self) -> Option<&i64> {
        self.total_application_capacity.as_ref()
    }

    pub fn reset_total_application_capacity(&mut self) {
        self.total_application_capacity = None;
    }
}