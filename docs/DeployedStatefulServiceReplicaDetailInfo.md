# DeployedStatefulServiceReplicaDetailInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | Full hierarchical name of the service in URI format starting with &#x60;fabric:&#x60;. | [optional] [default to null]
**partition_id** | [***::models::PartitionId**](PartitionId.md) | An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different. | [optional] [default to null]
**current_service_operation** | [***::models::ServiceOperationName**](ServiceOperationName.md) | Specifies the current active life-cycle operation on a stateful service replica or stateless service instance. | [optional] [default to null]
**current_service_operation_start_time_utc** | **String** | The start time of the current service operation in UTC format. | [optional] [default to null]
**reported_load** | [***::models::LoadMetricReportInfoList**](LoadMetricReportInfoList.md) | List of load reported by replica. | [optional] [default to null]
**replica_id** | [***::models::ReplicaId**](ReplicaId.md) | Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id. | [optional] [default to null]
**current_replicator_operation** | [***::models::ReplicatorOperationName**](ReplicatorOperationName.md) | Specifies the operation currently being executed by the Replicator. | [optional] [default to null]
**read_status** | [***::models::PartitionAccessStatus**](PartitionAccessStatus.md) | Specifies the access status of the partition. | [optional] [default to null]
**write_status** | [***::models::PartitionAccessStatus**](PartitionAccessStatus.md) | Specifies the access status of the partition. | [optional] [default to null]
**replicator_status** | [***::models::ReplicatorStatus**](ReplicatorStatus.md) | Represents a base class for primary or secondary replicator status. Contains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc. | [optional] [default to null]
**replica_status** | [***::models::KeyValueStoreReplicaStatus**](KeyValueStoreReplicaStatus.md) | Key value store related information for the replica. | [optional] [default to null]
**deployed_service_replica_query_result** | [***::models::DeployedStatefulServiceReplicaInfo**](DeployedStatefulServiceReplicaInfo.md) | Information about a stateful service replica deployed on a node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


