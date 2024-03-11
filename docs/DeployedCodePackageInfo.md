# DeployedCodePackageInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::CodePackageName**](CodePackageName.md) | The name of the code package. | [optional] [default to null]
**version** | **String** | The version of the code package specified in service manifest. | [optional] [default to null]
**service_manifest_name** | [***::models::ServiceManifestName**](ServiceManifestName.md) | The name of service manifest that specified this code package. | [optional] [default to null]
**service_package_activation_id** | [***::models::ServicePackageActivationId**](ServicePackageActivationId.md) | The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is &#39;SharedProcess&#39; (or if it is not specified, in which case it defaults to &#39;SharedProcess&#39;), then value of ServicePackageActivationId is always an empty string. | [optional] [default to null]
**host_type** | [***::models::HostType**](HostType.md) | Specifies the type of host for main entry point of a code package as specified in service manifest. | [optional] [default to null]
**host_isolation_mode** | [***::models::HostIsolationMode**](HostIsolationMode.md) | Specifies the isolation mode of main entry point of a code package when it&#39;s host type is ContainerHost. This is specified as part of container host policies in application manifest while importing service manifest. | [optional] [default to null]
**status** | [***::models::DeploymentStatus**](DeploymentStatus.md) | Specifies the status of a deployed application or service package on a Service Fabric node. | [optional] [default to null]
**run_frequency_interval** | **String** | The interval at which code package is run. This is used for periodic code package. | [optional] [default to null]
**setup_entry_point** | [***::models::CodePackageEntryPoint**](CodePackageEntryPoint.md) | Information about setup or main entry point of a code package deployed on a Service Fabric node. | [optional] [default to null]
**main_entry_point** | [***::models::CodePackageEntryPoint**](CodePackageEntryPoint.md) | Information about setup or main entry point of a code package deployed on a Service Fabric node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


