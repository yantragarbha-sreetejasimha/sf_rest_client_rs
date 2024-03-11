# AverageServiceLoadScalingTrigger

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::ScalingTriggerKind**](ScalingTriggerKind.md) | Specifies the kind of scaling trigger | [default to null]
**metric_name** | **String** | The name of the metric for which usage should be tracked. | [default to null]
**lower_load_threshold** | **String** | The lower limit of the load below which a scale in operation should be performed. | [default to null]
**upper_load_threshold** | **String** | The upper limit of the load beyond which a scale out operation should be performed. | [default to null]
**scale_interval_in_seconds** | **i64** | The period in seconds on which a decision is made whether to scale or not. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


