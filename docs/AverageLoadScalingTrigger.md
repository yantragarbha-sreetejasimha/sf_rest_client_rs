# AverageLoadScalingTrigger

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [***::models::AutoScalingTriggerKind**](AutoScalingTriggerKind.md) | The type of auto scaling trigger | [default to null]
**metric** | [***::models::AutoScalingMetric**](AutoScalingMetric.md) | Description of the metric that is used for scaling. | [default to null]
**lower_load_threshold** | **f64** | Lower load threshold (if average load is below this threshold, service will scale down). | [default to null]
**upper_load_threshold** | **f64** | Upper load threshold (if average load is above this threshold, service will scale up). | [default to null]
**scale_interval_in_seconds** | **i32** | Scale interval that indicates how often will this trigger be checked. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


