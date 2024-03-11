# FileShareBackupStorageDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**storage_kind** | [***::models::BackupStorageKind**](BackupStorageKind.md) | The kind of backup storage, where backups are saved. | [default to null]
**friendly_name** | **String** | Friendly name for this backup storage. | [optional] [default to null]
**path** | **String** | UNC path of the file share where to store or enumerate backups from. | [default to null]
**primary_user_name** | **String** | Primary user name to access the file share. | [optional] [default to null]
**primary_password** | **String** | Primary password to access the share location. | [optional] [default to null]
**secondary_user_name** | **String** | Secondary user name to access the file share. | [optional] [default to null]
**secondary_password** | **String** | Secondary password to access the share location | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


