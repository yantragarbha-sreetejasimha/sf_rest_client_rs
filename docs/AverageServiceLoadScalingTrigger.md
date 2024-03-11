# AverageServiceLoadScalingTrigger

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ScalingTriggerKind**](ScalingTriggerKind.md) | Specifies the kind of scaling trigger | [default to null]
**metric_name** | **String** | The name of the metric for which usage should be tracked. | [default to null]
**lower_load_threshold** | **String** | The lower limit of the load below which a scale in operation should be performed. | [default to null]
**upper_load_threshold** | **String** | The upper limit of the load beyond which a scale out operation should be performed. | [default to null]
**scale_interval_in_seconds** | **i64** | The period in seconds on which a decision is made whether to scale or not. | [default to null]
**use_only_primary_load** | **bool** | Flag determines whether only the load of primary replica should be considered for scaling. If set to true, then trigger will only consider the load of primary replicas of stateful service.  If set to false, trigger will consider load of all replicas.  This parameter cannot be set to true for stateless service. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


