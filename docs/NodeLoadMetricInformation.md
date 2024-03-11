# NodeLoadMetricInformation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the metric for which this load information is provided. | [optional] [default to null]
**node_capacity** | **String** | Total capacity on the node for this metric. | [optional] [default to null]
**node_load** | **String** | Current load on the node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of CurrentNodeLoad. | [optional] [default to null]
**node_remaining_capacity** | **String** | The remaining capacity on the node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of NodeCapacityRemaining. | [optional] [default to null]
**is_capacity_violation** | **bool** | Indicates if there is a capacity violation for this metric on the node. | [optional] [default to null]
**node_buffered_capacity** | **String** | The value that indicates the reserved capacity for this metric on the node. | [optional] [default to null]
**node_remaining_buffered_capacity** | **String** | The remaining reserved capacity for this metric on the node. In future releases of Service Fabric this parameter will be deprecated in favor of BufferedNodeCapacityRemaining. | [optional] [default to null]
**current_node_load** | **String** | Current load on the node for this metric. | [optional] [default to null]
**node_capacity_remaining** | **String** | The remaining capacity on the node for the metric. | [optional] [default to null]
**buffered_node_capacity_remaining** | **String** | The remaining capacity which is not reserved by NodeBufferPercentage for this metric on the node. | [optional] [default to null]
**planned_node_load_removal** | **String** | This value represents the load of the replicas that are planned to be removed in the future. This kind of load is reported for replicas that are currently being moving to other nodes and for replicas that are currently being dropped but still use the load on the source node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


