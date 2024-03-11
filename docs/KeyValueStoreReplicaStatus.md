# KeyValueStoreReplicaStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ReplicaKind**](ReplicaKind.md) | The role of a replica of a stateful service. | [default to null]
**database_row_count_estimate** | **String** | Value indicating the estimated number of rows in the underlying database. | [optional] [default to null]
**database_logical_size_estimate** | **String** | Value indicating the estimated size of the underlying database. | [optional] [default to null]
**copy_notification_current_key_filter** | **String** | Value indicating the latest key-prefix filter applied to enumeration during the callback. Null if there is no pending callback. | [optional] [default to null]
**copy_notification_current_progress** | **String** | Value indicating the latest number of keys enumerated during the callback. 0 if there is no pending callback. | [optional] [default to null]
**status_details** | **String** | Value indicating the current status details of the replica. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


