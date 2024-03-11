# MonitoringPolicyDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failure_action** | [***::models::FailureAction**](FailureAction.md) | The compensating action to perform when a Monitored upgrade encounters monitoring policy or health policy violations. Invalid indicates the failure action is invalid. Rollback specifies that the upgrade will start rolling back automatically. Manual indicates that the upgrade will switch to UnmonitoredManual upgrade mode. | [optional] [default to null]
**health_check_wait_duration_in_milliseconds** | [***::models::HealthCheckWaitDuration**](HealthCheckWaitDuration.md) | The amount of time to wait after completing an upgrade domain before applying health policies. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]
**health_check_stable_duration_in_milliseconds** | [***::models::HealthCheckStableDuration**](HealthCheckStableDuration.md) | The amount of time that the application or cluster must remain healthy before the upgrade proceeds to the next upgrade domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]
**health_check_retry_timeout_in_milliseconds** | [***::models::HealthCheckRetryTimeout**](HealthCheckRetryTimeout.md) | The amount of time to retry health evaluation when the application or cluster is unhealthy before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]
**upgrade_timeout_in_milliseconds** | [***::models::UpgradeTimeout**](UpgradeTimeout.md) | The amount of time the overall upgrade has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]
**upgrade_domain_timeout_in_milliseconds** | [***::models::UpgradeDomainTimeout**](UpgradeDomainTimeout.md) | The amount of time each upgrade domain has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


