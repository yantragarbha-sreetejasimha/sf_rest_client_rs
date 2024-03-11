# SecondaryIdleReplicatorStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**replication_queue_status** | [***::models::ReplicatorQueueStatus**](ReplicatorQueueStatus.md) | Details about the replication queue on the secondary replicator. | [optional] [default to null]
**last_replication_operation_received_time_utc** | **String** | The last time-stamp (UTC) at which a replication operation was received from the primary. UTC 0 represents an invalid value, indicating that a replication operation message was never received. | [optional] [default to null]
**is_in_build** | **bool** | Value that indicates whether the replica is currently being built. | [optional] [default to null]
**copy_queue_status** | [***::models::ReplicatorQueueStatus**](ReplicatorQueueStatus.md) | Details about the copy queue on the secondary replicator. | [optional] [default to null]
**last_copy_operation_received_time_utc** | **String** | The last time-stamp (UTC) at which a copy operation was received from the primary. UTC 0 represents an invalid value, indicating that a copy operation message was never received. | [optional] [default to null]
**last_acknowledgement_sent_time_utc** | **String** | The last time-stamp (UTC) at which an acknowledgment was sent to the primary replicator. UTC 0 represents an invalid value, indicating that an acknowledgment message was never sent. | [optional] [default to null]
**kind** | [***::models::ReplicaRole**](ReplicaRole.md) | The role of a replica of a stateful service. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


