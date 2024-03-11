# ReconfigurationInformation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**previous_configuration_role** | [***::models::ReplicaRole**](ReplicaRole.md) | Replica role before reconfiguration started. | [optional] [default to null]
**reconfiguration_phase** | [***::models::ReconfigurationPhase**](ReconfigurationPhase.md) | Current phase of ongoing reconfiguration. If no reconfiguration is taking place then this value will be \&quot;None\&quot;. | [optional] [default to null]
**reconfiguration_type** | [***::models::ReconfigurationType**](ReconfigurationType.md) | Type of current ongoing reconfiguration. If no reconfiguration is taking place then this value will be \&quot;None\&quot;. | [optional] [default to null]
**reconfiguration_start_time_utc** | **String** | Start time (in UTC) of the ongoing reconfiguration. If no reconfiguration is taking place then this value will be zero date-time. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


