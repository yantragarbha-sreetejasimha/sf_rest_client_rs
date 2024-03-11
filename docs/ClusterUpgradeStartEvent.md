# ClusterUpgradeStartEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**current_cluster_version** | **String** | Current Cluster version. | [default to null]
**target_cluster_version** | **String** | Target Cluster version. | [default to null]
**upgrade_type** | **String** | Type of upgrade. | [default to null]
**rolling_upgrade_mode** | **String** | Mode of upgrade. | [default to null]
**failure_action** | **String** | Action if failed. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


