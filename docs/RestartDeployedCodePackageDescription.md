# RestartDeployedCodePackageDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_manifest_name** | [***::models::ServiceManifestName**](ServiceManifestName.md) | The name of service manifest that specified this code package. | [default to null]
**service_package_activation_id** | [***::models::ServicePackageActivationId**](ServicePackageActivationId.md) | The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is &#39;SharedProcess&#39; (or if it is not specified, in which case it defaults to &#39;SharedProcess&#39;), then value of ServicePackageActivationId is always an empty string. | [optional] [default to null]
**code_package_name** | [***::models::CodePackageName**](CodePackageName.md) | The name of the code package defined in the service manifest. | [default to null]
**code_package_instance_id** | [***::models::CodePackageInstanceId**](CodePackageInstanceId.md) | The instance ID for currently running entry point. For a code package setup entry point (if specified) runs first and after it finishes main entry point is started. Each time entry point executable is run, its instance ID will change. If 0 is passed in as the code package instance ID, the API will restart the code package with whatever instance ID it is currently running. If an instance ID other than 0 is passed in, the API will restart the code package only if the current Instance ID matches the passed in instance ID. Note, passing in the exact instance ID (not 0) in the API is safer, because if ensures at most one restart of the code package. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


