# BackupProgressInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_state** | [***::models::BackupState**](BackupState.md) | Represents the current state of the partition backup operation. | [optional] [default to null]
**time_stamp_utc** | **String** | TimeStamp in UTC when operation succeeded or failed. | [optional] [default to null]
**backup_id** | **String** | Unique ID of the newly created backup. | [optional] [default to null]
**backup_location** | **String** | Location, relative to the backup store, of the newly created backup. | [optional] [default to null]
**epoch_of_last_backup_record** | [***::models::Epoch**](Epoch.md) | Specifies the epoch of the last record included in backup. | [optional] [default to null]
**lsn_of_last_backup_record** | **String** | The LSN of last record included in backup. | [optional] [default to null]
**failure_error** | [***::models::FabricErrorError**](FabricErrorError.md) | Denotes the failure encountered in performing backup operation. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


