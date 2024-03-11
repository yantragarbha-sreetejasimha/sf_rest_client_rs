# ApplicationHealthStateChunk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**application_name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**application_type_name** | [***::models::ApplicationTypeName**](ApplicationTypeName.md) | The application type name as defined in the application manifest. | [optional] [default to null]
**service_health_state_chunks** | [***::models::ServiceHealthStateChunkList**](ServiceHealthStateChunkList.md) | The list of service health state chunks in the cluster that respect the filters in the cluster health chunk query description. | [optional] [default to null]
**deployed_application_health_state_chunks** | [***::models::DeployedApplicationHealthStateChunkList**](DeployedApplicationHealthStateChunkList.md) | The list of deployed application health state chunks in the cluster that respect the filters in the cluster health chunk query description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


