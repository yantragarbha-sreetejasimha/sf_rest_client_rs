# ClusterConfigurationUpgradeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_config** | **String** | The cluster configuration as a JSON string. For example, [this file](https://github.com/Azure-Samples/service-fabric-dotnet-standalone-cluster-configuration/blob/master/Samples/ClusterConfig.Unsecure.DevCluster.json) contains JSON describing the [nodes and other properties of the cluster](https://docs.microsoft.com/azure/service-fabric/service-fabric-cluster-manifest). | [default to null]
**health_check_retry_timeout** | **String** | The length of time between attempts to perform health checks if the application or cluster is not healthy. | [optional] [default to null]
**health_check_wait_duration_in_seconds** | **String** | The length of time to wait after completing an upgrade domain before starting the health checks process. | [optional] [default to null]
**health_check_stable_duration_in_seconds** | **String** | The length of time that the application or cluster must remain healthy before the upgrade proceeds to the next upgrade domain. | [optional] [default to null]
**upgrade_domain_timeout_in_seconds** | **String** | The timeout for the upgrade domain. | [optional] [default to null]
**upgrade_timeout_in_seconds** | **String** | The upgrade timeout. | [optional] [default to null]
**max_percent_unhealthy_applications** | **i32** | The maximum allowed percentage of unhealthy applications during the upgrade. Allowed values are integer values from zero to 100. | [optional] [default to null]
**max_percent_unhealthy_nodes** | **i32** | The maximum allowed percentage of unhealthy nodes during the upgrade. Allowed values are integer values from zero to 100. | [optional] [default to null]
**max_percent_delta_unhealthy_nodes** | **i32** | The maximum allowed percentage of delta health degradation during the upgrade. Allowed values are integer values from zero to 100. | [optional] [default to null]
**max_percent_upgrade_domain_delta_unhealthy_nodes** | **i32** | The maximum allowed percentage of upgrade domain delta health degradation during the upgrade. Allowed values are integer values from zero to 100. | [optional] [default to null]
**application_health_policies** | [***::models::ApplicationHealthPolicies**](ApplicationHealthPolicies.md) | Defines the application health policy map used to evaluate the health of an application or one of its children entities. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


