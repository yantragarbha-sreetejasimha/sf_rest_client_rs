# StatefulServiceReplicaInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**replica_status** | [***::models::ReplicaStatus**](ReplicaStatus.md) | The status of a replica of a service. | [optional] [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**node_name** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [optional] [default to null]
**address** | **String** | The address the replica is listening on. | [optional] [default to null]
**last_in_build_duration_in_seconds** | **String** | The last in build duration of the replica in seconds. | [optional] [default to null]
**replica_role** | [***::models::ReplicaRole**](ReplicaRole.md) | The role of a replica of a stateful service. | [optional] [default to null]
**replica_id** | [***::models::ReplicaId**](ReplicaId.md) | Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


