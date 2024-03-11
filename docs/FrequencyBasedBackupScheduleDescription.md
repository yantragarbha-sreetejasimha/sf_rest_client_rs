# FrequencyBasedBackupScheduleDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schedule_kind** | [***::models::BackupScheduleKind**](BackupScheduleKind.md) | The kind of backup schedule, time based or frequency based. | [default to null]
**interval** | **String** | Defines the interval with which backups are periodically taken. It should be specified in ISO8601 format. Timespan in seconds is not supported and will be ignored while creating the policy. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


