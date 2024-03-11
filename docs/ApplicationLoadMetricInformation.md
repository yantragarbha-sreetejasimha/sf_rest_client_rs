# ApplicationLoadMetricInformation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the metric. | [optional] [default to null]
**reservation_capacity** | **i64** | This is the capacity reserved in the cluster for the application. It&#39;s the product of NodeReservationCapacity and MinimumNodes. If set to zero, no capacity is reserved for this metric. When setting application capacity or when updating application capacity this value must be smaller than or equal to MaximumCapacity for each metric. | [optional] [default to null]
**application_capacity** | **i64** | Total capacity for this metric in this application instance. | [optional] [default to null]
**application_load** | **i64** | Current load for this metric in this application instance. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


