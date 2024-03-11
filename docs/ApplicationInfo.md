# ApplicationInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [***::models::ApplicationId**](ApplicationId.md) | The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \&quot;\\~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp\\~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | [optional] [default to null]
**name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**type_name** | [***::models::ApplicationTypeName**](ApplicationTypeName.md) | The application type name as defined in the application manifest. | [optional] [default to null]
**type_version** | [***::models::ApplicationTypeVersion**](ApplicationTypeVersion.md) | The version of the application type as defined in the application manifest. | [optional] [default to null]
**status** | [***::models::ApplicationStatus**](ApplicationStatus.md) | The status of the application. | [optional] [default to null]
**parameters** | [***::models::ApplicationParameterList**](ApplicationParameterList.md) | List of application parameters with overridden values from their default values specified in the application manifest. | [optional] [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**application_definition_kind** | [***::models::ApplicationDefinitionKind**](ApplicationDefinitionKind.md) | The mechanism used to define a Service Fabric application. | [optional] [default to null]
**managed_application_identity** | [***::models::ManagedApplicationIdentityDescription**](ManagedApplicationIdentityDescription.md) | Managed application identity description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

