/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatelessServiceDescription : Describes a stateless service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatelessServiceDescription {
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
    /// The instance count.
    #[serde(rename = "InstanceCount")]
    instance_count: i32,
    /// MinInstanceCount is the minimum number of instances that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node. The actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ). Note, if InstanceCount is set to -1, during MinInstanceCount computation -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service.
    #[serde(rename = "MinInstanceCount")]
    min_instance_count: Option<::models::MinInstanceCount>,
    /// MinInstancePercentage is the minimum percentage of InstanceCount that must be up to meet the EnsureAvailability safety check during operations like upgrade or deactivate node. The actual number that is used is max( MinInstanceCount, ceil( MinInstancePercentage/100.0 * InstanceCount) ). Note, if InstanceCount is set to -1, during MinInstancePercentage computation, -1 is first converted into the number of nodes on which the instances are allowed to be placed according to the placement constraints on the service.
    #[serde(rename = "MinInstancePercentage")]
    min_instance_percentage: Option<::models::MinInstancePercentage>,
    /// Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified. This property can be a combination of those flags obtained using bitwise 'OR' operator. For example, if the provided value is 1 then the flags for InstanceCloseDelayDuration is set.  - None - Does not indicate any other properties are set. The value is zero. - InstanceCloseDelayDuration - Indicates the InstanceCloseDelayDuration property is set. The value is 1.
    #[serde(rename = "Flags")]
    flags: Option<i32>,
    /// Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster upgrade and disabling node. The endpoint exposed on this instance is removed prior to starting the delay, which prevents new connections to this instance. In addition, clients that have subscribed to service endpoint change events(https://docs.microsoft.com/en-us/dotnet/api/system.fabric.fabricclient.servicemanagementclient.registerservicenotificationfilter_async), can do the following upon receiving the endpoint removal notification:     - Stop sending new requests to this instance.     - Close existing connections after in-flight requests have completed.     - Connect to a different instance of the service partition for future requests. Note, the default value of InstanceCloseDelayDuration is 0, which indicates that there won't be any delay or removal of the endpoint prior to closing the instance.
    #[serde(rename = "InstanceCloseDelayDurationSeconds")]
    instance_close_delay_duration_seconds: Option<i64>,
}

impl StatelessServiceDescription {
    /// Describes a stateless service.
    pub fn new(
        service_kind: ::models::ServiceKind,
        service_name: ::models::ServiceName,
        service_type_name: ::models::ServiceTypeName,
        partition_description: ::models::PartitionSchemeDescription,
        instance_count: i32,
    ) -> StatelessServiceDescription {
        StatelessServiceDescription {
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
            instance_count,
            min_instance_count: None,
            min_instance_percentage: None,
            flags: None,
            instance_close_delay_duration_seconds: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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
    ) -> StatelessServiceDescription {
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

    pub fn set_instance_count(&mut self, instance_count: i32) {
        self.instance_count = instance_count;
    }

    pub fn with_instance_count(
        mut self,
        instance_count: i32,
    ) -> StatelessServiceDescription {
        self.instance_count = instance_count;
        self
    }

    pub fn instance_count(&self) -> &i32 {
        &self.instance_count
    }

    pub fn set_min_instance_count(
        &mut self,
        min_instance_count: ::models::MinInstanceCount,
    ) {
        self.min_instance_count = Some(min_instance_count);
    }

    pub fn with_min_instance_count(
        mut self,
        min_instance_count: ::models::MinInstanceCount,
    ) -> StatelessServiceDescription {
        self.min_instance_count = Some(min_instance_count);
        self
    }

    pub fn min_instance_count(&self) -> Option<&::models::MinInstanceCount> {
        self.min_instance_count.as_ref()
    }

    pub fn reset_min_instance_count(&mut self) {
        self.min_instance_count = None;
    }

    pub fn set_min_instance_percentage(
        &mut self,
        min_instance_percentage: ::models::MinInstancePercentage,
    ) {
        self.min_instance_percentage = Some(min_instance_percentage);
    }

    pub fn with_min_instance_percentage(
        mut self,
        min_instance_percentage: ::models::MinInstancePercentage,
    ) -> StatelessServiceDescription {
        self.min_instance_percentage = Some(min_instance_percentage);
        self
    }

    pub fn min_instance_percentage(
        &self,
    ) -> Option<&::models::MinInstancePercentage> {
        self.min_instance_percentage.as_ref()
    }

    pub fn reset_min_instance_percentage(&mut self) {
        self.min_instance_percentage = None;
    }

    pub fn set_flags(&mut self, flags: i32) {
        self.flags = Some(flags);
    }

    pub fn with_flags(mut self, flags: i32) -> StatelessServiceDescription {
        self.flags = Some(flags);
        self
    }

    pub fn flags(&self) -> Option<&i32> {
        self.flags.as_ref()
    }

    pub fn reset_flags(&mut self) {
        self.flags = None;
    }

    pub fn set_instance_close_delay_duration_seconds(
        &mut self,
        instance_close_delay_duration_seconds: i64,
    ) {
        self.instance_close_delay_duration_seconds =
            Some(instance_close_delay_duration_seconds);
    }

    pub fn with_instance_close_delay_duration_seconds(
        mut self,
        instance_close_delay_duration_seconds: i64,
    ) -> StatelessServiceDescription {
        self.instance_close_delay_duration_seconds =
            Some(instance_close_delay_duration_seconds);
        self
    }

    pub fn instance_close_delay_duration_seconds(&self) -> Option<&i64> {
        self.instance_close_delay_duration_seconds.as_ref()
    }

    pub fn reset_instance_close_delay_duration_seconds(&mut self) {
        self.instance_close_delay_duration_seconds = None;
    }
}
