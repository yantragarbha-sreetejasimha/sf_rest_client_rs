# PartitionHealthStateChunk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | The Id of the partition. | [optional] [default to null]
**replica_health_state_chunks** | [***::models::ReplicaHealthStateChunkList**](ReplicaHealthStateChunkList.md) | The list of replica health state chunks belonging to the partition that respect the filters in the cluster health chunk query description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


