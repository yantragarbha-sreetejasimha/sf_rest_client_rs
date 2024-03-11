# ChaosSchedule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | **String** | The date and time Chaos will start using this schedule. | [optional] [default to null]
**expiry_date** | **String** | The date and time Chaos will continue to use this schedule until. | [optional] [default to null]
**chaos_parameters_dictionary** | [**Vec<::models::ChaosParametersDictionaryItem>**](ChaosParametersDictionaryItem.md) | A mapping of string names to Chaos Parameters to be referenced by Chaos Schedule Jobs. | [optional] [default to null]
**jobs** | [**Vec<::models::ChaosScheduleJob>**](ChaosScheduleJob.md) | A list of all Chaos Schedule Jobs that will be automated by the schedule. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


