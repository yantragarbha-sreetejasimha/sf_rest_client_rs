# ReplicatorQueueStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_utilization_percentage** | **i32** | Represents the utilization of the queue. A value of 0 indicates that the queue is empty and a value of 100 indicates the queue is full. | [optional] [default to null]
**queue_memory_size** | **String** | Represents the virtual memory consumed by the queue in bytes. | [optional] [default to null]
**first_sequence_number** | **String** | On a primary replicator, this is semantically the sequence number of the operation for which all the secondary replicas have sent an acknowledgement. On a secondary replicator, this is the smallest sequence number of the operation that is present in the queue. | [optional] [default to null]
**completed_sequence_number** | **String** | On a primary replicator, this is semantically the highest sequence number of the operation for which all the secondary replicas have sent an acknowledgement. On a secondary replicator, this is semantically the highest sequence number that has been applied to the persistent state. | [optional] [default to null]
**committed_sequence_number** | **String** | On a primary replicator, this is semantically the highest sequence number of the operation for which a write quorum of the secondary replicas have sent an acknowledgement. On a secondary replicator, this is semantically the highest sequence number of the in-order operation received from the primary. | [optional] [default to null]
**last_sequence_number** | **String** | Represents the latest sequence number of the operation that is available in the queue. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


