# ResolvedServicePartition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::ServiceName**](ServiceName.md) | The full name of the service with &#39;fabric:&#39; URI scheme. | [default to null]
**partition_information** | [***::models::PartitionInformation**](PartitionInformation.md) | A representation of the resolved partition. | [default to null]
**endpoints** | [***::models::ResolvedServiceEndpointList**](ResolvedServiceEndpointList.md) | List of resolved service endpoints of a service partition. | [default to null]
**version** | **String** | The version of this resolved service partition result. This version should be passed in the next time the ResolveService call is made via the PreviousRspVersion query parameter. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


