# ServiceHealthStateChunk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | The name of the service whose health state chunk is provided in this object. | [optional] [default to null]
**partition_health_state_chunks** | [***::models::PartitionHealthStateChunkList**](PartitionHealthStateChunkList.md) | The list of partition health state chunks belonging to the service that respect the filters in the cluster health chunk query description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


