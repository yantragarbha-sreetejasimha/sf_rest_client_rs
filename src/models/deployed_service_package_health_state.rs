/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServicePackageHealthState : Represents the health state of a deployed service package, containing the entity identifier and the aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServicePackageHealthState {
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Name of the node on which the service package is deployed.
    #[serde(rename = "NodeName")]
    node_name: Option<::models::NodeName>,
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "ApplicationName")]
    application_name: Option<::models::ApplicationName>,
    /// Name of the manifest describing the service package.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: Option<::models::ServiceManifestName>,
    /// The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId is always an empty string.
    #[serde(rename = "ServicePackageActivationId")]
    service_package_activation_id: Option<::models::ServicePackageActivationId>,
}

impl Default for DeployedServicePackageHealthState {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedServicePackageHealthState {
    /// Represents the health state of a deployed service package, containing the entity identifier and the aggregated health state.
    pub fn new() -> DeployedServicePackageHealthState {
        DeployedServicePackageHealthState {
            aggregated_health_state: None,
            node_name: None,
            application_name: None,
            service_manifest_name: None,
            service_package_activation_id: None,
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
    ) -> DeployedServicePackageHealthState {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> DeployedServicePackageHealthState {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&::models::NodeName> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
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
    ) -> DeployedServicePackageHealthState {
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
    ) -> DeployedServicePackageHealthState {
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

    pub fn set_service_package_activation_id(
        &mut self,
        service_package_activation_id: ::models::ServicePackageActivationId,
    ) {
        self.service_package_activation_id =
            Some(service_package_activation_id);
    }

    pub fn with_service_package_activation_id(
        mut self,
        service_package_activation_id: ::models::ServicePackageActivationId,
    ) -> DeployedServicePackageHealthState {
        self.service_package_activation_id =
            Some(service_package_activation_id);
        self
    }

    pub fn service_package_activation_id(
        &self,
    ) -> Option<&::models::ServicePackageActivationId> {
        self.service_package_activation_id.as_ref()
    }

    pub fn reset_service_package_activation_id(&mut self) {
        self.service_package_activation_id = None;
    }
}
