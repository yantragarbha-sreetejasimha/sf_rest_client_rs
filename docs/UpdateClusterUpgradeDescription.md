# UpdateClusterUpgradeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**upgrade_kind** | [***::models::UpgradeType**](UpgradeType.md) | The type of upgrade out of the following possible values. | [optional] [default to null]
**update_description** | [***::models::RollingUpgradeUpdateDescription**](RollingUpgradeUpdateDescription.md) | Describes the parameters for updating a rolling upgrade of application or cluster. | [optional] [default to null]
**cluster_health_policy** | [***::models::ClusterHealthPolicy**](ClusterHealthPolicy.md) | Defines a health policy used to evaluate the health of the cluster or of a cluster node. | [optional] [default to null]
**enable_delta_health_evaluation** | [***::models::DeltaHealthEvaluationBool**](DeltaHealthEvaluationBool.md) | When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain. | [optional] [default to null]
**cluster_upgrade_health_policy** | [***::models::ClusterUpgradeHealthPolicyObject**](ClusterUpgradeHealthPolicyObject.md) | Defines a health policy used to evaluate the health of the cluster during a cluster upgrade. | [optional] [default to null]
**application_health_policy_map** | [***::models::ApplicationHealthPolicies**](ApplicationHealthPolicies.md) | Defines the application health policy map used to evaluate the health of an application or one of its children entities. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


