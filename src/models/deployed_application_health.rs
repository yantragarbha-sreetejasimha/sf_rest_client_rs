/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationHealth : Information about the health of an application deployed on a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationHealth {
    /// The HealthState representing the aggregated health state of the entity computed by Health Manager. The health evaluation of the entity reflects all events reported on the entity and its children (if any). The aggregation is done by applying the desired health policy.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// The list of health events reported on the entity.
    #[serde(rename = "HealthEvents")]
    health_events: Option<Vec<::models::HealthEvent>>,
    /// The unhealthy evaluations that show why the current aggregated health state was returned by Health Manager.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
    /// Shows the health statistics for all children types of the queried entity.
    #[serde(rename = "HealthStatistics")]
    health_statistics: Option<::models::HealthStatistics>,
    /// Name of the application deployed on the node whose health information is described by this object.
    #[serde(rename = "Name")]
    name: Option<::models::ApplicationName>,
    /// Name of the node where this application is deployed.
    #[serde(rename = "NodeName")]
    node_name: Option<::models::NodeName>,
    /// Deployed service package health states for the current deployed application as found in the health store.
    #[serde(rename = "DeployedServicePackageHealthStates")]
    deployed_service_package_health_states:
        Option<::models::DeployedServicePackageHealthStateList>,
}

impl Default for DeployedApplicationHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedApplicationHealth {
    /// Information about the health of an application deployed on a Service Fabric node.
    pub fn new() -> DeployedApplicationHealth {
        DeployedApplicationHealth {
            aggregated_health_state: None,
            health_events: None,
            unhealthy_evaluations: None,
            health_statistics: None,
            name: None,
            node_name: None,
            deployed_service_package_health_states: None,
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
    ) -> DeployedApplicationHealth {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_health_events(
        &mut self,
        health_events: Vec<::models::HealthEvent>,
    ) {
        self.health_events = Some(health_events);
    }

    pub fn with_health_events(
        mut self,
        health_events: Vec<::models::HealthEvent>,
    ) -> DeployedApplicationHealth {
        self.health_events = Some(health_events);
        self
    }

    pub fn health_events(&self) -> Option<&Vec<::models::HealthEvent>> {
        self.health_events.as_ref()
    }

    pub fn reset_health_events(&mut self) {
        self.health_events = None;
    }

    pub fn set_unhealthy_evaluations(
        &mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
    }

    pub fn with_unhealthy_evaluations(
        mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) -> DeployedApplicationHealth {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
        self
    }

    pub fn unhealthy_evaluations(
        &self,
    ) -> Option<&::models::UnhealthyEvaluations> {
        self.unhealthy_evaluations.as_ref()
    }

    pub fn reset_unhealthy_evaluations(&mut self) {
        self.unhealthy_evaluations = None;
    }

    pub fn set_health_statistics(
        &mut self,
        health_statistics: ::models::HealthStatistics,
    ) {
        self.health_statistics = Some(health_statistics);
    }

    pub fn with_health_statistics(
        mut self,
        health_statistics: ::models::HealthStatistics,
    ) -> DeployedApplicationHealth {
        self.health_statistics = Some(health_statistics);
        self
    }

    pub fn health_statistics(&self) -> Option<&::models::HealthStatistics> {
        self.health_statistics.as_ref()
    }

    pub fn reset_health_statistics(&mut self) {
        self.health_statistics = None;
    }

    pub fn set_name(&mut self, name: ::models::ApplicationName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::ApplicationName,
    ) -> DeployedApplicationHealth {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::ApplicationName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> DeployedApplicationHealth {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&::models::NodeName> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
    }

    pub fn set_deployed_service_package_health_states(
        &mut self,
        deployed_service_package_health_states: ::models::DeployedServicePackageHealthStateList,
    ) {
        self.deployed_service_package_health_states =
            Some(deployed_service_package_health_states);
    }

    pub fn with_deployed_service_package_health_states(
        mut self,
        deployed_service_package_health_states: ::models::DeployedServicePackageHealthStateList,
    ) -> DeployedApplicationHealth {
        self.deployed_service_package_health_states =
            Some(deployed_service_package_health_states);
        self
    }

    pub fn deployed_service_package_health_states(
        &self,
    ) -> Option<&::models::DeployedServicePackageHealthStateList> {
        self.deployed_service_package_health_states.as_ref()
    }

    pub fn reset_deployed_service_package_health_states(&mut self) {
        self.deployed_service_package_health_states = None;
    }
}
