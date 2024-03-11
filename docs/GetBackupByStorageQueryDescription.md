# GetBackupByStorageQueryDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date_time_filter** | **String** | Specifies the start date time in ISO8601 from which to enumerate backups. If not specified, backups are enumerated from the beginning. | [optional] [default to null]
**end_date_time_filter** | **String** | Specifies the end date time in ISO8601 till which to enumerate backups. If not specified, backups are enumerated till the end. | [optional] [default to null]
**latest** | **bool** | If specified as true, gets the most recent backup (within the specified time range) for every partition under the specified backup entity. | [optional] [default to null]
**storage** | [***::models::BackupStorageDescription**](BackupStorageDescription.md) | Describes the parameters for the backup storage from where to enumerate backups. This is optional and by default backups are enumerated from the backup storage where this backup entity is currently being backed up (as specified in backup policy). This parameter is useful to be able to enumerate backups from another cluster where you may intend to restore. | [default to null]
**backup_entity** | [***::models::BackupEntity**](BackupEntity.md) | Indicates the entity for which to enumerate backups. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


