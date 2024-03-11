/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceDescription : A ServiceDescription contains all of the information necessary to create a service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDescription {
    /// The service kind.
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "ApplicationName")]
    application_name: Option<::models::ApplicationName>,
    /// The full name of the service with 'fabric:' URI scheme.
    #[serde(rename = "ServiceName")]
    service_name: ::models::ServiceName,
    /// Name of the service type as specified in the service manifest.
    #[serde(rename = "ServiceTypeName")]
    service_type_name: ::models::ServiceTypeName,
    /// The initialization data as an array of bytes. Initialization data is passed to service instances or replicas when they are created.
    #[serde(rename = "InitializationData")]
    initialization_data: Option<String>,
    /// The partition description as an object.
    #[serde(rename = "PartitionDescription")]
    partition_description: ::models::PartitionSchemeDescription,
    /// The placement constraints as a string. Placement constraints are boolean expressions on node properties and allow for restricting a service to particular nodes based on the service requirements. For example, to place a service on nodes where NodeType is blue specify the following: \"NodeColor == blue)\".
    #[serde(rename = "PlacementConstraints")]
    placement_constraints: Option<String>,
    /// The correlation scheme.
    #[serde(rename = "CorrelationScheme")]
    correlation_scheme: Option<::models::CorrelationSchemeList>,
    /// The service load metrics.
    #[serde(rename = "ServiceLoadMetrics")]
    service_load_metrics: Option<::models::ServiceLoadMetricsList>,
    /// The service placement policies.
    #[serde(rename = "ServicePlacementPolicies")]
    service_placement_policies: Option<::models::ServicePlacementPoliciesList>,
    /// The move cost for the service.
    #[serde(rename = "DefaultMoveCost")]
    default_move_cost: Option<::models::MoveCost>,
    /// Indicates if the DefaultMoveCost property is specified.
    #[serde(rename = "IsDefaultMoveCostSpecified")]
    is_default_move_cost_specified: Option<bool>,
    /// The activation mode of service package to be used for a service.
    #[serde(rename = "ServicePackageActivationMode")]
    service_package_activation_mode:
        Option<::models::ServicePackageActivationMode>,
    /// The DNS name of the service. It requires the DNS system service to be enabled in Service Fabric cluster.
    #[serde(rename = "ServiceDnsName")]
    service_dns_name: Option<String>,
    /// Scaling policies for this service.
    #[serde(rename = "ScalingPolicies")]
    scaling_policies: Option<::models::ScalingPolicyDescriptionList>,
}

