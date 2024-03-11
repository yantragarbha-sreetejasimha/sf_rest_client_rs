# LoadedPartitionInformationQueryDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metric_name** | **String** | Name of the metric for which this information is provided. | [optional] [default to null]
**service_name** | **String** | Name of the service this partition belongs to. | [optional] [default to null]
**ordering** | [***::models::Ordering**](Ordering.md) | Ordering of partitions&#39; load. | [optional] [default to null]
**max_results** | **i64** | The maximum number of results to be returned as part of the paged queries. This parameter defines the upper bound on the number of results returned. The results returned can be less than the specified maximum results if they do not fit in the message as per the max message size restrictions defined in the configuration. If this parameter is zero or not specified, the paged query includes as many results as possible that fit in the return message. | [optional] [default to null]
**continuation_token** | [***::models::ContinuationToken**](ContinuationToken.md) | The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


