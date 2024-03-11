# NodeAddedEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**node_name** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [default to null]
**node_id** | **String** | Id of Node. | [default to null]
**node_instance** | **i64** | Id of Node instance. | [default to null]
**node_type** | **String** | Type of Node. | [default to null]
**fabric_version** | **String** | Fabric version. | [default to null]
**ip_address_or_fqdn** | **String** | IP address or FQDN. | [default to null]
**node_capacities** | **String** | Capacities. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


