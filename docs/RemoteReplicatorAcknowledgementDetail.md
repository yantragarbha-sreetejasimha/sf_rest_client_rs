# RemoteReplicatorAcknowledgementDetail

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**average_receive_duration** | **String** | Represents the average duration it takes for the remote replicator to receive an operation. | [optional] [default to null]
**average_apply_duration** | **String** | Represents the average duration it takes for the remote replicator to apply an operation. This usually entails writing the operation to disk. | [optional] [default to null]
**not_received_count** | **String** | Represents the number of operations not yet received by a remote replicator. | [optional] [default to null]
**received_and_not_applied_count** | **String** | Represents the number of operations received and not yet applied by a remote replicator. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


