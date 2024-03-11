# NamedPartitionInformation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_partition_kind** | [***::models::ServicePartitionKind**](ServicePartitionKind.md) | The kind of partitioning scheme used to partition the service. | [default to null]
**id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [optional] [default to null]
**name** | **String** | Name of the partition. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


