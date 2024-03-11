# DeployedApplicationHealthStateChunk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**node_name** | **String** | The name of node where the application is deployed. | [optional] [default to null]
**deployed_service_package_health_state_chunks** | [***::models::DeployedServicePackageHealthStateChunkList**](DeployedServicePackageHealthStateChunkList.md) | The list of deployed service package health state chunks belonging to the deployed application that respect the filters in the cluster health chunk query description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


