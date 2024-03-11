# ReplicaHealthEvaluation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::HealthEvaluationKind**](HealthEvaluationKind.md) | The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values. | [default to null]
**aggregated_health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**description** | **String** | Description of the health evaluation, which represents a summary of the evaluation process. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | Id of the partition to which the replica belongs. | [optional] [default to null]
**replica_or_instance_id** | [***::models::ReplicaOrInstanceId**](ReplicaOrInstanceId.md) | Id of a stateful service replica or a stateless service instance. This ID is used in the queries that apply to both stateful and stateless services. It is used by Service Fabric to uniquely identify a replica of a partition of a stateful service or an instance of a stateless service partition. It is unique within a partition and does not change for the lifetime of the replica or the instance. If a stateful replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the ID. If a stateless instance is failed over on the same or different node it will get a different value for the ID. | [optional] [default to null]
**unhealthy_evaluations** | [***::models::UnhealthyEvaluations**](UnhealthyEvaluations.md) | List of unhealthy evaluations that led to the current aggregated health state of the replica. The types of the unhealthy evaluations can be EventHealthEvaluation. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


