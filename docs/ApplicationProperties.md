# ApplicationProperties

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | User readable description of the application. | [optional] [default to null]
**debug_params** | **String** | Internal use. | [optional] [default to null]
**services** | [**Vec<::models::ServiceResourceDescription>**](ServiceResourceDescription.md) | describes the services in the application. | [optional] [default to null]
**health_state** | [***::models::HealthState**](HealthState.md) | Describes the health state of an application resource. | [optional] [default to null]
**unhealthy_evaluation** | **String** | When the application&#39;s health state is not &#39;Ok&#39;, this additional details from service fabric Health Manager for the user to know why the application is marked unhealthy. | [optional] [default to null]
**status** | **String** | Status of the application resource. | [optional] [default to null]
**status_details** | **String** | Gives additional information about the current status of the application deployment. | [optional] [default to null]
**service_names** | **Vec<String>** | Names of the services in the application. | [optional] [default to null]
**diagnostics** | [***::models::DiagnosticsDescription**](DiagnosticsDescription.md) | Describes the diagnostics definition and usage for an application resource. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


