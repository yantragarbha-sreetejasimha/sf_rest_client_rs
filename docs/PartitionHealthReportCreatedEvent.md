# PartitionHealthReportCreatedEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::FabricEventKind**](FabricEventKind.md) | The kind of FabricEvent. | [default to null]
**event_instance_id** | **String** | The identifier for the FabricEvent instance. | [default to null]
**time_stamp** | **String** | The time event was logged. | [default to null]
**has_correlated_events** | **bool** | Shows there is existing related events available. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [default to null]
**source_id** | **String** | Id of report source. | [default to null]
**property** | **String** | Describes the property. | [default to null]
**health_state** | **String** | Describes the property health state. | [default to null]
**time_to_live_ms** | **i64** | Time to live in milli-seconds. | [default to null]
**sequence_number** | **i64** | Sequence number of report. | [default to null]
**description** | **String** | Description of report. | [default to null]
**remove_when_expired** | **bool** | Indicates the removal when it expires. | [default to null]
**source_utc_timestamp** | **String** | Source time. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


