# ServiceLoadMetricDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the metric. If the service chooses to report load during runtime, the load metric name should match the name that is specified in Name exactly. Note that metric names are case-sensitive. | [default to null]
**weight** | [***::models::ServiceLoadMetricWeight**](ServiceLoadMetricWeight.md) | The service load metric relative weight, compared to other metrics configured for this service, as a number. | [optional] [default to null]
**primary_default_load** | **i32** | Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Primary replica. | [optional] [default to null]
**secondary_default_load** | **i32** | Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is a Secondary replica. | [optional] [default to null]
**auxiliary_default_load** | **i32** | Used only for Stateful services. The default amount of load, as a number, that this service creates for this metric when it is an Auxiliary replica. | [optional] [default to null]
**default_load** | **i32** | Used only for Stateless services. The default amount of load, as a number, that this service creates for this metric. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


