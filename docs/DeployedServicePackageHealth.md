# DeployedServicePackageHealth

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregated_health_state** | [***::models::HealthState**](HealthState.md) | The HealthState representing the aggregated health state of the entity computed by Health Manager. The health evaluation of the entity reflects all events reported on the entity and its children (if any). The aggregation is done by applying the desired health policy. | [optional] [default to null]
**health_events** | [**Vec<::models::HealthEvent>**](HealthEvent.md) | The list of health events reported on the entity. | [optional] [default to null]
**unhealthy_evaluations** | [***::models::UnhealthyEvaluations**](UnhealthyEvaluations.md) | The unhealthy evaluations that show why the current aggregated health state was returned by Health Manager. | [optional] [default to null]
**health_statistics** | [***::models::HealthStatistics**](HealthStatistics.md) | Shows the health statistics for all children types of the queried entity. | [optional] [default to null]
**application_name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**service_manifest_name** | [***::models::ServiceManifestName**](ServiceManifestName.md) | Name of the service manifest. | [optional] [default to null]
**node_name** | [***::models::NodeName**](NodeName.md) | Name of the node where this service package is deployed. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

