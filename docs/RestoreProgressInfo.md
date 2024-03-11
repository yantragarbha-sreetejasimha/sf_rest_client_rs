# RestoreProgressInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**restore_state** | [***::models::RestoreState**](RestoreState.md) | Represents the current state of the partition restore operation. | [optional] [default to null]
**time_stamp_utc** | **String** | Timestamp when operation succeeded or failed. | [optional] [default to null]
**restored_epoch** | [***::models::Epoch**](Epoch.md) | Describes the epoch at which the partition is restored. | [optional] [default to null]
**restored_lsn** | **String** | Restored LSN. | [optional] [default to null]
**failure_error** | [***::models::FabricErrorError**](FabricErrorError.md) | Denotes the failure encountered in performing restore operation. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


