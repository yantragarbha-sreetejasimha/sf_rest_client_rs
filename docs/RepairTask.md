# RepairTask

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the repair task. | [default to null]
**version** | **String** | The version of the repair task. When creating a new repair task, the version must be set to zero.  When updating a repair task, the version is used for optimistic concurrency checks.  If the version is set to zero, the update will not check for write conflicts.  If the version is set to a non-zero value, then the update will only succeed if the actual current version of the repair task matches this value. | [optional] [default to null]
**description** | **String** | A description of the purpose of the repair task, or other informational details. May be set when the repair task is created, and is immutable once set. | [optional] [default to null]
**state** | **String** | The workflow state of the repair task. Valid initial states are Created, Claimed, and Preparing. | [default to null]
**flags** | **i32** | A bitwise-OR of the following values, which gives additional details about the status of the repair task. - 1 - Cancellation of the repair has been requested - 2 - Abort of the repair has been requested - 4 - Approval of the repair was forced via client request | [optional] [default to null]
**action** | **String** | The requested repair action. Must be specified when the repair task is created, and is immutable once set. | [default to null]
**target** | [***::models::RepairTargetDescriptionBase**](RepairTargetDescriptionBase.md) | The target object determines what actions the system will take to prepare for the impact of the repair, prior to approving execution of the repair. May be set when the repair task is created, and is immutable once set. | [optional] [default to null]
**executor** | **String** | The name of the repair executor. Must be specified in Claimed and later states, and is immutable once set. | [optional] [default to null]
**executor_data** | **String** | A data string that the repair executor can use to store its internal state. | [optional] [default to null]
**impact** | [***::models::RepairImpactDescriptionBase**](RepairImpactDescriptionBase.md) | The impact object determines what actions the system will take to prepare for the impact of the repair, prior to approving execution of the repair. Impact must be specified by the repair executor when transitioning to the Preparing state, and is immutable once set. | [optional] [default to null]
**result_status** | **String** | A value describing the overall result of the repair task execution. Must be specified in the Restoring and later states, and is immutable once set. | [optional] [default to null]
**result_code** | **i32** | A numeric value providing additional details about the result of the repair task execution. May be specified in the Restoring and later states, and is immutable once set. | [optional] [default to null]
**result_details** | **String** | A string providing additional details about the result of the repair task execution. May be specified in the Restoring and later states, and is immutable once set. | [optional] [default to null]
**history** | [***::models::RepairTaskHistory**](RepairTaskHistory.md) | An object that contains timestamps of the repair task&#39;s state transitions. These timestamps are updated by the system, and cannot be directly modified. | [optional] [default to null]
**preparing_health_check_state** | [***::models::RepairTaskHealthCheckState**](RepairTaskHealthCheckState.md) | The workflow state of the health check when the repair task is in the Preparing state. | [optional] [default to null]
**restoring_health_check_state** | [***::models::RepairTaskHealthCheckState**](RepairTaskHealthCheckState.md) | The workflow state of the health check when the repair task is in the Restoring state. | [optional] [default to null]
**perform_preparing_health_check** | **bool** | A value to determine if health checks will be performed when the repair task enters the Preparing state. | [optional] [default to null]
**perform_restoring_health_check** | **bool** | A value to determine if health checks will be performed when the repair task enters the Restoring state. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


