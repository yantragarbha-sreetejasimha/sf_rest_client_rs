# ClusterUpgradeDescriptionObject

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_version** | [***::models::ClusterFabricConfigVersionString**](ClusterFabricConfigVersionString.md) | The cluster configuration version (specified in the cluster manifest). | [optional] [default to null]
**code_version** | [***::models::ClusterFabricCodeVersionString**](ClusterFabricCodeVersionString.md) | The ServiceFabric code version of the cluster. | [optional] [default to null]
**upgrade_kind** | [***::models::UpgradeKind**](UpgradeKind.md) | The kind of upgrade out of the following possible values. | [optional] [default to null]
**rolling_upgrade_mode** | [***::models::UpgradeMode**](UpgradeMode.md) | The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored. | [optional] [default to null]
**upgrade_replica_set_check_timeout_in_seconds** | [***::models::UpgradeReplicaSetCheckTimeout**](UpgradeReplicaSetCheckTimeout.md) | The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer). | [optional] [default to null]
**force_restart** | [***::models::ForceRestart**](ForceRestart.md) | If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data). | [optional] [default to null]
**sort_order** | [***::models::UpgradeSortOrder**](UpgradeSortOrder.md) | Defines the order in which an upgrade proceeds through the cluster. | [optional] [default to null]
**enable_delta_health_evaluation** | [***::models::DeltaHealthEvaluationBool**](DeltaHealthEvaluationBool.md) | When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain. | [optional] [default to null]
**monitoring_policy** | [***::models::MonitoringPolicyDescription**](MonitoringPolicyDescription.md) | Describes the parameters for monitoring an upgrade in Monitored mode. | [optional] [default to null]
**cluster_health_policy** | [***::models::ClusterHealthPolicy**](ClusterHealthPolicy.md) | Defines a health policy used to evaluate the health of the cluster or of a cluster node. | [optional] [default to null]
**cluster_upgrade_health_policy** | [***::models::ClusterUpgradeHealthPolicyObject**](ClusterUpgradeHealthPolicyObject.md) | Defines a health policy used to evaluate the health of the cluster during a cluster upgrade. | [optional] [default to null]
**application_health_policy_map** | [***::models::ApplicationHealthPolicyMap**](ApplicationHealthPolicyMap.md) | Defines a map that contains specific application health policies for different applications. Each entry specifies as key the application name and as value an ApplicationHealthPolicy used to evaluate the application health. If an application is not specified in the map, the application health evaluation uses the ApplicationHealthPolicy found in its application manifest or the default application health policy (if no health policy is defined in the manifest). The map is empty by default. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


