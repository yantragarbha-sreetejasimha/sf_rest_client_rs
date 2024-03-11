# ApplicationUpgradeProgressInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::TargetApplicationName**](TargetApplicationName.md) | The name of the target application, including the &#39;fabric:&#39; URI scheme. | [optional] [default to null]
**type_name** | [***::models::ApplicationTypeName**](ApplicationTypeName.md) | The application type name as defined in the application manifest. | [optional] [default to null]
**target_application_type_version** | [***::models::TargetApplicationTypeVersion**](TargetApplicationTypeVersion.md) | The target application type version (found in the application manifest) for the application upgrade. | [optional] [default to null]
**upgrade_domains** | [***::models::UpgradeDomainInfoList**](UpgradeDomainInfoList.md) | List of upgrade domains and their statuses. | [optional] [default to null]
**upgrade_state** | [***::models::UpgradeState**](UpgradeState.md) | The state of the upgrade domain. | [optional] [default to null]
**next_upgrade_domain** | [***::models::NextUpgradeDomain**](NextUpgradeDomain.md) | The name of the next upgrade domain to be processed. | [optional] [default to null]
**rolling_upgrade_mode** | [***::models::UpgradeMode**](UpgradeMode.md) | The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored. | [optional] [default to null]
**upgrade_description** | [***::models::ApplicationUpgradeDescription**](ApplicationUpgradeDescription.md) | Describes the parameters for an application upgrade. Note that upgrade description replaces the existing application description. This means that if the parameters are not specified, the existing parameters on the applications will be overwritten with the empty parameters list. This would result in the application using the default value of the parameters from the application manifest. If you do not want to change any existing parameter values, please get the application parameters first using the GetApplicationInfo query and then supply those values as Parameters in this ApplicationUpgradeDescription. | [optional] [default to null]
**upgrade_duration_in_milliseconds** | **String** | The estimated total amount of time spent processing the overall upgrade. | [optional] [default to null]
**upgrade_domain_duration_in_milliseconds** | **String** | The estimated total amount of time spent processing the current upgrade domain. | [optional] [default to null]
**unhealthy_evaluations** | [***::models::UnhealthyEvaluations**](UnhealthyEvaluations.md) | List of health evaluations that resulted in the current aggregated health state. | [optional] [default to null]
**current_upgrade_domain_progress** | [***::models::CurrentUpgradeDomainProgressInfo**](CurrentUpgradeDomainProgressInfo.md) | Information about the current in-progress upgrade domain. | [optional] [default to null]
**start_timestamp_utc** | **String** | The estimated UTC datetime when the upgrade started. | [optional] [default to null]
**failure_timestamp_utc** | **String** | The estimated UTC datetime when the upgrade failed and FailureAction was executed. | [optional] [default to null]
**failure_reason** | [***::models::FailureReason**](FailureReason.md) | The cause of an upgrade failure that resulted in FailureAction being executed. | [optional] [default to null]
**upgrade_domain_progress_at_failure** | [***::models::FailureUpgradeDomainProgressInfo**](FailureUpgradeDomainProgressInfo.md) | Information about the upgrade domain progress at the time of upgrade failure. | [optional] [default to null]
**upgrade_status_details** | **String** | Additional detailed information about the status of the pending upgrade. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


