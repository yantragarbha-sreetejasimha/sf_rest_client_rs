# ClusterHealthPolicies

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_health_policy_map** | [***::models::ApplicationHealthPolicyMap**](ApplicationHealthPolicyMap.md) | Defines a map that contains specific application health policies for different applications. Each entry specifies as key the application name and as value an ApplicationHealthPolicy used to evaluate the application health. If an application is not specified in the map, the application health evaluation uses the ApplicationHealthPolicy found in its application manifest or the default application health policy (if no health policy is defined in the manifest). The map is empty by default. | [optional] [default to null]
**cluster_health_policy** | [***::models::ClusterHealthPolicy**](ClusterHealthPolicy.md) | Defines a health policy used to evaluate the health of the cluster or of a cluster node. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


