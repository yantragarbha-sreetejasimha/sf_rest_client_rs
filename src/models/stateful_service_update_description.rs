/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatefulServiceUpdateDescription : Describes an update for a stateful service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulServiceUpdateDescription {
    /// The service kind.
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified. This property can be a combination of those flags obtained using bitwise 'OR' operator. For example, if the provided value is 6 then the flags for ReplicaRestartWaitDuration (2) and QuorumLossWaitDuration (4) are set.  - None - Does not indicate any other properties are set. The value is zero. - TargetReplicaSetSize/InstanceCount - Indicates whether the TargetReplicaSetSize property (for Stateful services) or the InstanceCount property (for Stateless services) is set. The value is 1. - ReplicaRestartWaitDuration - Indicates the ReplicaRestartWaitDuration property is set. The value is  2. - QuorumLossWaitDuration - Indicates the QuorumLossWaitDuration property is set. The value is 4. - StandByReplicaKeepDuration - Indicates the StandByReplicaKeepDuration property is set. The value is 8. - MinReplicaSetSize - Indicates the MinReplicaSetSize property is set. The value is 16. - PlacementConstraints - Indicates the PlacementConstraints property is set. The value is 32. - PlacementPolicyList - Indicates the ServicePlacementPolicies property is set. The value is 64. - Correlation - Indicates the CorrelationScheme property is set. The value is 128. - Metrics - Indicates the ServiceLoadMetrics property is set. The value is 256. - DefaultMoveCost - Indicates the DefaultMoveCost property is set. The value is 512. - ScalingPolicy - Indicates the ScalingPolicies property is set. The value is 1024. - ServicePlacementTimeLimit - Indicates the ServicePlacementTimeLimit property is set. The value is 2048. - MinInstanceCount - Indicates the MinInstanceCount property is set. The value is 4096. - MinInstancePercentage - Indicates the MinInstancePercentage property is set. The value is 8192. - InstanceCloseDelayDuration - Indicates the InstanceCloseDelayDuration property is set. The value is 16384.
    #[serde(rename = "Flags")]
    flags: Option<String>,
    /// The placement constraints as a string. Placement constraints are boolean expressions on node properties and allow for restricting a service to particular nodes based on the service requirements. For example, to place a service on nodes where NodeType is blue specify the following: \"NodeColor == blue)\".
    #[serde(rename = "PlacementConstraints")]
    placement_constraints: Option<String>,
    /// The correlation scheme.
    #[serde(rename = "CorrelationScheme")]
    correlation_scheme: Option<::models::CorrelationSchemeList>,
    /// The service load metrics.
    #[serde(rename = "LoadMetrics")]
    load_metrics: Option<::models::ServiceLoadMetricsList>,
    /// The service placement policies.
    #[serde(rename = "ServicePlacementPolicies")]
    service_placement_policies: Option<::models::ServicePlacementPoliciesList>,
    /// The move cost for the service.
    #[serde(rename = "DefaultMoveCost")]
    default_move_cost: Option<::models::MoveCost>,
    /// Scaling policies for this service.
    #[serde(rename = "ScalingPolicies")]
    scaling_policies: Option<::models::ScalingPolicyDescriptionList>,
    /// The target replica set size as a number.
    #[serde(rename = "TargetReplicaSetSize")]
    target_replica_set_size: Option<i32>,
    /// The minimum replica set size as a number.
    #[serde(rename = "MinReplicaSetSize")]
    min_replica_set_size: Option<i32>,
    /// The duration, in seconds, between when a replica goes down and when a new replica is created.
    #[serde(rename = "ReplicaRestartWaitDurationSeconds")]
    replica_restart_wait_duration_seconds: Option<String>,
    /// The maximum duration, in seconds, for which a partition is allowed to be in a state of quorum loss.
    #[serde(rename = "QuorumLossWaitDurationSeconds")]
    quorum_loss_wait_duration_seconds: Option<String>,
    /// The definition on how long StandBy replicas should be maintained before being removed.
    #[serde(rename = "StandByReplicaKeepDurationSeconds")]
    stand_by_replica_keep_duration_seconds: Option<String>,
    /// The duration for which replicas can stay InBuild before reporting that build is stuck.
    #[serde(rename = "ServicePlacementTimeLimitSeconds")]
    service_placement_time_limit_seconds: Option<String>,
}

