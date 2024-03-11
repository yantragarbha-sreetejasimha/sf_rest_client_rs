# ApplicationUpgradeDescription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [***::models::TargetApplicationName**](TargetApplicationName.md) | The name of the target application, including the &#39;fabric:&#39; URI scheme. | [default to null]
**target_application_type_version** | [***::models::TargetApplicationTypeVersion**](TargetApplicationTypeVersion.md) | The target application type version (found in the application manifest) for the application upgrade. | [default to null]
**parameters** | [***::models::ApplicationParameterList**](ApplicationParameterList.md) | List of application parameters with overridden values from their default values specified in the application manifest. | [default to null]
**upgrade_kind** | [***::models::UpgradeKind**](UpgradeKind.md) | The kind of upgrade out of the following possible values. | [default to null]
**rolling_upgrade_mode** | [***::models::UpgradeMode**](UpgradeMode.md) | The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored. | [optional] [default to null]
**upgrade_replica_set_check_timeout_in_seconds** | [***::models::UpgradeReplicaSetCheckTimeout**](UpgradeReplicaSetCheckTimeout.md) | The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer). | [optional] [default to null]
**force_restart** | [***::models::ForceRestart**](ForceRestart.md) | If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data). | [optional] [default to null]
**monitoring_policy** | [***::models::MonitoringPolicyDescription**](MonitoringPolicyDescription.md) | Describes the parameters for monitoring an upgrade in Monitored mode. | [optional] [default to null]
**application_health_policy** | [***::models::ApplicationHealthPolicy**](ApplicationHealthPolicy.md) | Defines a health policy used to evaluate the health of an application or one of its children entities. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


