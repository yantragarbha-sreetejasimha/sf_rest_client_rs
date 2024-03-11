# DeployedServicePackageInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::ServiceManifestName**](ServiceManifestName.md) | The name of the service package as specified in the service manifest. | [optional] [default to null]
**version** | **String** | The version of the service package specified in service manifest. | [optional] [default to null]
**status** | [***::models::DeploymentStatus**](DeploymentStatus.md) | Specifies the status of a deployed application or service package on a Service Fabric node. | [optional] [default to null]
**service_package_activation_id** | [***::models::ServicePackageActivationId**](ServicePackageActivationId.md) | The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is &#39;SharedProcess&#39; (or if it is not specified, in which case it defaults to &#39;SharedProcess&#39;), then value of ServicePackageActivationId is always an empty string. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


