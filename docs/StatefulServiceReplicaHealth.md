# StatefulServiceReplicaHealth

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | Id of the partition to which this replica belongs. | [optional] [default to null]
**aggregated_health_state** | [***::models::HealthState**](HealthState.md) | The HealthState representing the aggregated health state of the entity computed by Health Manager. The health evaluation of the entity reflects all events reported on the entity and its children (if any). The aggregation is done by applying the desired health policy. | [optional] [default to null]
**health_events** | [**Vec<::models::HealthEvent>**](HealthEvent.md) | The list of health events reported on the entity. | [optional] [default to null]
**unhealthy_evaluations** | [***::models::UnhealthyEvaluations**](UnhealthyEvaluations.md) | The unhealthy evaluations that show why the current aggregated health state was returned by Health Manager. | [optional] [default to null]
**health_statistics** | [***::models::HealthStatistics**](HealthStatistics.md) | Shows the health statistics for all children types of the queried entity. | [optional] [default to null]
**replica_id** | [***::models::ReplicaId**](ReplicaId.md) | Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

