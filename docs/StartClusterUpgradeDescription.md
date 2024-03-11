# StartClusterUpgradeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code_version** | **String** | The cluster code version. | [optional] [default to null]
**config_version** | **String** | The cluster configuration version. | [optional] [default to null]
**upgrade_kind** | [***::models::UpgradeKind**](UpgradeKind.md) | The kind of upgrade out of the following possible values. | [optional] [default to null]
**rolling_upgrade_mode** | [***::models::UpgradeMode**](UpgradeMode.md) | The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, Monitored, and UnmonitoredDeferred. | [optional] [default to null]
**upgrade_replica_set_check_timeout_in_seconds** | [***::models::UpgradeReplicaSetCheckTimeout**](UpgradeReplicaSetCheckTimeout.md) | The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer). | [optional] [default to null]
**force_restart** | [***::models::ForceRestart**](ForceRestart.md) | If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data). | [optional] [default to null]
**sort_order** | [***::models::UpgradeSortOrder**](UpgradeSortOrder.md) | Defines the order in which an upgrade proceeds through the cluster. | [optional] [default to null]
**monitoring_policy** | [***::models::MonitoringPolicyDescription**](MonitoringPolicyDescription.md) | Describes the parameters for monitoring an upgrade in Monitored mode. | [optional] [default to null]
**cluster_health_policy** | [***::models::ClusterHealthPolicy**](ClusterHealthPolicy.md) | Defines a health policy used to evaluate the health of the cluster or of a cluster node. | [optional] [default to null]
**enable_delta_health_evaluation** | **bool** | When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain. | [optional] [default to null]
**cluster_upgrade_health_policy** | [***::models::ClusterUpgradeHealthPolicyObject**](ClusterUpgradeHealthPolicyObject.md) | Defines a health policy used to evaluate the health of the cluster during a cluster upgrade. | [optional] [default to null]
**application_health_policy_map** | [***::models::ApplicationHealthPolicies**](ApplicationHealthPolicies.md) | Defines the application health policy map used to evaluate the health of an application or one of its children entities. | [optional] [default to null]
**instance_close_delay_duration_in_seconds** | [***::models::InstanceCloseDelayDurationInSeconds**](InstanceCloseDelayDurationInSeconds.md) | Duration in seconds, to wait before a stateless instance is closed, to allow the active requests to drain gracefully. This would be effective when the instance is closing during the application/cluster upgrade, only for those instances which have a non-zero delay duration configured in the service description. See InstanceCloseDelayDurationSeconds property in $ref: \&quot;#/definitions/StatelessServiceDescription.yaml\&quot; for details. Note, the default value of InstanceCloseDelayDurationInSeconds is 4294967295, which indicates that the behavior will entirely depend on the delay configured in the stateless service description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


