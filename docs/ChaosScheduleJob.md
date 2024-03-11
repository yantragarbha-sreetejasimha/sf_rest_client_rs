# ChaosScheduleJob

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chaos_parameters** | **String** | A reference to which Chaos Parameters of the Chaos Schedule to use. | [optional] [default to null]
**days** | [***::models::ChaosScheduleJobActiveDaysOfWeek**](ChaosScheduleJobActiveDaysOfWeek.md) | Defines the days of the week that a Chaos Schedule Job will run for. | [optional] [default to null]
**times** | [**Vec<::models::TimeRange>**](TimeRange.md) | A list of Time Ranges that specify when during active days that this job will run. The times are interpreted as UTC. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


