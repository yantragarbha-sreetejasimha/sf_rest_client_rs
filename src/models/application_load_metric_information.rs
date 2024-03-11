/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationLoadMetricInformation : Describes load information for a custom resource balancing metric. This can be used to limit the total consumption of this metric by the services of this application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationLoadMetricInformation {
    /// The name of the metric.
    #[serde(rename = "Name")]
    name: Option<String>,
    /// This is the capacity reserved in the cluster for the application. It's the product of NodeReservationCapacity and MinimumNodes. If set to zero, no capacity is reserved for this metric. When setting application capacity or when updating application capacity this value must be smaller than or equal to MaximumCapacity for each metric.
    #[serde(rename = "ReservationCapacity")]
    reservation_capacity: Option<i64>,
    /// Total capacity for this metric in this application instance.
    #[serde(rename = "ApplicationCapacity")]
    application_capacity: Option<i64>,
    /// Current load for this metric in this application instance.
    #[serde(rename = "ApplicationLoad")]
    application_load: Option<i64>,
}

impl Default for ApplicationLoadMetricInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationLoadMetricInformation {
    /// Describes load information for a custom resource balancing metric. This can be used to limit the total consumption of this metric by the services of this application.
    pub fn new() -> ApplicationLoadMetricInformation {
        ApplicationLoadMetricInformation {
            name: None,
            reservation_capacity: None,
            application_capacity: None,
            application_load: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: String,
    ) -> ApplicationLoadMetricInformation {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_reservation_capacity(&mut self, reservation_capacity: i64) {
        self.reservation_capacity = Some(reservation_capacity);
    }

    pub fn with_reservation_capacity(
        mut self,
        reservation_capacity: i64,
    ) -> ApplicationLoadMetricInformation {
        self.reservation_capacity = Some(reservation_capacity);
        self
    }

    pub fn reservation_capacity(&self) -> Option<&i64> {
        self.reservation_capacity.as_ref()
    }

    pub fn reset_reservation_capacity(&mut self) {
        self.reservation_capacity = None;
    }

    pub fn set_application_capacity(&mut self, application_capacity: i64) {
        self.application_capacity = Some(application_capacity);
    }

    pub fn with_application_capacity(
        mut self,
        application_capacity: i64,
    ) -> ApplicationLoadMetricInformation {
        self.application_capacity = Some(application_capacity);
        self
    }

    pub fn application_capacity(&self) -> Option<&i64> {
        self.application_capacity.as_ref()
    }

    pub fn reset_application_capacity(&mut self) {
        self.application_capacity = None;
    }

    pub fn set_application_load(&mut self, application_load: i64) {
        self.application_load = Some(application_load);
    }

    pub fn with_application_load(
        mut self,
        application_load: i64,
    ) -> ApplicationLoadMetricInformation {
        self.application_load = Some(application_load);
        self
    }

    pub fn application_load(&self) -> Option<&i64> {
        self.application_load.as_ref()
    }

    pub fn reset_application_load(&mut self) {
        self.application_load = None;
    }
}
