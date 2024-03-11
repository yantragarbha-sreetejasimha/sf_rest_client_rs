# PartitionBackupEntity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity_kind** | [***::models::BackupEntityKind**](BackupEntityKind.md) | The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled. | [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | The full name of the service with &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


