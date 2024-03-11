/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationHealthState : Represents the health state of an application, which contains the application identifier and the aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationHealthState {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: Option<::models::ApplicationName>,
}

impl Default for ApplicationHealthState {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHealthState {
    /// Represents the health state of an application, which contains the application identifier and the aggregated health state.
    pub fn new() -> ApplicationHealthState {
        ApplicationHealthState {
            aggregated_health_state: None,
            name: None,
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
    ) -> ApplicationHealthState {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_name(&mut self, name: ::models::ApplicationName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::ApplicationName,
    ) -> ApplicationHealthState {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::ApplicationName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }
}
