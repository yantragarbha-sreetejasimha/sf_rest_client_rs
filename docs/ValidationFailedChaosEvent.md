# ValidationFailedChaosEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ChaosEventKind**](ChaosEventKind.md) | The kind of Chaos event. | [default to null]
**time_stamp_utc** | **String** | The UTC timestamp when this Chaos event was generated. | [default to null]
**reason** | **String** | Describes why the ValidationFailedChaosEvent was generated. This may happen because more than MaxPercentUnhealthyNodes are unhealthy for more than MaxClusterStabilizationTimeout. This reason will be in the Reason property of the ValidationFailedChaosEvent as a string. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


