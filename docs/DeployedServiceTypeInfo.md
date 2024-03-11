# DeployedServiceTypeInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_type_name** | [***::models::ServiceTypeName**](ServiceTypeName.md) | Name of the service type as specified in the service manifest. | [optional] [default to null]
**service_manifest_name** | [***::models::ServiceManifestName**](ServiceManifestName.md) | The name of the service manifest in which this service type is defined. | [optional] [default to null]
**code_package_name** | [***::models::CodePackageName**](CodePackageName.md) | The name of the code package that registered the service type. | [optional] [default to null]
**status** | [***::models::ServiceTypeRegistrationStatus**](ServiceTypeRegistrationStatus.md) | The status of the service type registration on the node. | [optional] [default to null]
**service_package_activation_id** | [***::models::ServicePackageActivationId**](ServicePackageActivationId.md) | The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is &#39;SharedProcess&#39; (or if it is not specified, in which case it defaults to &#39;SharedProcess&#39;), then value of ServicePackageActivationId is always an empty string. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


