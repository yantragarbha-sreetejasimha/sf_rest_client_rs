# ClusterUpgradeProgressObject

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code_version** | [***::models::ClusterFabricCodeVersionString**](ClusterFabricCodeVersionString.md) | The ServiceFabric code version of the cluster. | [optional] [default to null]
**config_version** | [***::models::ClusterFabricConfigVersionString**](ClusterFabricConfigVersionString.md) | The cluster configuration version (specified in the cluster manifest). | [optional] [default to null]
**upgrade_domains** | [***::models::UpgradeDomainInfoList**](UpgradeDomainInfoList.md) | List of upgrade domains and their statuses. Not applicable to node-by-node upgrades. | [optional] [default to null]
**upgrade_units** | [***::models::UpgradeUnitInfoList**](UpgradeUnitInfoList.md) | List of upgrade units and their statuses. | [optional] [default to null]
**upgrade_state** | [***::models::UpgradeState**](UpgradeState.md) | The state of the upgrade domain. | [optional] [default to null]
**next_upgrade_domain** | [***::models::NextUpgradeDomain**](NextUpgradeDomain.md) | The name of the next upgrade domain to be processed. Not applicable to node-by-node upgrades. | [optional] [default to null]
**rolling_upgrade_mode** | [***::models::UpgradeMode**](UpgradeMode.md) | The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, Monitored, and UnmonitoredDeferred. | [optional] [default to null]
**upgrade_description** | [***::models::ClusterUpgradeDescriptionObject**](ClusterUpgradeDescriptionObject.md) | Represents a ServiceFabric cluster upgrade | [optional] [default to null]
**upgrade_duration_in_milliseconds** | [***::models::UpgradeDurationString**](UpgradeDurationString.md) | The estimated elapsed time spent processing the current overall upgrade. | [optional] [default to null]
**upgrade_domain_duration_in_milliseconds** | [***::models::UpgradeDomainDurationString**](UpgradeDomainDurationString.md) | The estimated elapsed time spent processing the current upgrade domain. Not applicable to node-by-node upgrades. | [optional] [default to null]
**unhealthy_evaluations** | [***::models::UnhealthyEvaluations**](UnhealthyEvaluations.md) | List of health evaluations that resulted in the current aggregated health state. | [optional] [default to null]
**current_upgrade_domain_progress** | [***::models::CurrentUpgradeDomainProgressInfo**](CurrentUpgradeDomainProgressInfo.md) | Information about the current in-progress upgrade domain. Not applicable to node-by-node upgrades. | [optional] [default to null]
**current_upgrade_units_progress** | [***::models::CurrentUpgradeUnitsProgressInfo**](CurrentUpgradeUnitsProgressInfo.md) | Information about the current in-progress upgrade units. | [optional] [default to null]
**start_timestamp_utc** | [***::models::UpgradeStartTimeUtcString**](UpgradeStartTimeUTCString.md) | The start time of the upgrade in UTC. | [optional] [default to null]
**failure_timestamp_utc** | [***::models::UpgradeFailureTimeUtcString**](UpgradeFailureTimeUTCString.md) | The failure time of the upgrade in UTC. | [optional] [default to null]
**failure_reason** | [***::models::FailureReason**](FailureReason.md) | The cause of an upgrade failure that resulted in FailureAction being executed. | [optional] [default to null]
**upgrade_domain_progress_at_failure** | [***::models::FailedUpgradeDomainProgressObject**](FailedUpgradeDomainProgressObject.md) | The detailed upgrade progress for nodes in the current upgrade domain at the point of failure. Not applicable to node-by-node upgrades. | [optional] [default to null]
**is_node_by_node** | **bool** | Indicates whether this upgrade is node-by-node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


