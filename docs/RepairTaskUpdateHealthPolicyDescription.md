# RepairTaskUpdateHealthPolicyDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the repair task to be updated. | [default to null]
**version** | **String** | The current version number of the repair task. If non-zero, then the request will only succeed if this value matches the actual current value of the repair task. If zero, then no version check is performed. | [optional] [default to null]
**perform_preparing_health_check** | **bool** | A boolean indicating if health check is to be performed in the Preparing stage of the repair task. If not specified the existing value should not be altered. Otherwise, specify the desired new value. | [optional] [default to null]
**perform_restoring_health_check** | **bool** | A boolean indicating if health check is to be performed in the Restoring stage of the repair task. If not specified the existing value should not be altered. Otherwise, specify the desired new value. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