impl StatefulServiceUpdateDescription {
    /// Describes an update for a stateful service.
    pub fn new(
        service_kind: ::models::ServiceKind,
    ) -> StatefulServiceUpdateDescription {
        StatefulServiceUpdateDescription {
            service_kind,
            flags: None,
            placement_constraints: None,
            correlation_scheme: None,
            load_metrics: None,
            service_placement_policies: None,
            default_move_cost: None,
            scaling_policies: None,
            target_replica_set_size: None,
            min_replica_set_size: None,
            replica_restart_wait_duration_seconds: None,
            quorum_loss_wait_duration_seconds: None,
            stand_by_replica_keep_duration_seconds: None,
            service_placement_time_limit_seconds: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatefulServiceUpdateDescription {
        self.service_kind = service_kind;
        self
    }

    pub fn service_kind(&self) -> &::models::ServiceKind {
        &self.service_kind
    }

    pub fn set_flags(&mut self, flags: String) {
        self.flags = Some(flags);
    }

    pub fn with_flags(
        mut self,
        flags: String,
    ) -> StatefulServiceUpdateDescription {
        self.flags = Some(flags);
        self
    }

    pub fn flags(&self) -> Option<&String> {
        self.flags.as_ref()
    }

    pub fn reset_flags(&mut self) {
        self.flags = None;
    }

    pub fn set_placement_constraints(&mut self, placement_constraints: String) {
        self.placement_constraints = Some(placement_constraints);
    }

    pub fn with_placement_constraints(
        mut self,
        placement_constraints: String,
    ) -> StatefulServiceUpdateDescription {
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
    ) -> StatefulServiceUpdateDescription {
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

    pub fn set_load_metrics(
        &mut self,
        load_metrics: ::models::ServiceLoadMetricsList,
    ) {
        self.load_metrics = Some(load_metrics);
    }

    pub fn with_load_metrics(
        mut self,
        load_metrics: ::models::ServiceLoadMetricsList,
    ) -> StatefulServiceUpdateDescription {
        self.load_metrics = Some(load_metrics);
        self
    }

    pub fn load_metrics(&self) -> Option<&::models::ServiceLoadMetricsList> {
        self.load_metrics.as_ref()
    }

    pub fn reset_load_metrics(&mut self) {
        self.load_metrics = None;
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
    ) -> StatefulServiceUpdateDescription {
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
    ) -> StatefulServiceUpdateDescription {
        self.default_move_cost = Some(default_move_cost);
        self
    }

    pub fn default_move_cost(&self) -> Option<&::models::MoveCost> {
        self.default_move_cost.as_ref()
    }

    pub fn reset_default_move_cost(&mut self) {
        self.default_move_cost = None;
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
    ) -> StatefulServiceUpdateDescription {
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
        self.target_replica_set_size = Some(target_replica_set_size);
    }

    pub fn with_target_replica_set_size(
        mut self,
        target_replica_set_size: i32,
    ) -> StatefulServiceUpdateDescription {
        self.target_replica_set_size = Some(target_replica_set_size);
        self
    }

    pub fn target_replica_set_size(&self) -> Option<&i32> {
        self.target_replica_set_size.as_ref()
    }

    pub fn reset_target_replica_set_size(&mut self) {
        self.target_replica_set_size = None;
    }

    pub fn set_min_replica_set_size(&mut self, min_replica_set_size: i32) {
        self.min_replica_set_size = Some(min_replica_set_size);
    }

    pub fn with_min_replica_set_size(
        mut self,
        min_replica_set_size: i32,
    ) -> StatefulServiceUpdateDescription {
        self.min_replica_set_size = Some(min_replica_set_size);
        self
    }

    pub fn min_replica_set_size(&self) -> Option<&i32> {
        self.min_replica_set_size.as_ref()
    }

    pub fn reset_min_replica_set_size(&mut self) {
        self.min_replica_set_size = None;
    }

    pub fn set_replica_restart_wait_duration_seconds(
        &mut self,
        replica_restart_wait_duration_seconds: String,
    ) {
        self.replica_restart_wait_duration_seconds =
            Some(replica_restart_wait_duration_seconds);
    }

    pub fn with_replica_restart_wait_duration_seconds(
        mut self,
        replica_restart_wait_duration_seconds: String,
    ) -> StatefulServiceUpdateDescription {
        self.replica_restart_wait_duration_seconds =
            Some(replica_restart_wait_duration_seconds);
        self
    }

    pub fn replica_restart_wait_duration_seconds(&self) -> Option<&String> {
        self.replica_restart_wait_duration_seconds.as_ref()
    }

    pub fn reset_replica_restart_wait_duration_seconds(&mut self) {
        self.replica_restart_wait_duration_seconds = None;
    }

    pub fn set_quorum_loss_wait_duration_seconds(
        &mut self,
        quorum_loss_wait_duration_seconds: String,
    ) {
        self.quorum_loss_wait_duration_seconds =
            Some(quorum_loss_wait_duration_seconds);
    }

    pub fn with_quorum_loss_wait_duration_seconds(
        mut self,
        quorum_loss_wait_duration_seconds: String,
    ) -> StatefulServiceUpdateDescription {
        self.quorum_loss_wait_duration_seconds =
            Some(quorum_loss_wait_duration_seconds);
        self
    }

    pub fn quorum_loss_wait_duration_seconds(&self) -> Option<&String> {
        self.quorum_loss_wait_duration_seconds.as_ref()
    }

    pub fn reset_quorum_loss_wait_duration_seconds(&mut self) {
        self.quorum_loss_wait_duration_seconds = None;
    }

    pub fn set_stand_by_replica_keep_duration_seconds(
        &mut self,
        stand_by_replica_keep_duration_seconds: String,
    ) {
        self.stand_by_replica_keep_duration_seconds =
            Some(stand_by_replica_keep_duration_seconds);
    }

    pub fn with_stand_by_replica_keep_duration_seconds(
        mut self,
        stand_by_replica_keep_duration_seconds: String,
    ) -> StatefulServiceUpdateDescription {
        self.stand_by_replica_keep_duration_seconds =
            Some(stand_by_replica_keep_duration_seconds);
        self
    }

    pub fn stand_by_replica_keep_duration_seconds(&self) -> Option<&String> {
        self.stand_by_replica_keep_duration_seconds.as_ref()
    }

    pub fn reset_stand_by_replica_keep_duration_seconds(&mut self) {
        self.stand_by_replica_keep_duration_seconds = None;
    }

    pub fn set_service_placement_time_limit_seconds(
        &mut self,
        service_placement_time_limit_seconds: String,
    ) {
        self.service_placement_time_limit_seconds =
            Some(service_placement_time_limit_seconds);
    }

    pub fn with_service_placement_time_limit_seconds(
        mut self,
        service_placement_time_limit_seconds: String,
    ) -> StatefulServiceUpdateDescription {
        self.service_placement_time_limit_seconds =
            Some(service_placement_time_limit_seconds);
        self
    }

    pub fn service_placement_time_limit_seconds(&self) -> Option<&String> {
        self.service_placement_time_limit_seconds.as_ref()
    }

    pub fn reset_service_placement_time_limit_seconds(&mut self) {
        self.service_placement_time_limit_seconds = None;
    }
}
