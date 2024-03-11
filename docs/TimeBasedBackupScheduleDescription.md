# TimeBasedBackupScheduleDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schedule_kind** | [***::models::BackupScheduleKind**](BackupScheduleKind.md) | The kind of backup schedule, time based or frequency based. | [default to null]
**schedule_frequency_type** | [***::models::BackupScheduleFrequencyType**](BackupScheduleFrequencyType.md) | Describes the frequency with which to run the time based backup schedule. | [default to null]
**run_days** | [***::models::DayOfWeekList**](DayOfWeekList.md) | List of days of a week when to trigger the periodic backup. This is valid only when the backup schedule frequency type is weekly. | [optional] [default to null]
**run_times** | [***::models::TimeList**](TimeList.md) | Represents the list of exact time during the day in ISO8601 format. Like &#39;19:00:00&#39; will represent &#39;7PM&#39; during the day. Date specified along with time will be ignored. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


