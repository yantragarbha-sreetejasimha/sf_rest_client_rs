# BasicRetentionPolicyDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retention_policy_type** | [***::models::RetentionPolicyType**](RetentionPolicyType.md) | The type of retention policy. Currently only \&quot;Basic\&quot; retention policy is supported. | [default to null]
**retention_duration** | **String** | It is the minimum duration for which a backup created, will remain stored in the storage and might get deleted after that span of time. It should be specified in ISO8601 format. | [default to null]
**minimum_number_of_backups** | **i32** | It is the minimum number of backups to be retained at any point of time. If specified with a non zero value, backups will not be deleted even if the backups have gone past retention duration and have number of backups less than or equal to it. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


