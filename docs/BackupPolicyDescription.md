# BackupPolicyDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The unique name identifying this backup policy. | [default to null]
**auto_restore_on_data_loss** | **bool** | Specifies whether to trigger restore automatically using the latest available backup in case the partition experiences a data loss event. | [default to null]
**max_incremental_backups** | **i32** | Defines the maximum number of incremental backups to be taken between two full backups. This is just the upper limit. A full backup may be taken before specified number of incremental backups are completed in one of the following conditions - The replica has never taken a full backup since it has become primary, - Some of the log records since the last backup has been truncated, or - Replica passed the MaxAccumulatedBackupLogSizeInMB limit. | [default to null]
**schedule** | [***::models::BackupScheduleDescription**](BackupScheduleDescription.md) | Describes the backup schedule parameters. | [default to null]
**storage** | [***::models::BackupStorageDescription**](BackupStorageDescription.md) | Describes the details of backup storage where to store the periodic backups. | [default to null]
**retention_policy** | [***::models::RetentionPolicyDescription**](RetentionPolicyDescription.md) | Describes the policy to retain backups in storage. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


