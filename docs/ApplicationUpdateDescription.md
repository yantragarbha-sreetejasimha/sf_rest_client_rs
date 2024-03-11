# ApplicationUpdateDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flags** | **String** | Flags indicating whether other properties are set. Each of the associated properties corresponds to a flag, specified below, which, if set, indicate that the property is specified. If flags are not specified for a certain property, the property will not be updated even if the new value is provided. This property can be a combination of those flags obtained using bitwise &#39;OR&#39; operator. Exception is RemoveApplicationCapacity which cannot be specified along with other parameters. For example, if the provided value is 3 then the flags for MinimumNodes (1) and MaximumNodes (2) are set.  - None - Does not indicate any other properties are set. The value is 0. - MinimumNodes - Indicates whether the MinimumNodes property is set. The value is 1. - MaximumNodes - Indicates whether the MinimumNodes property is set. The value is  2. - ApplicationMetrics - Indicates whether the ApplicationMetrics property is set. The value is 4. | [optional] [default to null]
**remove_application_capacity** | **bool** | Used to clear all parameters related to Application Capacity for this application. | It is not possible to specify this parameter together with other Application Capacity parameters. | [optional] [default to null]
**minimum_nodes** | **i64** | The minimum number of nodes where Service Fabric will reserve capacity for this application. Note that this does not mean that the services of this application will be placed on all of those nodes. If this property is set to zero, no capacity will be reserved. The value of this property cannot be more than the value of the MaximumNodes property. | [optional] [default to null]
**maximum_nodes** | **i64** | The maximum number of nodes where Service Fabric will reserve capacity for this application. Note that this does not mean that the services of this application will be placed on all of those nodes. By default, the value of this property is zero and it means that the services can be placed on any node. | [optional] [default to 0]
**application_metrics** | [***::models::ApplicationMetricDescriptionList**](ApplicationMetricDescriptionList.md) | List of application capacity metric description. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


