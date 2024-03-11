# NodeUpgradeProgressInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_name** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [optional] [default to null]
**upgrade_phase** | [***::models::NodeUpgradePhase**](NodeUpgradePhase.md) | The state of the upgrading node. | [optional] [default to null]
**pending_safety_checks** | [***::models::SafetyCheckInfoList**](SafetyCheckInfoList.md) | List of pending safety checks | [optional] [default to null]
**upgrade_duration** | [***::models::NodeUpgradeDurationString**](NodeUpgradeDurationString.md) | The estimated time spent processing the node since it was deactivated during a node-by-node upgrade. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


