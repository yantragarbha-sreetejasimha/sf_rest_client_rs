/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairTask : Represents a repair task, which includes information about what kind of repair was requested, what its progress is, and what its final result was.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairTask {
    /// The ID of the repair task.
    #[serde(rename = "TaskId")]
    task_id: String,
    /// The version of the repair task. When creating a new repair task, the version must be set to zero.  When updating a repair task, the version is used for optimistic concurrency checks.  If the version is set to zero, the update will not check for write conflicts.  If the version is set to a non-zero value, then the update will only succeed if the actual current version of the repair task matches this value.
    #[serde(rename = "Version")]
    version: Option<String>,
    /// A description of the purpose of the repair task, or other informational details. May be set when the repair task is created, and is immutable once set.
    #[serde(rename = "Description")]
    description: Option<String>,
    /// The workflow state of the repair task. Valid initial states are Created, Claimed, and Preparing.
    #[serde(rename = "State")]
    state: String,
    /// A bitwise-OR of the following values, which gives additional details about the status of the repair task. - 1 - Cancellation of the repair has been requested - 2 - Abort of the repair has been requested - 4 - Approval of the repair was forced via client request
    #[serde(rename = "Flags")]
    flags: Option<i32>,
    /// The requested repair action. Must be specified when the repair task is created, and is immutable once set.
    #[serde(rename = "Action")]
    action: String,
    /// The target object determines what actions the system will take to prepare for the impact of the repair, prior to approving execution of the repair. May be set when the repair task is created, and is immutable once set.
    #[serde(rename = "Target")]
    target: Option<::models::RepairTargetDescriptionBase>,
    /// The name of the repair executor. Must be specified in Claimed and later states, and is immutable once set.
    #[serde(rename = "Executor")]
    executor: Option<String>,
    /// A data string that the repair executor can use to store its internal state.
    #[serde(rename = "ExecutorData")]
    executor_data: Option<String>,
    /// The impact object determines what actions the system will take to prepare for the impact of the repair, prior to approving execution of the repair. Impact must be specified by the repair executor when transitioning to the Preparing state, and is immutable once set.
    #[serde(rename = "Impact")]
    impact: Option<::models::RepairImpactDescriptionBase>,
    /// A value describing the overall result of the repair task execution. Must be specified in the Restoring and later states, and is immutable once set.
    #[serde(rename = "ResultStatus")]
    result_status: Option<String>,
    /// A numeric value providing additional details about the result of the repair task execution. May be specified in the Restoring and later states, and is immutable once set.
    #[serde(rename = "ResultCode")]
    result_code: Option<i32>,
    /// A string providing additional details about the result of the repair task execution. May be specified in the Restoring and later states, and is immutable once set.
    #[serde(rename = "ResultDetails")]
    result_details: Option<String>,
    /// An object that contains timestamps of the repair task's state transitions. These timestamps are updated by the system, and cannot be directly modified.
    #[serde(rename = "History")]
    history: Option<::models::RepairTaskHistory>,
    /// The workflow state of the health check when the repair task is in the Preparing state.
    #[serde(rename = "PreparingHealthCheckState")]
    preparing_health_check_state: Option<::models::RepairTaskHealthCheckState>,
    /// The workflow state of the health check when the repair task is in the Restoring state.
    #[serde(rename = "RestoringHealthCheckState")]
    restoring_health_check_state: Option<::models::RepairTaskHealthCheckState>,
    /// A value to determine if health checks will be performed when the repair task enters the Preparing state.
    #[serde(rename = "PerformPreparingHealthCheck")]
    perform_preparing_health_check: Option<bool>,
    /// A value to determine if health checks will be performed when the repair task enters the Restoring state.
    #[serde(rename = "PerformRestoringHealthCheck")]
    perform_restoring_health_check: Option<bool>,
}

