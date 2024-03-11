# StatelessServiceTypeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ServiceKind**](ServiceKind.md) | The kind of service (Stateless or Stateful). | [default to null]
**is_stateful** | **bool** | Indicates whether the service type is a stateful service type or a stateless service type. This property is true if the service type is a stateful service type, false otherwise. | [optional] [default to null]
**service_type_name** | [***::models::ServiceTypeName**](ServiceTypeName.md) | Name of the service type as specified in the service manifest. | [optional] [default to null]
**placement_constraints** | **String** | The placement constraint to be used when instantiating this service in a Service Fabric cluster. | [optional] [default to null]
**load_metrics** | [***::models::ServiceLoadMetricsList**](ServiceLoadMetricsList.md) | The service load metrics is given as an array of ServiceLoadMetricDescription objects. | [optional] [default to null]
**service_placement_policies** | [***::models::ServicePlacementPolicyDescriptionList**](ServicePlacementPolicyDescriptionList.md) | List of service placement policy descriptions. | [optional] [default to null]
**extensions** | [***::models::ServiceTypeExtensionDescriptionList**](ServiceTypeExtensionDescriptionList.md) | List of service type extensions. | [optional] [default to null]
**use_implicit_host** | **bool** | A flag indicating if this type is not implemented and hosted by a user service process, but is implicitly hosted by a system created process. This value is true for services using the guest executable services, false otherwise. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


