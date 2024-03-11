# StatefulServiceReplicaHealthState

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregated_health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | The ID of the partition to which this replica belongs. | [optional] [default to null]
**replica_id** | [***::models::ReplicaId**](ReplicaId.md) | Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


