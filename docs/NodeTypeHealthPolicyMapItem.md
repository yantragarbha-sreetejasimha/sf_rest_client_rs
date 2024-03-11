# NodeTypeHealthPolicyMapItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | The key of the node type health policy map item. This is the name of the node type. | [default to null]
**value** | **i32** | The value of the node type health policy map item. If the percentage is respected but there is at least one unhealthy node in the node type, the health is evaluated as Warning.  The percentage is calculated by dividing the number of unhealthy nodes over the total number of nodes in the node type.  The computation rounds up to tolerate one failure on small numbers of nodes. The max percent unhealthy nodes allowed for the node type. Must be between zero and 100. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


