/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationProcessExitedEvent : Process Exited event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationProcessExitedEvent {
    /// The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\", the application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions.
    #[serde(rename = "ApplicationId")]
    application_id: Option<::models::ApplicationId>,
    /// The kind of FabricEvent.
    #[serde(rename = "Kind")]
    kind: ::models::FabricEventKind,
    /// The identifier for the FabricEvent instance.
    #[serde(rename = "EventInstanceId")]
    event_instance_id: String,
    /// The category of event.
    #[serde(rename = "Category")]
    category: Option<String>,
    /// The time event was logged.
    #[serde(rename = "TimeStamp")]
    time_stamp: String,
    /// Shows there is existing related events available.
    #[serde(rename = "HasCorrelatedEvents")]
    has_correlated_events: Option<bool>,
    /// Name of Service.
    #[serde(rename = "ServiceName")]
    service_name: String,
    /// Name of Service package.
    #[serde(rename = "ServicePackageName")]
    service_package_name: String,
    /// Activation Id of Service package.
    #[serde(rename = "ServicePackageActivationId")]
    service_package_activation_id: String,
    /// Indicates IsExclusive flag.
    #[serde(rename = "IsExclusive")]
    is_exclusive: bool,
    /// Name of Code package.
    #[serde(rename = "CodePackageName")]
    code_package_name: String,
    /// Type of EntryPoint.
    #[serde(rename = "EntryPointType")]
    entry_point_type: String,
    /// Name of executable.
    #[serde(rename = "ExeName")]
    exe_name: String,
    /// Process Id.
    #[serde(rename = "ProcessId")]
    process_id: i64,
    /// Host Id.
    #[serde(rename = "HostId")]
    host_id: String,
    /// Exit code of process.
    #[serde(rename = "ExitCode")]
    exit_code: i64,
    /// Indicates if termination is unexpected.
    #[serde(rename = "UnexpectedTermination")]
    unexpected_termination: bool,
    /// Start time of process.
    #[serde(rename = "StartTime")]
    start_time: String,
}

