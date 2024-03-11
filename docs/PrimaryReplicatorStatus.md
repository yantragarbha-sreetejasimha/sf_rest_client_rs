# PrimaryReplicatorStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ReplicaRole**](ReplicaRole.md) | The role of a replica of a stateful service. | [default to null]
**replication_queue_status** | [***::models::ReplicatorQueueStatus**](ReplicatorQueueStatus.md) | Details about the replication queue on the primary replicator. | [optional] [default to null]
**remote_replicators** | [***::models::RemoteReplicatorStatusList**](RemoteReplicatorStatusList.md) | The status of all the active and idle secondary replicators that the primary is aware of. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


