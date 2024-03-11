# RepairTaskCancelDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the repair task. | [default to null]
**version** | **String** | The current version number of the repair task. If non-zero, then the request will only succeed if this value matches the actual current version of the repair task. If zero, then no version check is performed. | [optional] [default to null]
**request_abort** | **bool** | _True_ if the repair should be stopped as soon as possible even if it has already started executing. _False_ if the repair should be cancelled only if execution has not yet started. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


