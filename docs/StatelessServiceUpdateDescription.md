# StatelessServiceUpdateDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The service kind. | [default to null]
**flags** | **String** | Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified. This property can be a combination of those flags obtained using bitwise &#39;OR&#39; operator. For example, if the provided value is 6 then the flags for ReplicaRestartWaitDuration (2) and QuorumLossWaitDuration (4) are set.  - None - Does not indicate any other properties are set. The value is zero. - TargetReplicaSetSize/InstanceCount - Indicates whether the TargetReplicaSetSize property (for Stateful services) or the InstanceCount property (for Stateless services) is set. The value is 1. - ReplicaRestartWaitDuration - Indicates the ReplicaRestartWaitDuration property is set. The value is  2. - QuorumLossWaitDuration - Indicates the QuorumLossWaitDuration property is set. The value is 4. - StandByReplicaKeepDuration - Indicates the StandByReplicaKeepDuration property is set. The value is 8. - MinReplicaSetSize - Indicates the MinReplicaSetSize property is set. The value is 16. - PlacementConstraints - Indicates the PlacementConstraints property is set. The value is 32. - PlacementPolicyList - Indicates the ServicePlacementPolicies property is set. The value is 64. - Correlation - Indicates the CorrelationScheme property is set. The value is 128. - Metrics - Indicates the ServiceLoadMetrics property is set. The value is 256. - DefaultMoveCost - Indicates the DefaultMoveCost property is set. The value is 512. - ScalingPolicy - Indicates the ScalingPolicies property is set. The value is 1024. | [optional] [default to null]
**placement_constraints** | **String** | The placement constraints as a string. Placement constraints are boolean expressions on node properties and allow for restricting a service to particular nodes based on the service requirements. For example, to place a service on nodes where NodeType is blue specify the following: \&quot;NodeColor &#x3D;&#x3D; blue)\&quot;. | [optional] [default to null]
**correlation_scheme** | [***::models::CorrelationSchemeList**](CorrelationSchemeList.md) | The correlation scheme. | [optional] [default to null]
**load_metrics** | [***::models::ServiceLoadMetricsList**](ServiceLoadMetricsList.md) | The service load metrics. | [optional] [default to null]
**service_placement_policies** | [***::models::ServicePlacementPoliciesList**](ServicePlacementPoliciesList.md) | The service placement policies. | [optional] [default to null]
**default_move_cost** | [***::models::MoveCost**](MoveCost.md) | The move cost for the service. | [optional] [default to null]
**scaling_policies** | [***::models::ScalingPolicyDescriptionList**](ScalingPolicyDescriptionList.md) | Scaling policies for this service. | [optional] [default to null]
**instance_count** | **i32** | The instance count. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


