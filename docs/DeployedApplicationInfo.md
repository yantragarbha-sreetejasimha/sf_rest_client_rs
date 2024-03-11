# DeployedApplicationInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [***::models::ApplicationId**](ApplicationId.md) | The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \&quot;\\~\&quot; character. For example, if the application name is \&quot;fabric:/myapp/app1\&quot;, the application identity would be \&quot;myapp\\~app1\&quot; in 6.0+ and \&quot;myapp/app1\&quot; in previous versions. | [optional] [default to null]
**name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**type_name** | [***::models::ApplicationTypeName**](ApplicationTypeName.md) | The application type name as defined in the application manifest. | [optional] [default to null]
**type_version** | [***::models::ApplicationTypeVersion**](ApplicationTypeVersion.md) | The version of the application type as defined in the application manifest. | [optional] [default to null]
**status** | [***::models::DeployedApplicationStatus**](DeployedApplicationStatus.md) | The status of the application deployed on the node. Following are the possible values. | [optional] [default to null]
**work_directory** | **String** | The work directory of the application on the node. The work directory can be used to store application data. | [optional] [default to null]
**log_directory** | **String** | The log directory of the application on the node. The log directory can be used to store application logs. | [optional] [default to null]
**temp_directory** | **String** | The temp directory of the application on the node. The code packages belonging to the application are forked with this directory set as their temporary directory. | [optional] [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


