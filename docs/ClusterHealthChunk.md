# ClusterHealthChunk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**health_state** | [***::models::HealthState**](HealthState.md) | The HealthState representing the aggregated health state of the cluster computed by Health Manager. The health evaluation of the entity reflects all events reported on the entity and its children (if any). The aggregation is done by applying the desired cluster health policy and the application health policies. | [optional] [default to null]
**node_health_state_chunks** | [***::models::NodeHealthStateChunkList**](NodeHealthStateChunkList.md) | The list of node health state chunks in the cluster that respect the filters in the cluster health chunk query description. | [optional] [default to null]
**application_health_state_chunks** | [***::models::ApplicationHealthStateChunkList**](ApplicationHealthStateChunkList.md) | The list of application health state chunks in the cluster that respect the filters in the cluster health chunk query description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


