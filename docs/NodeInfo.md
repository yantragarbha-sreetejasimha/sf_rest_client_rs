# NodeInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::NodeName**](NodeName.md) | The name of a Service Fabric node. | [optional] [default to null]
**ip_address_or_fqdn** | **String** | The IP address or fully qualified domain name of the node. | [optional] [default to null]
**_type** | **String** | The type of the node. | [optional] [default to null]
**code_version** | **String** | The version of Service Fabric binaries that the node is running. | [optional] [default to null]
**config_version** | **String** | The version of Service Fabric cluster manifest that the node is using. | [optional] [default to null]
**node_status** | [***::models::NodeStatus**](NodeStatus.md) | The status of the node. | [optional] [default to null]
**node_up_time_in_seconds** | **String** | Time in seconds since the node has been in NodeStatus Up. Value zero indicates that the node is not Up. | [optional] [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc. | [optional] [default to null]
**is_seed_node** | **bool** | Indicates if the node is a seed node or not. Returns true if the node is a seed node, otherwise false. A quorum of seed nodes are required for proper operation of Service Fabric cluster. | [optional] [default to null]
**upgrade_domain** | **String** | The upgrade domain of the node. | [optional] [default to null]
**fault_domain** | **String** | The fault domain of the node. | [optional] [default to null]
**id** | [***::models::NodeId**](NodeId.md) | An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name. | [optional] [default to null]
**instance_id** | **String** | The ID representing the node instance. While the ID of the node is deterministically generated from the node name and remains same across restarts, the InstanceId changes every time node restarts. | [optional] [default to null]
**node_deactivation_info** | [***::models::NodeDeactivationInfo**](NodeDeactivationInfo.md) | Information about the node deactivation. This information is valid for a node that is undergoing deactivation or has already been deactivated. | [optional] [default to null]
**is_stopped** | **bool** | Indicates if the node is stopped by calling stop node API or not. Returns true if the node is stopped, otherwise false. | [optional] [default to null]
**node_down_time_in_seconds** | **String** | Time in seconds since the node has been in NodeStatus Down. Value zero indicates node is not NodeStatus Down. | [optional] [default to null]
**node_up_at** | **String** | Date time in UTC when the node came up. If the node has never been up then this value will be zero date time. | [optional] [default to null]
**node_down_at** | **String** | Date time in UTC when the node went down. If node has never been down then this value will be zero date time. | [optional] [default to null]
**node_tags** | [***::models::NodeTagsList**](NodeTagsList.md) | List that contains tags, which will be applied to the nodes. | [optional] [default to null]
**is_node_by_node_upgrade_in_progress** | **bool** | Indicates if a node-by-node upgrade is currently being performed on this node. | [optional] [default to null]
**infrastructure_placement_id** | **String** | PlacementID used by the InfrastructureService. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


