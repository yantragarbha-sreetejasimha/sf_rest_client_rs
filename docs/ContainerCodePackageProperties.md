# ContainerCodePackageProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the code package. | [default to null]
**image** | **String** | The Container image to use. | [default to null]
**image_registry_credential** | [***::models::ImageRegistryCredential**](ImageRegistryCredential.md) | Image registry credential. | [optional] [default to null]
**entrypoint** | **String** | Override for the default entry point in the container. | [optional] [default to null]
**commands** | **Vec<String>** | Command array to execute within the container in exec form. | [optional] [default to null]
**environment_variables** | [**Vec<::models::EnvironmentVariable>**](EnvironmentVariable.md) | The environment variables to set in this container | [optional] [default to null]
**settings** | [**Vec<::models::Setting>**](Setting.md) | The settings to set in this container. The setting file path can be fetched from environment variable \&quot;Fabric_SettingPath\&quot;. The path for Windows container is \&quot;C:\\\\secrets\&quot;. The path for Linux container is \&quot;/var/secrets\&quot;. | [optional] [default to null]
**labels** | [**Vec<::models::ContainerLabel>**](ContainerLabel.md) | The labels to set in this container. | [optional] [default to null]
**endpoints** | [**Vec<::models::EndpointProperties>**](EndpointProperties.md) | The endpoints exposed by this container. | [optional] [default to null]
**resources** | [***::models::ResourceRequirements**](ResourceRequirements.md) | This type describes the resource requirements for a container or a service. | [default to null]
**volume_refs** | [**Vec<::models::ContainerVolume>**](ContainerVolume.md) | The volumes to be attached to the container. | [optional] [default to null]
**instance_view** | [***::models::ContainerInstanceView**](ContainerInstanceView.md) | Runtime information of a container instance. | [optional] [default to null]
**diagnostics** | [***::models::DiagnosticsRef**](DiagnosticsRef.md) | Reference to sinks in DiagnosticsDescription. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


