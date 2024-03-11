# NodeHealthStateFilter

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_name_filter** | **String** | Name of the node that matches the filter. The filter is applied only to the specified node, if it exists. If the node doesn&#39;t exist, no node is returned in the cluster health chunk based on this filter. If the node exists, it is included in the cluster health chunk if the health state matches the other filter properties. If not specified, all nodes that match the parent filters (if any) are taken into consideration and matched against the other filter members, like health state filter. | [optional] [default to null]
**health_state_filter** | **i32** | The filter for the health state of the nodes. It allows selecting nodes if they match the desired health states. The possible values are integer value of one of the following health states. Only nodes that match the filter are returned. All nodes are used to evaluate the cluster aggregated health state. If not specified, default value is None, unless the node name is specified. If the filter has default value and node name is specified, the matching node is returned. The state values are flag-based enumeration, so the value could be a combination of these values obtained using bitwise &#39;OR&#39; operator. For example, if the provided value is 6, it matches nodes with HealthState value of OK (2) and Warning (4).  - Default - Default value. Matches any HealthState. The value is zero. - None - Filter that doesn&#39;t match any HealthState value. Used in order to return no results on a given collection of states. The value is 1. - Ok - Filter that matches input with HealthState value Ok. The value is 2. - Warning - Filter that matches input with HealthState value Warning. The value is 4. - Error - Filter that matches input with HealthState value Error. The value is 8. - All - Filter that matches input with any HealthState value. The value is 65535. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


