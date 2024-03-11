# AddRemoveReplicaScalingMechanism

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::AutoScalingMechanismKind**](AutoScalingMechanismKind.md) | The type of auto scaling mechanism. | [default to null]
**min_count** | **i32** | Minimum number of containers (scale down won&#39;t be performed below this number). | [default to null]
**max_count** | **i32** | Maximum number of containers (scale up won&#39;t be performed above this number). | [default to null]
**scale_increment** | **i32** | Each time auto scaling is performed, this number of containers will be added or removed. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


