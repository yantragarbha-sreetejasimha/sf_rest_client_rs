# ComposeDeploymentUpgradeProgressInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deployment_name** | [***::models::TargetDeploymentName**](TargetDeploymentName.md) | The name of the target deployment. | [optional] [default to null]
**application_name** | [***::models::TargetApplicationName**](TargetApplicationName.md) | The name of the target application, including the &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**upgrade_state** | [***::models::ComposeDeploymentUpgradeState**](ComposeDeploymentUpgradeState.md) | The state of the compose deployment upgrade. | [optional] [default to null]
**upgrade_status_details** | **String** | Additional detailed information about the status of the pending upgrade. | [optional] [default to null]
**upgrade_kind** | [***::models::UpgradeKind**](UpgradeKind.md) | The kind of upgrade out of the following possible values. | [optional] [default to null]
**rolling_upgrade_mode** | [***::models::UpgradeMode**](UpgradeMode.md) | The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored. | [optional] [default to null]
**force_restart** | [***::models::ForceRestart**](ForceRestart.md) | If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data). | [optional] [default to null]
**upgrade_replica_set_check_timeout_in_seconds** | [***::models::UpgradeReplicaSetCheckTimeout**](UpgradeReplicaSetCheckTimeout.md) | The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer). | [optional] [default to null]
**monitoring_policy** | [***::models::MonitoringPolicyDescription**](MonitoringPolicyDescription.md) | Describes the parameters for monitoring an upgrade in Monitored mode. | [optional] [default to null]
**application_health_policy** | [***::models::ApplicationHealthPolicy**](ApplicationHealthPolicy.md) | Defines a health policy used to evaluate the health of an application or one of its children entities. | [optional] [default to null]
**target_application_type_version** | [***::models::TargetApplicationTypeVersion**](TargetApplicationTypeVersion.md) | The target application type version (found in the application manifest) for the application upgrade. | [optional] [default to null]
**upgrade_duration** | [***::models::UpgradeDuration**](UpgradeDuration.md) | The estimated amount of time that the overall upgrade elapsed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]
**current_upgrade_domain_duration** | [***::models::CurrentUpgradeDomainDuration**](CurrentUpgradeDomainDuration.md) | The estimated amount of time spent processing current Upgrade Domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds. | [optional] [default to null]
**application_unhealthy_evaluations** | [***::models::ApplicationUnhealthyEvaluations**](ApplicationUnhealthyEvaluations.md) | List of health evaluations that resulted in the current aggregated health state. | [optional] [default to null]
**current_upgrade_domain_progress** | [***::models::CurrentUpgradeDomainProgressInfo**](CurrentUpgradeDomainProgressInfo.md) | Information about the current in-progress upgrade domain. | [optional] [default to null]
**start_timestamp_utc** | **String** | The estimated UTC datetime when the upgrade started. | [optional] [default to null]
**failure_timestamp_utc** | **String** | The estimated UTC datetime when the upgrade failed and FailureAction was executed. | [optional] [default to null]
**failure_reason** | [***::models::FailureReason**](FailureReason.md) | The cause of an upgrade failure that resulted in FailureAction being executed. | [optional] [default to null]
**upgrade_domain_progress_at_failure** | [***::models::FailureUpgradeDomainProgressInfo**](FailureUpgradeDomainProgressInfo.md) | Information about the upgrade domain progress at the time of upgrade failure. | [optional] [default to null]
**application_upgrade_status_details** | **String** | Additional details of application upgrade including failure message. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


