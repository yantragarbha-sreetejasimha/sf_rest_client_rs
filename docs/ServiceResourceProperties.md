# ServiceResourceProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**os_type** | [***::models::OperatingSystemType**](OperatingSystemType.md) | The operation system required by the code in service. | [default to null]
**code_packages** | [**Vec<::models::ContainerCodePackageProperties>**](ContainerCodePackageProperties.md) | Describes the set of code packages that forms the service. A code package describes the container and the properties for running it. All the code packages are started together on the same host and share the same context (network, process etc.). | [default to null]
**network_refs** | [**Vec<::models::NetworkRef>**](NetworkRef.md) | The names of the private networks that this service needs to be part of. | [optional] [default to null]
**diagnostics** | [***::models::DiagnosticsRef**](DiagnosticsRef.md) | Reference to sinks in DiagnosticsDescription. | [optional] [default to null]
**description** | **String** | User readable description of the service. | [optional] [default to null]
**replica_count** | **i32** | The number of replicas of the service to create. Defaults to 1 if not specified. | [optional] [default to null]
**auto_scaling_policies** | [**Vec<::models::AutoScalingPolicy>**](AutoScalingPolicy.md) | Auto scaling policies | [optional] [default to null]
**status** | [***::models::ResourceStatus**](ResourceStatus.md) | Status of the service. | [optional] [default to null]
**status_details** | **String** | Gives additional information about the current status of the service. | [optional] [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | Describes the health state of an application resource. | [optional] [default to null]
**unhealthy_evaluation** | **String** | When the service&#39;s health state is not &#39;Ok&#39;, this additional details from service fabric Health Manager for the user to know why the service is marked unhealthy. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


