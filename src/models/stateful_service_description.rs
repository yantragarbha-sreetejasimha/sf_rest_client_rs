/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatefulServiceDescription : Describes a stateful service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulServiceDescription {
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
    /// The target replica set size as a number.
    #[serde(rename = "TargetReplicaSetSize")]
    target_replica_set_size: i32,
    /// The minimum replica set size as a number.
    #[serde(rename = "MinReplicaSetSize")]
    min_replica_set_size: i32,
    /// A flag indicating whether this is a persistent service which stores states on the local disk. If it is then the value of this property is true, if not it is false.
    #[serde(rename = "HasPersistedState")]
    has_persisted_state: bool,
    /// Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified. This property can be a combination of those flags obtained using bitwise 'OR' operator. For example, if the provided value is 6 then the flags for QuorumLossWaitDuration (2) and StandByReplicaKeepDuration(4) are set.  - None - Does not indicate any other properties are set. The value is zero. - ReplicaRestartWaitDuration - Indicates the ReplicaRestartWaitDuration property is set. The value is 1. - QuorumLossWaitDuration - Indicates the QuorumLossWaitDuration property is set. The value is 2. - StandByReplicaKeepDuration - Indicates the StandByReplicaKeepDuration property is set. The value is 4. - ServicePlacementTimeLimit - Indicates the ServicePlacementTimeLimit property is set. The value is 8. - DropSourceReplicaOnMove - Indicates the DropSourceReplicaOnMove property is set. The value is 16.
    #[serde(rename = "Flags")]
    flags: Option<i32>,
    /// The duration, in seconds, between when a replica goes down and when a new replica is created.
    #[serde(rename = "ReplicaRestartWaitDurationSeconds")]
    replica_restart_wait_duration_seconds: Option<i64>,
    /// The maximum duration, in seconds, for which a partition is allowed to be in a state of quorum loss.
    #[serde(rename = "QuorumLossWaitDurationSeconds")]
    quorum_loss_wait_duration_seconds: Option<i64>,
    /// The definition on how long StandBy replicas should be maintained before being removed.
    #[serde(rename = "StandByReplicaKeepDurationSeconds")]
    stand_by_replica_keep_duration_seconds: Option<i64>,
    /// The duration for which replicas can stay InBuild before reporting that build is stuck.
    #[serde(rename = "ServicePlacementTimeLimitSeconds")]
    service_placement_time_limit_seconds: Option<i64>,
    /// Indicates whether to drop source Secondary replica even if the target replica has not finished build. If desired behavior is to drop it as soon as possible the value of this property is true, if not it is false.
    #[serde(rename = "DropSourceReplicaOnMove")]
    drop_source_replica_on_move: Option<bool>,
}

