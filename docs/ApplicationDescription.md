# ApplicationDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::ApplicationName**](ApplicationName.md) | The name of the application, including the &#39;fabric:&#39; URI scheme. | [default to null]
**type_name** | [***::models::ApplicationTypeName**](ApplicationTypeName.md) | The application type name as defined in the application manifest. | [default to null]
**type_version** | [***::models::ApplicationTypeVersion**](ApplicationTypeVersion.md) | The version of the application type as defined in the application manifest. | [default to null]
**parameter_list** | [***::models::ApplicationParameterList**](ApplicationParameterList.md) | List of application parameters with overridden values from their default values specified in the application manifest. | [optional] [default to null]
**application_capacity** | [***::models::ApplicationCapacityDescription**](ApplicationCapacityDescription.md) | Describes capacity information for services of this application. This description can be used for describing the following. - Reserving the capacity for the services on the nodes - Limiting the total number of nodes that services of this application can run on - Limiting the custom capacity metrics to limit the total consumption of this metric by the services of this application | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


