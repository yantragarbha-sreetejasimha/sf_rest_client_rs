/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceHealthState : Represents the health state of a service, which contains the service identifier and its aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceHealthState {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Name of the service whose health state is represented by this object.
    #[serde(rename = "ServiceName")]
    service_name: Option<::models::ServiceName>,
}

impl Default for ServiceHealthState {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceHealthState {
    /// Represents the health state of a service, which contains the service identifier and its aggregated health state.
    pub fn new() -> ServiceHealthState {
        ServiceHealthState {
            aggregated_health_state: None,
            service_name: None,
        }
    }

    pub fn set_aggregated_health_state(
        &mut self,
        aggregated_health_state: ::models::HealthState,
    ) {
        self.aggregated_health_state = Some(aggregated_health_state);
    }

    pub fn with_aggregated_health_state(
        mut self,
        aggregated_health_state: ::models::HealthState,
    ) -> ServiceHealthState {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_service_name(&mut self, service_name: ::models::ServiceName) {
        self.service_name = Some(service_name);
    }

    pub fn with_service_name(
        mut self,
        service_name: ::models::ServiceName,
    ) -> ServiceHealthState {
        self.service_name = Some(service_name);
        self
    }

    pub fn service_name(&self) -> Option<&::models::ServiceName> {
        self.service_name.as_ref()
    }

    pub fn reset_service_name(&mut self) {
        self.service_name = None;
    }
}
