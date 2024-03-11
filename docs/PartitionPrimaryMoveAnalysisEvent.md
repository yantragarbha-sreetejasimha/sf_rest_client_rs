# PartitionPrimaryMoveAnalysisEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [default to null]
**metadata** | [***::models::AnalysisEventMetadata**](AnalysisEventMetadata.md) | Metadata about an Analysis Event. | [default to null]
**when_move_completed** | **String** | Time when the move was completed. | [default to null]
**previous_node** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [default to null]
**current_node** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [default to null]
**move_reason** | **String** | Move reason. | [default to null]
**relevant_traces** | **String** | Relevant traces. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