impl StatefulServiceDescription {
    /// Describes a stateful service.
    pub fn new(
        service_kind: ::models::ServiceKind,
        service_name: ::models::ServiceName,
        service_type_name: ::models::ServiceTypeName,
        partition_description: ::models::PartitionSchemeDescription,
        target_replica_set_size: i32,
        min_replica_set_size: i32,
        has_persisted_state: bool,
    ) -> StatefulServiceDescription {
        StatefulServiceDescription {
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
            target_replica_set_size,
            min_replica_set_size,
            has_persisted_state,
            flags: None,
            replica_restart_wait_duration_seconds: None,
            quorum_loss_wait_duration_seconds: None,
            stand_by_replica_keep_duration_seconds: None,
            service_placement_time_limit_seconds: None,
            drop_source_replica_on_move: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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
    ) -> StatefulServiceDescription {
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

    pub fn set_target_replica_set_size(
        &mut self,
        target_replica_set_size: i32,
    ) {
        self.target_replica_set_size = target_replica_set_size;
    }

    pub fn with_target_replica_set_size(
        mut self,
        target_replica_set_size: i32,
    ) -> StatefulServiceDescription {
        self.target_replica_set_size = target_replica_set_size;
        self
    }

    pub fn target_replica_set_size(&self) -> &i32 {
        &self.target_replica_set_size
    }

    pub fn set_min_replica_set_size(&mut self, min_replica_set_size: i32) {
        self.min_replica_set_size = min_replica_set_size;
    }

    pub fn with_min_replica_set_size(
        mut self,
        min_replica_set_size: i32,
    ) -> StatefulServiceDescription {
        self.min_replica_set_size = min_replica_set_size;
        self
    }

    pub fn min_replica_set_size(&self) -> &i32 {
        &self.min_replica_set_size
    }

    pub fn set_has_persisted_state(&mut self, has_persisted_state: bool) {
        self.has_persisted_state = has_persisted_state;
    }

    pub fn with_has_persisted_state(
        mut self,
        has_persisted_state: bool,
    ) -> StatefulServiceDescription {
        self.has_persisted_state = has_persisted_state;
        self
    }

    pub fn has_persisted_state(&self) -> &bool {
        &self.has_persisted_state
    }

    pub fn set_flags(&mut self, flags: i32) {
        self.flags = Some(flags);
    }

    pub fn with_flags(mut self, flags: i32) -> StatefulServiceDescription {
        self.flags = Some(flags);
        self
    }

    pub fn flags(&self) -> Option<&i32> {
        self.flags.as_ref()
    }

    pub fn reset_flags(&mut self) {
        self.flags = None;
    }

    pub fn set_replica_restart_wait_duration_seconds(
        &mut self,
        replica_restart_wait_duration_seconds: i64,
    ) {
        self.replica_restart_wait_duration_seconds =
            Some(replica_restart_wait_duration_seconds);
    }

    pub fn with_replica_restart_wait_duration_seconds(
        mut self,
        replica_restart_wait_duration_seconds: i64,
    ) -> StatefulServiceDescription {
        self.replica_restart_wait_duration_seconds =
            Some(replica_restart_wait_duration_seconds);
        self
    }

    pub fn replica_restart_wait_duration_seconds(&self) -> Option<&i64> {
        self.replica_restart_wait_duration_seconds.as_ref()
    }

    pub fn reset_replica_restart_wait_duration_seconds(&mut self) {
        self.replica_restart_wait_duration_seconds = None;
    }

    pub fn set_quorum_loss_wait_duration_seconds(
        &mut self,
        quorum_loss_wait_duration_seconds: i64,
    ) {
        self.quorum_loss_wait_duration_seconds =
            Some(quorum_loss_wait_duration_seconds);
    }

    pub fn with_quorum_loss_wait_duration_seconds(
        mut self,
        quorum_loss_wait_duration_seconds: i64,
    ) -> StatefulServiceDescription {
        self.quorum_loss_wait_duration_seconds =
            Some(quorum_loss_wait_duration_seconds);
        self
    }

    pub fn quorum_loss_wait_duration_seconds(&self) -> Option<&i64> {
        self.quorum_loss_wait_duration_seconds.as_ref()
    }

    pub fn reset_quorum_loss_wait_duration_seconds(&mut self) {
        self.quorum_loss_wait_duration_seconds = None;
    }

    pub fn set_stand_by_replica_keep_duration_seconds(
        &mut self,
        stand_by_replica_keep_duration_seconds: i64,
    ) {
        self.stand_by_replica_keep_duration_seconds =
            Some(stand_by_replica_keep_duration_seconds);
    }

    pub fn with_stand_by_replica_keep_duration_seconds(
        mut self,
        stand_by_replica_keep_duration_seconds: i64,
    ) -> StatefulServiceDescription {
        self.stand_by_replica_keep_duration_seconds =
            Some(stand_by_replica_keep_duration_seconds);
        self
    }

    pub fn stand_by_replica_keep_duration_seconds(&self) -> Option<&i64> {
        self.stand_by_replica_keep_duration_seconds.as_ref()
    }

    pub fn reset_stand_by_replica_keep_duration_seconds(&mut self) {
        self.stand_by_replica_keep_duration_seconds = None;
    }

    pub fn set_service_placement_time_limit_seconds(
        &mut self,
        service_placement_time_limit_seconds: i64,
    ) {
        self.service_placement_time_limit_seconds =
            Some(service_placement_time_limit_seconds);
    }

    pub fn with_service_placement_time_limit_seconds(
        mut self,
        service_placement_time_limit_seconds: i64,
    ) -> StatefulServiceDescription {
        self.service_placement_time_limit_seconds =
            Some(service_placement_time_limit_seconds);
        self
    }

    pub fn service_placement_time_limit_seconds(&self) -> Option<&i64> {
        self.service_placement_time_limit_seconds.as_ref()
    }

    pub fn reset_service_placement_time_limit_seconds(&mut self) {
        self.service_placement_time_limit_seconds = None;
    }

    pub fn set_drop_source_replica_on_move(
        &mut self,
        drop_source_replica_on_move: bool,
    ) {
        self.drop_source_replica_on_move = Some(drop_source_replica_on_move);
    }

    pub fn with_drop_source_replica_on_move(
        mut self,
        drop_source_replica_on_move: bool,
    ) -> StatefulServiceDescription {
        self.drop_source_replica_on_move = Some(drop_source_replica_on_move);
        self
    }

    pub fn drop_source_replica_on_move(&self) -> Option<&bool> {
        self.drop_source_replica_on_move.as_ref()
    }

    pub fn reset_drop_source_replica_on_move(&mut self) {
        self.drop_source_replica_on_move = None;
    }
}
