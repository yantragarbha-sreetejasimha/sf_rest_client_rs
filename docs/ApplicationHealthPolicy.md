# ApplicationHealthPolicy

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**consider_warning_as_error** | **bool** | Indicates whether warnings are treated with the same severity as errors. | [optional] [default to null]
**max_percent_unhealthy_deployed_applications** | **i32** | The maximum allowed percentage of unhealthy deployed applications. Allowed values are Byte values from zero to 100. The percentage represents the maximum tolerated percentage of deployed applications that can be unhealthy before the application is considered in error. This is calculated by dividing the number of unhealthy deployed applications over the number of nodes where the application is currently deployed on in the cluster. The computation rounds up to tolerate one failure on small numbers of nodes. Default percentage is zero. | [optional] [default to null]
**default_service_type_health_policy** | [***::models::ServiceTypeHealthPolicy**](ServiceTypeHealthPolicy.md) | The health policy used by default to evaluate the health of a service type. | [optional] [default to null]
**service_type_health_policy_map** | [***::models::ServiceTypeHealthPolicyMap**](ServiceTypeHealthPolicyMap.md) | The map with service type health policy per service type name. The map is empty by default. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


