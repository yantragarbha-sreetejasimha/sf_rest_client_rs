# StoppedChaosEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ChaosEventKind**](ChaosEventKind.md) | The kind of Chaos event. | [default to null]
**time_stamp_utc** | **String** | The UTC timestamp when this Chaos event was generated. | [default to null]
**reason** | **String** | Describes why Chaos stopped. Chaos can stop because of StopChaos API call or the timeToRun provided in ChaosParameters is over. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


