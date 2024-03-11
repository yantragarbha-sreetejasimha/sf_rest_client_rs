# ChaosNodeRestartScheduledEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_name** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [optional] [default to null]
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**category** | **String** | The category of event. | [optional] [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**node_instance_id** | **i64** | Id of Node instance. | [default to null]
**fault_group_id** | **String** | Id of fault group. | [default to null]
**fault_id** | **String** | Id of fault. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


