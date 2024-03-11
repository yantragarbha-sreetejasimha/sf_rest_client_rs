# PartitionBackupConfigurationInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::BackupEntityKind**](BackupEntityKind.md) | The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled. | [default to null]
**policy_name** | **String** | The name of the backup policy which is applicable to this Service Fabric application or service or partition. | [optional] [default to null]
**policy_inherited_from** | [***::models::BackupPolicyScope**](BackupPolicyScope.md) | Specifies the scope at which the backup policy is applied. | [optional] [default to null]
**suspension_info** | [***::models::BackupSuspensionInfo**](BackupSuspensionInfo.md) | Describes the backup suspension details. | [optional] [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | The full name of the service with &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | The partition ID identifying the partition. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


