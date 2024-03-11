# UniformInt64RangePartitionSchemeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**partition_scheme** | [***::models::PartitionScheme**](PartitionScheme.md) | Specifies how the service is partitioned. | [default to null]
**count** | **i32** | The number of partitions. | [default to null]
**low_key** | **String** | String indicating the lower bound of the partition key range that should be split between the partitions. | [default to null]
**high_key** | **String** | String indicating the upper bound of the partition key range that should be split between the partitions. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


