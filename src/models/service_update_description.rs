/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceUpdateDescription : A ServiceUpdateDescription contains all of the information necessary to update a service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceUpdateDescription {
    /// The service kind.
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified. This property can be a combination of those flags obtained using bitwise 'OR' operator. For example, if the provided value is 6 then the flags for ReplicaRestartWaitDuration (2) and QuorumLossWaitDuration (4) are set.  - None - Does not indicate any other properties are set. The value is zero. - TargetReplicaSetSize/InstanceCount - Indicates whether the TargetReplicaSetSize property (for Stateful services) or the InstanceCount property (for Stateless services) is set. The value is 1. - ReplicaRestartWaitDuration - Indicates the ReplicaRestartWaitDuration property is set. The value is  2. - QuorumLossWaitDuration - Indicates the QuorumLossWaitDuration property is set. The value is 4. - StandByReplicaKeepDuration - Indicates the StandByReplicaKeepDuration property is set. The value is 8. - MinReplicaSetSize - Indicates the MinReplicaSetSize property is set. The value is 16. - PlacementConstraints - Indicates the PlacementConstraints property is set. The value is 32. - PlacementPolicyList - Indicates the ServicePlacementPolicies property is set. The value is 64. - Correlation - Indicates the CorrelationScheme property is set. The value is 128. - Metrics - Indicates the ServiceLoadMetrics property is set. The value is 256. - DefaultMoveCost - Indicates the DefaultMoveCost property is set. The value is 512. - ScalingPolicy - Indicates the ScalingPolicies property is set. The value is 1024. - ServicePlacementTimeLimit - Indicates the ServicePlacementTimeLimit property is set. The value is 2048. - MinInstanceCount - Indicates the MinInstanceCount property is set. The value is 4096. - MinInstancePercentage - Indicates the MinInstancePercentage property is set. The value is 8192. - InstanceCloseDelayDuration - Indicates the InstanceCloseDelayDuration property is set. The value is 16384. - InstanceRestartWaitDuration - Indicates the InstanceCloseDelayDuration property is set. The value is 32768. - DropSourceReplicaOnMove - Indicates the DropSourceReplicaOnMove property is set. The value is 65536. - ServiceDnsName - Indicates the ServiceDnsName property is set. The value is 131072. - TagsForPlacement - Indicates the TagsForPlacement property is set. The value is 1048576. - TagsForRunning - Indicates the TagsForRunning property is set. The value is 2097152.
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
    /// The DNS name of the service.
    #[serde(rename = "ServiceDnsName")]
    service_dns_name: Option<String>,
    /// Tags for placement of this service.
    #[serde(rename = "TagsForPlacement")]
    tags_for_placement: Option<::models::NodeTagsDescription>,
    /// Tags for running of this service.
    #[serde(rename = "TagsForRunning")]
    tags_for_running: Option<::models::NodeTagsDescription>,
}

impl ServiceUpdateDescription {
    /// A ServiceUpdateDescription contains all of the information necessary to update a service.
    pub fn new(
        service_kind: ::models::ServiceKind,
    ) -> ServiceUpdateDescription {
        ServiceUpdateDescription {
            service_kind,
            flags: None,
            placement_constraints: None,
            correlation_scheme: None,
            load_metrics: None,
            service_placement_policies: None,
            default_move_cost: None,
            scaling_policies: None,
            service_dns_name: None,
            tags_for_placement: None,
            tags_for_running: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> ServiceUpdateDescription {
        self.service_kind = service_kind;
        self
    }

    pub fn service_kind(&self) -> &::models::ServiceKind {
        &self.service_kind
    }

    pub fn set_flags(&mut self, flags: String) {
        self.flags = Some(flags);
    }

    pub fn with_flags(mut self, flags: String) -> ServiceUpdateDescription {
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
    ) -> ServiceUpdateDescription {
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
    ) -> ServiceUpdateDescription {
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
    ) -> ServiceUpdateDescription {
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
    ) -> ServiceUpdateDescription {
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
    ) -> ServiceUpdateDescription {
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
    ) -> ServiceUpdateDescription {
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

    pub fn set_service_dns_name(&mut self, service_dns_name: String) {
        self.service_dns_name = Some(service_dns_name);
    }

    pub fn with_service_dns_name(
        mut self,
        service_dns_name: String,
    ) -> ServiceUpdateDescription {
        self.service_dns_name = Some(service_dns_name);
        self
    }

    pub fn service_dns_name(&self) -> Option<&String> {
        self.service_dns_name.as_ref()
    }

    pub fn reset_service_dns_name(&mut self) {
        self.service_dns_name = None;
    }

    pub fn set_tags_for_placement(
        &mut self,
        tags_for_placement: ::models::NodeTagsDescription,
    ) {
        self.tags_for_placement = Some(tags_for_placement);
    }

    pub fn with_tags_for_placement(
        mut self,
        tags_for_placement: ::models::NodeTagsDescription,
    ) -> ServiceUpdateDescription {
        self.tags_for_placement = Some(tags_for_placement);
        self
    }

    pub fn tags_for_placement(&self) -> Option<&::models::NodeTagsDescription> {
        self.tags_for_placement.as_ref()
    }

    pub fn reset_tags_for_placement(&mut self) {
        self.tags_for_placement = None;
    }

    pub fn set_tags_for_running(
        &mut self,
        tags_for_running: ::models::NodeTagsDescription,
    ) {
        self.tags_for_running = Some(tags_for_running);
    }

    pub fn with_tags_for_running(
        mut self,
        tags_for_running: ::models::NodeTagsDescription,
    ) -> ServiceUpdateDescription {
        self.tags_for_running = Some(tags_for_running);
        self
    }

    pub fn tags_for_running(&self) -> Option<&::models::NodeTagsDescription> {
        self.tags_for_running.as_ref()
    }

    pub fn reset_tags_for_running(&mut self) {
        self.tags_for_running = None;
    }
}