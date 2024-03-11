# ServiceFromTemplateDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [default to null]
**service_name** | [***::models::ServiceName**](ServiceName.md) | The full name of the service with &#39;fabric:&#39; URI scheme. | [default to null]
**service_type_name** | [***::models::ServiceTypeName**](ServiceTypeName.md) | Name of the service type as specified in the service manifest. | [default to null]
**initialization_data** | **String** | The initialization data for the newly created service instance. | [optional] [default to null]
**service_package_activation_mode** | [***::models::ServicePackageActivationMode**](ServicePackageActivationMode.md) | The activation mode of service package to be used for a service. | [optional] [default to null]
**service_dns_name** | **String** | The DNS name of the service. It requires the DNS system service to be enabled in Service Fabric cluster. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