impl ApplicationProcessExitedEvent {
    /// Process Exited event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        service_name: String,
        service_package_name: String,
        service_package_activation_id: String,
        is_exclusive: bool,
        code_package_name: String,
        entry_point_type: String,
        exe_name: String,
        process_id: i64,
        host_id: String,
        exit_code: i64,
        unexpected_termination: bool,
        start_time: String,
    ) -> ApplicationProcessExitedEvent {
        ApplicationProcessExitedEvent {
            application_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            service_name,
            service_package_name,
            service_package_activation_id,
            is_exclusive,
            code_package_name,
            entry_point_type,
            exe_name,
            process_id,
            host_id,
            exit_code,
            unexpected_termination,
            start_time,
        }
    }

    pub fn set_application_id(
        &mut self,
        application_id: ::models::ApplicationId,
    ) {
        self.application_id = Some(application_id);
    }

    pub fn with_application_id(
        mut self,
        application_id: ::models::ApplicationId,
    ) -> ApplicationProcessExitedEvent {
        self.application_id = Some(application_id);
        self
    }

    pub fn application_id(&self) -> Option<&::models::ApplicationId> {
        self.application_id.as_ref()
    }

    pub fn reset_application_id(&mut self) {
        self.application_id = None;
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ApplicationProcessExitedEvent {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::FabricEventKind {
        &self.kind
    }

    pub fn set_event_instance_id(&mut self, event_instance_id: String) {
        self.event_instance_id = event_instance_id;
    }

    pub fn with_event_instance_id(
        mut self,
        event_instance_id: String,
    ) -> ApplicationProcessExitedEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn with_category(
        mut self,
        category: String,
    ) -> ApplicationProcessExitedEvent {
        self.category = Some(category);
        self
    }

    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    pub fn reset_category(&mut self) {
        self.category = None;
    }

    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    pub fn with_time_stamp(
        mut self,
        time_stamp: String,
    ) -> ApplicationProcessExitedEvent {
        self.time_stamp = time_stamp;
        self
    }

    pub fn time_stamp(&self) -> &String {
        &self.time_stamp
    }

    pub fn set_has_correlated_events(&mut self, has_correlated_events: bool) {
        self.has_correlated_events = Some(has_correlated_events);
    }

    pub fn with_has_correlated_events(
        mut self,
        has_correlated_events: bool,
    ) -> ApplicationProcessExitedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = service_name;
    }

    pub fn with_service_name(
        mut self,
        service_name: String,
    ) -> ApplicationProcessExitedEvent {
        self.service_name = service_name;
        self
    }

    pub fn service_name(&self) -> &String {
        &self.service_name
    }

    pub fn set_service_package_name(&mut self, service_package_name: String) {
        self.service_package_name = service_package_name;
    }

    pub fn with_service_package_name(
        mut self,
        service_package_name: String,
    ) -> ApplicationProcessExitedEvent {
        self.service_package_name = service_package_name;
        self
    }

    pub fn service_package_name(&self) -> &String {
        &self.service_package_name
    }

    pub fn set_service_package_activation_id(
        &mut self,
        service_package_activation_id: String,
    ) {
        self.service_package_activation_id = service_package_activation_id;
    }

    pub fn with_service_package_activation_id(
        mut self,
        service_package_activation_id: String,
    ) -> ApplicationProcessExitedEvent {
        self.service_package_activation_id = service_package_activation_id;
        self
    }

    pub fn service_package_activation_id(&self) -> &String {
        &self.service_package_activation_id
    }

    pub fn set_is_exclusive(&mut self, is_exclusive: bool) {
        self.is_exclusive = is_exclusive;
    }

    pub fn with_is_exclusive(
        mut self,
        is_exclusive: bool,
    ) -> ApplicationProcessExitedEvent {
        self.is_exclusive = is_exclusive;
        self
    }

    pub fn is_exclusive(&self) -> &bool {
        &self.is_exclusive
    }

    pub fn set_code_package_name(&mut self, code_package_name: String) {
        self.code_package_name = code_package_name;
    }

    pub fn with_code_package_name(
        mut self,
        code_package_name: String,
    ) -> ApplicationProcessExitedEvent {
        self.code_package_name = code_package_name;
        self
    }

    pub fn code_package_name(&self) -> &String {
        &self.code_package_name
    }

    pub fn set_entry_point_type(&mut self, entry_point_type: String) {
        self.entry_point_type = entry_point_type;
    }

    pub fn with_entry_point_type(
        mut self,
        entry_point_type: String,
    ) -> ApplicationProcessExitedEvent {
        self.entry_point_type = entry_point_type;
        self
    }

    pub fn entry_point_type(&self) -> &String {
        &self.entry_point_type
    }

    pub fn set_exe_name(&mut self, exe_name: String) {
        self.exe_name = exe_name;
    }

    pub fn with_exe_name(
        mut self,
        exe_name: String,
    ) -> ApplicationProcessExitedEvent {
        self.exe_name = exe_name;
        self
    }

    pub fn exe_name(&self) -> &String {
        &self.exe_name
    }

    pub fn set_process_id(&mut self, process_id: i64) {
        self.process_id = process_id;
    }

    pub fn with_process_id(
        mut self,
        process_id: i64,
    ) -> ApplicationProcessExitedEvent {
        self.process_id = process_id;
        self
    }

    pub fn process_id(&self) -> &i64 {
        &self.process_id
    }

    pub fn set_host_id(&mut self, host_id: String) {
        self.host_id = host_id;
    }

    pub fn with_host_id(
        mut self,
        host_id: String,
    ) -> ApplicationProcessExitedEvent {
        self.host_id = host_id;
        self
    }

    pub fn host_id(&self) -> &String {
        &self.host_id
    }

    pub fn set_exit_code(&mut self, exit_code: i64) {
        self.exit_code = exit_code;
    }

    pub fn with_exit_code(
        mut self,
        exit_code: i64,
    ) -> ApplicationProcessExitedEvent {
        self.exit_code = exit_code;
        self
    }

    pub fn exit_code(&self) -> &i64 {
        &self.exit_code
    }

    pub fn set_unexpected_termination(&mut self, unexpected_termination: bool) {
        self.unexpected_termination = unexpected_termination;
    }

    pub fn with_unexpected_termination(
        mut self,
        unexpected_termination: bool,
    ) -> ApplicationProcessExitedEvent {
        self.unexpected_termination = unexpected_termination;
        self
    }

    pub fn unexpected_termination(&self) -> &bool {
        &self.unexpected_termination
    }

    pub fn set_start_time(&mut self, start_time: String) {
        self.start_time = start_time;
    }

    pub fn with_start_time(
        mut self,
        start_time: String,
    ) -> ApplicationProcessExitedEvent {
        self.start_time = start_time;
        self
    }

    pub fn start_time(&self) -> &String {
        &self.start_time
    }
}
