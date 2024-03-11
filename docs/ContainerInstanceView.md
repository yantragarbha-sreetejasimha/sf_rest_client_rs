# ContainerInstanceView

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**restart_count** | **i32** | The number of times the container has been restarted. | [optional] [default to null]
**current_state** | [***::models::ContainerState**](ContainerState.md) | Current container instance state. | [optional] [default to null]
**previous_state** | [***::models::ContainerState**](ContainerState.md) | Previous container instance state. | [optional] [default to null]
**events** | [**Vec<::models::ContainerEvent>**](ContainerEvent.md) | The events of this container instance. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


