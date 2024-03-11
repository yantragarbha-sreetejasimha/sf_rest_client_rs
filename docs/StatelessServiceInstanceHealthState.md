# StatelessServiceInstanceHealthState

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | The ID of the partition to which this replica belongs. | [optional] [default to null]
**aggregated_health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**replica_id** | [***::models::ReplicaId**](ReplicaId.md) | Id of the stateless service instance on the wire this field is called ReplicaId. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