impl ServiceDescription {
    /// A ServiceDescription contains all of the information necessary to create a service.
    pub fn new(
        service_kind: ::models::ServiceKind,
        service_name: ::models::ServiceName,
        service_type_name: ::models::ServiceTypeName,
        partition_description: ::models::PartitionSchemeDescription,
    ) -> ServiceDescription {
        ServiceDescription {
            service_kind,
            application_name: None,
            service_name,
            service_type_name,
            initialization_data: None,
            partition_description,
            placement_constraints: None,
            correlation_scheme: None,
            service_load_metrics: None,
            service_placement_policies: None,
            default_move_cost: None,
            is_default_move_cost_specified: None,
            service_package_activation_mode: None,
            service_dns_name: None,
            scaling_policies: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> ServiceDescription {
        self.service_kind = service_kind;
        self
    }

    pub fn service_kind(&self) -> &::models::ServiceKind {
        &self.service_kind
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
    ) -> ServiceDescription {
        self.application_name = Some(application_name);
        self
    }

    pub fn application_name(&self) -> Option<&::models::ApplicationName> {
        self.application_name.as_ref()
    }

    pub fn reset_application_name(&mut self) {
        self.application_name = None;
    }

    pub fn set_service_name(&mut self, service_name: ::models::ServiceName) {
        self.service_name = service_name;
    }

    pub fn with_service_name(
        mut self,
        service_name: ::models::ServiceName,
    ) -> ServiceDescription {
        self.service_name = service_name;
        self
    }

    pub fn service_name(&self) -> &::models::ServiceName {
        &self.service_name
    }

    pub fn set_service_type_name(
        &mut self,
        service_type_name: ::models::ServiceTypeName,
    ) {
        self.service_type_name = service_type_name;
    }

    pub fn with_service_type_name(
        mut self,
        service_type_name: ::models::ServiceTypeName,
    ) -> ServiceDescription {
        self.service_type_name = service_type_name;
        self
    }

    pub fn service_type_name(&self) -> &::models::ServiceTypeName {
        &self.service_type_name
    }

    pub fn set_initialization_data(&mut self, initialization_data: String) {
        self.initialization_data = Some(initialization_data);
    }

    pub fn with_initialization_data(
        mut self,
        initialization_data: String,
    ) -> ServiceDescription {
        self.initialization_data = Some(initialization_data);
        self
    }

    pub fn initialization_data(&self) -> Option<&String> {
        self.initialization_data.as_ref()
    }

    pub fn reset_initialization_data(&mut self) {
        self.initialization_data = None;
    }

    pub fn set_partition_description(
        &mut self,
        partition_description: ::models::PartitionSchemeDescription,
    ) {
        self.partition_description = partition_description;
    }

    pub fn with_partition_description(
        mut self,
        partition_description: ::models::PartitionSchemeDescription,
    ) -> ServiceDescription {
        self.partition_description = partition_description;
        self
    }

    pub fn partition_description(
        &self,
    ) -> &::models::PartitionSchemeDescription {
        &self.partition_description
    }

    pub fn set_placement_constraints(&mut self, placement_constraints: String) {
        self.placement_constraints = Some(placement_constraints);
    }

    pub fn with_placement_constraints(
        mut self,
        placement_constraints: String,
    ) -> ServiceDescription {
        self.placement_constraints = Some(placement_constraints);
        self
    }

    pub fn placement_constraints(&self) -> Option<&String> {
        self.placement_constraints.as_ref()
    }

    pub fn reset_placement_constraints(&mut self) {
        self.placement_constraints = None;
    }

    pub fn set_correlation_scheme(
        &mut self,
        correlation_scheme: ::models::CorrelationSchemeList,
    ) {
        self.correlation_scheme = Some(correlation_scheme);
    }

    pub fn with_correlation_scheme(
        mut self,
        correlation_scheme: ::models::CorrelationSchemeList,
    ) -> ServiceDescription {
        self.correlation_scheme = Some(correlation_scheme);
        self
    }

    pub fn correlation_scheme(
        &self,
    ) -> Option<&::models::CorrelationSchemeList> {
        self.correlation_scheme.as_ref()
    }

    pub fn reset_correlation_scheme(&mut self) {
        self.correlation_scheme = None;
    }

    pub fn set_service_load_metrics(
        &mut self,
        service_load_metrics: ::models::ServiceLoadMetricsList,
    ) {
        self.service_load_metrics = Some(service_load_metrics);
    }

    pub fn with_service_load_metrics(
        mut self,
        service_load_metrics: ::models::ServiceLoadMetricsList,
    ) -> ServiceDescription {
        self.service_load_metrics = Some(service_load_metrics);
        self
    }

    pub fn service_load_metrics(
        &self,
    ) -> Option<&::models::ServiceLoadMetricsList> {
        self.service_load_metrics.as_ref()
    }

    pub fn reset_service_load_metrics(&mut self) {
        self.service_load_metrics = None;
    }

    pub fn set_service_placement_policies(
        &mut self,
        service_placement_policies: ::models::ServicePlacementPoliciesList,
    ) {
        self.service_placement_policies = Some(service_placement_policies);
    }

    pub fn with_service_placement_policies(
        mut self,
        service_placement_policies: ::models::ServicePlacementPoliciesList,
    ) -> ServiceDescription {
        self.service_placement_policies = Some(service_placement_policies);
        self
    }

    pub fn service_placement_policies(
        &self,
    ) -> Option<&::models::ServicePlacementPoliciesList> {
        self.service_placement_policies.as_ref()
    }

    pub fn reset_service_placement_policies(&mut self) {
        self.service_placement_policies = None;
    }

    pub fn set_default_move_cost(
        &mut self,
        default_move_cost: ::models::MoveCost,
    ) {
        self.default_move_cost = Some(default_move_cost);
    }

    pub fn with_default_move_cost(
        mut self,
        default_move_cost: ::models::MoveCost,
    ) -> ServiceDescription {
        self.default_move_cost = Some(default_move_cost);
        self
    }

    pub fn default_move_cost(&self) -> Option<&::models::MoveCost> {
        self.default_move_cost.as_ref()
    }

    pub fn reset_default_move_cost(&mut self) {
        self.default_move_cost = None;
    }

    pub fn set_is_default_move_cost_specified(
        &mut self,
        is_default_move_cost_specified: bool,
    ) {
        self.is_default_move_cost_specified =
            Some(is_default_move_cost_specified);
    }

    pub fn with_is_default_move_cost_specified(
        mut self,
        is_default_move_cost_specified: bool,
    ) -> ServiceDescription {
        self.is_default_move_cost_specified =
            Some(is_default_move_cost_specified);
        self
    }

    pub fn is_default_move_cost_specified(&self) -> Option<&bool> {
        self.is_default_move_cost_specified.as_ref()
    }

    pub fn reset_is_default_move_cost_specified(&mut self) {
        self.is_default_move_cost_specified = None;
    }

    pub fn set_service_package_activation_mode(
        &mut self,
        service_package_activation_mode: ::models::ServicePackageActivationMode,
    ) {
        self.service_package_activation_mode =
            Some(service_package_activation_mode);
    }

    pub fn with_service_package_activation_mode(
        mut self,
        service_package_activation_mode: ::models::ServicePackageActivationMode,
    ) -> ServiceDescription {
        self.service_package_activation_mode =
            Some(service_package_activation_mode);
        self
    }

    pub fn service_package_activation_mode(
        &self,
    ) -> Option<&::models::ServicePackageActivationMode> {
        self.service_package_activation_mode.as_ref()
    }

    pub fn reset_service_package_activation_mode(&mut self) {
        self.service_package_activation_mode = None;
    }

    pub fn set_service_dns_name(&mut self, service_dns_name: String) {
        self.service_dns_name = Some(service_dns_name);
    }

    pub fn with_service_dns_name(
        mut self,
        service_dns_name: String,
    ) -> ServiceDescription {
        self.service_dns_name = Some(service_dns_name);
        self
    }

    pub fn service_dns_name(&self) -> Option<&String> {
        self.service_dns_name.as_ref()
    }

    pub fn reset_service_dns_name(&mut self) {
        self.service_dns_name = None;
    }

    pub fn set_scaling_policies(
        &mut self,
        scaling_policies: ::models::ScalingPolicyDescriptionList,
    ) {
        self.scaling_policies = Some(scaling_policies);
    }

    pub fn with_scaling_policies(
        mut self,
        scaling_policies: ::models::ScalingPolicyDescriptionList,
    ) -> ServiceDescription {
        self.scaling_policies = Some(scaling_policies);
        self
    }

    pub fn scaling_policies(
        &self,
    ) -> Option<&::models::ScalingPolicyDescriptionList> {
        self.scaling_policies.as_ref()
    }

    pub fn reset_scaling_policies(&mut self) {
        self.scaling_policies = None;
    }
}
