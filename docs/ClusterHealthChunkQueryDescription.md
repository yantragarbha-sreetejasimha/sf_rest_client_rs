# ClusterHealthChunkQueryDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_filters** | [**Vec<::models::NodeHealthStateFilter>**](NodeHealthStateFilter.md) | Defines a list of filters that specify which nodes to be included in the returned cluster health chunk. If no filters are specified, no nodes are returned. All the nodes are used to evaluate the cluster&#39;s aggregated health state, regardless of the input filters. The cluster health chunk query may specify multiple node filters. For example, it can specify a filter to return all nodes with health state Error and another filter to always include a node identified by its NodeName. | [optional] [default to null]
**application_filters** | [**Vec<::models::ApplicationHealthStateFilter>**](ApplicationHealthStateFilter.md) | Defines a list of filters that specify which applications to be included in the returned cluster health chunk. If no filters are specified, no applications are returned. All the applications are used to evaluate the cluster&#39;s aggregated health state, regardless of the input filters. The cluster health chunk query may specify multiple application filters. For example, it can specify a filter to return all applications with health state Error and another filter to always include applications of a specified application type. | [optional] [default to null]
**cluster_health_policy** | [***::models::ClusterHealthPolicy**](ClusterHealthPolicy.md) | Defines a health policy used to evaluate the health of the cluster or of a cluster node. | [optional] [default to null]
**application_health_policies** | [***::models::ApplicationHealthPolicies**](ApplicationHealthPolicies.md) | Defines the application health policy map used to evaluate the health of an application or one of its children entities. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


