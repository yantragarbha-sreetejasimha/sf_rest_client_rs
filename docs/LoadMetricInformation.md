# LoadMetricInformation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the metric for which this load information is provided. | [optional] [default to null]
**is_balanced_before** | **bool** | Value that indicates whether the metrics is balanced or not before resource balancer run | [optional] [default to null]
**is_balanced_after** | **bool** | Value that indicates whether the metrics is balanced or not after resource balancer run. | [optional] [default to null]
**deviation_before** | **String** | The standard average deviation of the metrics before resource balancer run. | [optional] [default to null]
**deviation_after** | **String** | The standard average deviation of the metrics after resource balancer run. | [optional] [default to null]
**balancing_threshold** | **String** | The balancing threshold for a certain metric. | [optional] [default to null]
**action** | **String** | The current action being taken with regard to this metric | [optional] [default to null]
**activity_threshold** | **String** | The Activity Threshold specified for this metric in the system Cluster Manifest. | [optional] [default to null]
**cluster_capacity** | **String** | The total cluster capacity for a given metric | [optional] [default to null]
**cluster_load** | **String** | The total cluster load. In future releases of Service Fabric this parameter will be deprecated in favor of CurrentClusterLoad. | [optional] [default to null]
**current_cluster_load** | **String** | The total cluster load. | [optional] [default to null]
**cluster_remaining_capacity** | **String** | The remaining capacity for the metric in the cluster. In future releases of Service Fabric this parameter will be deprecated in favor of ClusterCapacityRemaining. | [optional] [default to null]
**cluster_capacity_remaining** | **String** | The remaining capacity for the metric in the cluster. | [optional] [default to null]
**is_cluster_capacity_violation** | **bool** | Indicates that the metric is currently over capacity in the cluster. | [optional] [default to null]
**node_buffer_percentage** | **String** | The reserved percentage of total node capacity for this metric. | [optional] [default to null]
**cluster_buffered_capacity** | **String** | Remaining capacity in the cluster excluding the reserved space. In future releases of Service Fabric this parameter will be deprecated in favor of BufferedClusterCapacityRemaining. | [optional] [default to null]
**buffered_cluster_capacity_remaining** | **String** | Remaining capacity in the cluster excluding the reserved space. | [optional] [default to null]
**cluster_remaining_buffered_capacity** | **String** | The remaining percentage of cluster total capacity for this metric. | [optional] [default to null]
**min_node_load_value** | **String** | The minimum load on any node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of MinimumNodeLoad. | [optional] [default to null]
**minimum_node_load** | **String** | The minimum load on any node for this metric. | [optional] [default to null]
**min_node_load_node_id** | [***::models::NodeId**](NodeId.md) | The node id of the node with the minimum load for this metric. | [optional] [default to null]
**max_node_load_value** | **String** | The maximum load on any node for this metric. In future releases of Service Fabric this parameter will be deprecated in favor of MaximumNodeLoad. | [optional] [default to null]
**maximum_node_load** | **String** | The maximum load on any node for this metric. | [optional] [default to null]
**max_node_load_node_id** | [***::models::NodeId**](NodeId.md) | The node id of the node with the maximum load for this metric. | [optional] [default to null]
**planned_load_removal** | **String** | This value represents the load of the replicas that are planned to be removed in the future within the cluster. This kind of load is reported for replicas that are currently being moving to other nodes and for replicas that are currently being dropped but still use the load on the source node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


