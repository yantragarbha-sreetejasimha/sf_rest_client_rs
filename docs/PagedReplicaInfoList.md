# PagedReplicaInfoList

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**continuation_token** | [***::models::ContinuationToken**](ContinuationToken.md) | The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response. | [optional] [default to null]
**items** | [**Vec<::models::ReplicaInfo>**](ReplicaInfo.md) | List of replica information. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


