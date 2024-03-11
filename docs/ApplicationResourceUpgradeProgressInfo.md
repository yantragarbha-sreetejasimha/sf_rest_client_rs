# ApplicationResourceUpgradeProgressInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the Application resource. | [optional] [default to null]
**target_application_type_version** | **String** | The target application version for the application upgrade. | [optional] [default to null]
**start_timestamp_utc** | **String** | The estimated UTC datetime when the upgrade started. | [optional] [default to null]
**upgrade_state** | [***::models::ApplicationResourceUpgradeState**](ApplicationResourceUpgradeState.md) | The state of the application resource upgrade. | [optional] [default to null]
**percent_completed** | **String** | The estimated percent of replicas are completed in the upgrade. | [optional] [default to null]
**service_upgrade_progress** | [***::models::ServiceUpgradeProgressList**](ServiceUpgradeProgressList.md) | List of service upgrade progresses. | [optional] [default to null]
**rolling_upgrade_mode** | [***::models::RollingUpgradeMode**](RollingUpgradeMode.md) | The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored. | [optional] [default to null]
**upgrade_duration** | **String** | The estimated amount of time that the overall upgrade elapsed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]
**application_upgrade_status_details** | **String** | Additional detailed information about the status of the pending upgrade. | [optional] [default to null]
**upgrade_replica_set_check_timeout_in_seconds** | **i64** | The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer). | [optional] [default to 42949672925]
**failure_timestamp_utc** | **String** | The estimated UTC datetime when the upgrade failed and FailureAction was executed. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


