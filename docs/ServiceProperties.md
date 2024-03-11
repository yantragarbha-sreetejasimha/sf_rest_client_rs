# ServiceProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | User readable description of the service. | [optional] [default to null]
**replica_count** | **i32** | The number of replicas of the service to create. Defaults to 1 if not specified. | [optional] [default to null]
**auto_scaling_policies** | [**Vec<::models::AutoScalingPolicy>**](AutoScalingPolicy.md) | Auto scaling policies | [optional] [default to null]
**status** | [***::models::ResourceStatus**](ResourceStatus.md) | Status of the service. | [optional] [default to null]
**status_details** | **String** | Gives additional information about the current status of the service. | [optional] [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | Describes the health state of an application resource. | [optional] [default to null]
**unhealthy_evaluation** | **String** | When the service&#39;s health state is not &#39;Ok&#39;, this additional details from service fabric Health Manager for the user to know why the service is marked unhealthy. | [optional] [default to null]
**identity_refs** | [**Vec<::models::ServiceIdentity>**](ServiceIdentity.md) | The service identity list. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


