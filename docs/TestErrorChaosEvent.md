# TestErrorChaosEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ChaosEventKind**](ChaosEventKind.md) | The kind of Chaos event. | [default to null]
**time_stamp_utc** | **String** | The UTC timestamp when this Chaos event was generated. | [default to null]
**reason** | **String** | Describes why TestErrorChaosEvent was generated. For example, Chaos tries to fault a partition but finds that the partition is no longer fault tolerant, then a TestErrorEvent gets generated with the reason stating that the partition is not fault tolerant. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


