# RemoteReplicatorStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**replica_id** | [***::models::ReplicaId**](ReplicaId.md) | Represents the replica ID of the remote secondary replicator. | [optional] [default to null]
**last_acknowledgement_processed_time_utc** | **String** | The last timestamp (in UTC) when an acknowledgement from the secondary replicator was processed on the primary. UTC 0 represents an invalid value, indicating that no acknowledgement messages were ever processed. | [optional] [default to null]
**last_received_replication_sequence_number** | **String** | The highest replication operation sequence number that the secondary has received from the primary. | [optional] [default to null]
**last_applied_replication_sequence_number** | **String** | The highest replication operation sequence number that the secondary has applied to its state. | [optional] [default to null]
**is_in_build** | **bool** | A value that indicates whether the secondary replica is in the process of being built. | [optional] [default to null]
**last_received_copy_sequence_number** | **String** | The highest copy operation sequence number that the secondary has received from the primary. A value of -1 implies that the secondary has received all copy operations. | [optional] [default to null]
**last_applied_copy_sequence_number** | **String** | The highest copy operation sequence number that the secondary has applied to its state. A value of -1 implies that the secondary has applied all copy operations and the copy process is complete. | [optional] [default to null]
**remote_replicator_acknowledgement_status** | [***::models::RemoteReplicatorAcknowledgementStatus**](RemoteReplicatorAcknowledgementStatus.md) | Represents the acknowledgment status for the remote secondary replicator. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