impl RepairTask {
    /// Represents a repair task, which includes information about what kind of repair was requested, what its progress is, and what its final result was.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.
    pub fn new(task_id: String, state: String, action: String) -> RepairTask {
        RepairTask {
            task_id,
            version: None,
            description: None,
            state,
            flags: None,
            action,
            target: None,
            executor: None,
            executor_data: None,
            impact: None,
            result_status: None,
            result_code: None,
            result_details: None,
            history: None,
            preparing_health_check_state: None,
            restoring_health_check_state: None,
            perform_preparing_health_check: None,
            perform_restoring_health_check: None,
        }
    }

    pub fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }

    pub fn with_task_id(mut self, task_id: String) -> RepairTask {
        self.task_id = task_id;
        self
    }

    pub fn task_id(&self) -> &String {
        &self.task_id
    }

    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn with_version(mut self, version: String) -> RepairTask {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn reset_version(&mut self) {
        self.version = None;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(mut self, description: String) -> RepairTask {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    pub fn with_state(mut self, state: String) -> RepairTask {
        self.state = state;
        self
    }

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn set_flags(&mut self, flags: i32) {
        self.flags = Some(flags);
    }

    pub fn with_flags(mut self, flags: i32) -> RepairTask {
        self.flags = Some(flags);
        self
    }

    pub fn flags(&self) -> Option<&i32> {
        self.flags.as_ref()
    }

    pub fn reset_flags(&mut self) {
        self.flags = None;
    }

    pub fn set_action(&mut self, action: String) {
        self.action = action;
    }

    pub fn with_action(mut self, action: String) -> RepairTask {
        self.action = action;
        self
    }

    pub fn action(&self) -> &String {
        &self.action
    }

    pub fn set_target(
        &mut self,
        target: ::models::RepairTargetDescriptionBase,
    ) {
        self.target = Some(target);
    }

    pub fn with_target(
        mut self,
        target: ::models::RepairTargetDescriptionBase,
    ) -> RepairTask {
        self.target = Some(target);
        self
    }

    pub fn target(&self) -> Option<&::models::RepairTargetDescriptionBase> {
        self.target.as_ref()
    }

    pub fn reset_target(&mut self) {
        self.target = None;
    }

    pub fn set_executor(&mut self, executor: String) {
        self.executor = Some(executor);
    }

    pub fn with_executor(mut self, executor: String) -> RepairTask {
        self.executor = Some(executor);
        self
    }

    pub fn executor(&self) -> Option<&String> {
        self.executor.as_ref()
    }

    pub fn reset_executor(&mut self) {
        self.executor = None;
    }

    pub fn set_executor_data(&mut self, executor_data: String) {
        self.executor_data = Some(executor_data);
    }

    pub fn with_executor_data(mut self, executor_data: String) -> RepairTask {
        self.executor_data = Some(executor_data);
        self
    }

    pub fn executor_data(&self) -> Option<&String> {
        self.executor_data.as_ref()
    }

    pub fn reset_executor_data(&mut self) {
        self.executor_data = None;
    }

    pub fn set_impact(
        &mut self,
        impact: ::models::RepairImpactDescriptionBase,
    ) {
        self.impact = Some(impact);
    }

    pub fn with_impact(
        mut self,
        impact: ::models::RepairImpactDescriptionBase,
    ) -> RepairTask {
        self.impact = Some(impact);
        self
    }

    pub fn impact(&self) -> Option<&::models::RepairImpactDescriptionBase> {
        self.impact.as_ref()
    }

    pub fn reset_impact(&mut self) {
        self.impact = None;
    }

    pub fn set_result_status(&mut self, result_status: String) {
        self.result_status = Some(result_status);
    }

    pub fn with_result_status(mut self, result_status: String) -> RepairTask {
        self.result_status = Some(result_status);
        self
    }

    pub fn result_status(&self) -> Option<&String> {
        self.result_status.as_ref()
    }

    pub fn reset_result_status(&mut self) {
        self.result_status = None;
    }

    pub fn set_result_code(&mut self, result_code: i32) {
        self.result_code = Some(result_code);
    }

    pub fn with_result_code(mut self, result_code: i32) -> RepairTask {
        self.result_code = Some(result_code);
        self
    }

    pub fn result_code(&self) -> Option<&i32> {
        self.result_code.as_ref()
    }

    pub fn reset_result_code(&mut self) {
        self.result_code = None;
    }

    pub fn set_result_details(&mut self, result_details: String) {
        self.result_details = Some(result_details);
    }

    pub fn with_result_details(mut self, result_details: String) -> RepairTask {
        self.result_details = Some(result_details);
        self
    }

    pub fn result_details(&self) -> Option<&String> {
        self.result_details.as_ref()
    }

    pub fn reset_result_details(&mut self) {
        self.result_details = None;
    }

    pub fn set_history(&mut self, history: ::models::RepairTaskHistory) {
        self.history = Some(history);
    }

    pub fn with_history(
        mut self,
        history: ::models::RepairTaskHistory,
    ) -> RepairTask {
        self.history = Some(history);
        self
    }

    pub fn history(&self) -> Option<&::models::RepairTaskHistory> {
        self.history.as_ref()
    }

    pub fn reset_history(&mut self) {
        self.history = None;
    }

    pub fn set_preparing_health_check_state(
        &mut self,
        preparing_health_check_state: ::models::RepairTaskHealthCheckState,
    ) {
        self.preparing_health_check_state = Some(preparing_health_check_state);
    }

    pub fn with_preparing_health_check_state(
        mut self,
        preparing_health_check_state: ::models::RepairTaskHealthCheckState,
    ) -> RepairTask {
        self.preparing_health_check_state = Some(preparing_health_check_state);
        self
    }

    pub fn preparing_health_check_state(
        &self,
    ) -> Option<&::models::RepairTaskHealthCheckState> {
        self.preparing_health_check_state.as_ref()
    }

    pub fn reset_preparing_health_check_state(&mut self) {
        self.preparing_health_check_state = None;
    }

    pub fn set_restoring_health_check_state(
        &mut self,
        restoring_health_check_state: ::models::RepairTaskHealthCheckState,
    ) {
        self.restoring_health_check_state = Some(restoring_health_check_state);
    }

    pub fn with_restoring_health_check_state(
        mut self,
        restoring_health_check_state: ::models::RepairTaskHealthCheckState,
    ) -> RepairTask {
        self.restoring_health_check_state = Some(restoring_health_check_state);
        self
    }

    pub fn restoring_health_check_state(
        &self,
    ) -> Option<&::models::RepairTaskHealthCheckState> {
        self.restoring_health_check_state.as_ref()
    }

    pub fn reset_restoring_health_check_state(&mut self) {
        self.restoring_health_check_state = None;
    }

    pub fn set_perform_preparing_health_check(
        &mut self,
        perform_preparing_health_check: bool,
    ) {
        self.perform_preparing_health_check =
            Some(perform_preparing_health_check);
    }

    pub fn with_perform_preparing_health_check(
        mut self,
        perform_preparing_health_check: bool,
    ) -> RepairTask {
        self.perform_preparing_health_check =
            Some(perform_preparing_health_check);
        self
    }

    pub fn perform_preparing_health_check(&self) -> Option<&bool> {
        self.perform_preparing_health_check.as_ref()
    }

    pub fn reset_perform_preparing_health_check(&mut self) {
        self.perform_preparing_health_check = None;
    }

    pub fn set_perform_restoring_health_check(
        &mut self,
        perform_restoring_health_check: bool,
    ) {
        self.perform_restoring_health_check =
            Some(perform_restoring_health_check);
    }

    pub fn with_perform_restoring_health_check(
        mut self,
        perform_restoring_health_check: bool,
    ) -> RepairTask {
        self.perform_restoring_health_check =
            Some(perform_restoring_health_check);
        self
    }

    pub fn perform_restoring_health_check(&self) -> Option<&bool> {
        self.perform_restoring_health_check.as_ref()
    }

    pub fn reset_perform_restoring_health_check(&mut self) {
        self.perform_restoring_health_check = None;
    }
}
