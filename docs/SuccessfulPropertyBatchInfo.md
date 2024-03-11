# SuccessfulPropertyBatchInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::PropertyBatchInfoKind**](PropertyBatchInfoKind.md) | The kind of property batch info, determined by the results of a property batch. The following are the possible values. | [default to null]
**properties** | [***Value**](Value.md) | A map containing the properties that were requested through any \&quot;Get\&quot; property batch operations. The key represents the index of the \&quot;Get\&quot; operation in the original request, in string form. The value is the property. If a property is not found, it will not be in the map. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


