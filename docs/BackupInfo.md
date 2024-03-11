# BackupInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_id** | **String** | Unique backup ID . | [optional] [default to null]
**backup_chain_id** | **String** | Unique backup chain ID. All backups part of the same chain has the same backup chain id. A backup chain is comprised of 1 full backup and multiple incremental backups. | [optional] [default to null]
**application_name** | **String** | Name of the Service Fabric application this partition backup belongs to. | [optional] [default to null]
**service_name** | **String** | Name of the Service Fabric service this partition backup belongs to. | [optional] [default to null]
**partition_information** | [***::models::PartitionInformation**](PartitionInformation.md) | Information about the partition to which this backup belongs to | [optional] [default to null]
**backup_location** | **String** | Location of the backup, relative to the backup store. | [optional] [default to null]
**backup_type** | [***::models::BackupType**](BackupType.md) | Describes the type of backup, whether its full or incremental. | [optional] [default to null]
**epoch_of_last_backup_record** | [***::models::Epoch**](Epoch.md) | Epoch of the last record in this backup. | [optional] [default to null]
**lsn_of_last_backup_record** | **String** | LSN of the last record in this backup. | [optional] [default to null]
**creation_time_utc** | **String** | The date time when this backup was taken. | [optional] [default to null]
**service_manifest_version** | **String** | Manifest Version of the service this partition backup belongs to. | [optional] [default to null]
**failure_error** | [***::models::FabricErrorError**](FabricErrorError.md) | Denotes the failure encountered in getting backup point information. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


