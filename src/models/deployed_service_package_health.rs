/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServicePackageHealth : Information about the health of a service package for a specific application deployed on a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServicePackageHealth {
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
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "ApplicationName")]
    application_name: Option<::models::ApplicationName>,
    /// Name of the service manifest.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: Option<::models::ServiceManifestName>,
    /// Name of the node where this service package is deployed.
    #[serde(rename = "NodeName")]
    node_name: Option<::models::NodeName>,
}

impl Default for DeployedServicePackageHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedServicePackageHealth {
    /// Information about the health of a service package for a specific application deployed on a Service Fabric node.
    pub fn new() -> DeployedServicePackageHealth {
        DeployedServicePackageHealth {
            aggregated_health_state: None,
            health_events: None,
            unhealthy_evaluations: None,
            health_statistics: None,
            application_name: None,
            service_manifest_name: None,
            node_name: None,
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
    ) -> DeployedServicePackageHealth {
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
    ) -> DeployedServicePackageHealth {
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
    ) -> DeployedServicePackageHealth {
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
    ) -> DeployedServicePackageHealth {
        self.health_statistics = Some(health_statistics);
        self
    }

    pub fn health_statistics(&self) -> Option<&::models::HealthStatistics> {
        self.health_statistics.as_ref()
    }

    pub fn reset_health_statistics(&mut self) {
        self.health_statistics = None;
    }

    pub fn set_application_name(
        &mut self,
        application_name: ::models::ApplicationName,
    ) {
        self.application_name = Some(application_name);
    }

    pub fn with_application_name(
        mut self,
        application_name: ::models::ApplicationName,
    ) -> DeployedServicePackageHealth {
        self.application_name = Some(application_name);
        self
    }

    pub fn application_name(&self) -> Option<&::models::ApplicationName> {
        self.application_name.as_ref()
    }

    pub fn reset_application_name(&mut self) {
        self.application_name = None;
    }

    pub fn set_service_manifest_name(
        &mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) {
        self.service_manifest_name = Some(service_manifest_name);
    }

    pub fn with_service_manifest_name(
        mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) -> DeployedServicePackageHealth {
        self.service_manifest_name = Some(service_manifest_name);
        self
    }

    pub fn service_manifest_name(
        &self,
    ) -> Option<&::models::ServiceManifestName> {
        self.service_manifest_name.as_ref()
    }

    pub fn reset_service_manifest_name(&mut self) {
        self.service_manifest_name = None;
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> DeployedServicePackageHealth {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&::models::NodeName> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
    }
}
