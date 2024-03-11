# ServiceReplicaProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**os_type** | **String** | The Operating system type required by the code in service. | [default to null]
**code_packages** | [**Vec<::models::ContainerCodePackageProperties>**](ContainerCodePackageProperties.md) | Describes the set of code packages that forms the service. A code package describes the container and the properties for running it. All the code packages are started together on the same host and share the same context (network, process etc.). | [default to null]
**network_refs** | [**Vec<::models::NetworkRef>**](NetworkRef.md) | The names of the private networks that this service needs to be part of. | [optional] [default to null]
**diagnostics** | [***::models::DiagnosticsRef**](DiagnosticsRef.md) | Reference to sinks in DiagnosticsDescription. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


