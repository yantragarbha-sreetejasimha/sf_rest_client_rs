/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerDeactivatedEvent : Container Deactivated event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerDeactivatedEvent {
    /// The kind of FabricEvent.
    #[serde(rename = "Kind")]
    kind: ::models::FabricEventKind,
    /// The identifier for the FabricEvent instance.
    #[serde(rename = "EventInstanceId")]
    event_instance_id: String,
    /// The time event was logged.
    #[serde(rename = "TimeStamp")]
    time_stamp: String,
    /// Shows there is existing related events available.
    #[serde(rename = "HasCorrelatedEvents")]
    has_correlated_events: Option<bool>,
    /// The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\", the application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions.
    #[serde(rename = "ApplicationId")]
    application_id: ::models::ApplicationId,
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
    /// Name of Container image.
    #[serde(rename = "ImageName")]
    image_name: String,
    /// Name of Container.
    #[serde(rename = "ContainerName")]
    container_name: String,
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

impl ContainerDeactivatedEvent {
    /// Container Deactivated event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        application_id: ::models::ApplicationId,
        service_name: String,
        service_package_name: String,
        service_package_activation_id: String,
        is_exclusive: bool,
        code_package_name: String,
        entry_point_type: String,
        image_name: String,
        container_name: String,
        host_id: String,
        exit_code: i64,
        unexpected_termination: bool,
        start_time: String,
    ) -> ContainerDeactivatedEvent {
        ContainerDeactivatedEvent {
            kind,
            event_instance_id,
            time_stamp,
            has_correlated_events: None,
            application_id,
            service_name,
            service_package_name,
            service_package_activation_id,
            is_exclusive,
            code_package_name,
            entry_point_type,
            image_name,
            container_name,
            host_id,
            exit_code,
            unexpected_termination,
            start_time,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    pub fn with_time_stamp(
        mut self,
        time_stamp: String,
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_application_id(
        &mut self,
        application_id: ::models::ApplicationId,
    ) {
        self.application_id = application_id;
    }

    pub fn with_application_id(
        mut self,
        application_id: ::models::ApplicationId,
    ) -> ContainerDeactivatedEvent {
        self.application_id = application_id;
        self
    }

    pub fn application_id(&self) -> &::models::ApplicationId {
        &self.application_id
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = service_name;
    }

    pub fn with_service_name(
        mut self,
        service_name: String,
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
        self.entry_point_type = entry_point_type;
        self
    }

    pub fn entry_point_type(&self) -> &String {
        &self.entry_point_type
    }

    pub fn set_image_name(&mut self, image_name: String) {
        self.image_name = image_name;
    }

    pub fn with_image_name(
        mut self,
        image_name: String,
    ) -> ContainerDeactivatedEvent {
        self.image_name = image_name;
        self
    }

    pub fn image_name(&self) -> &String {
        &self.image_name
    }

    pub fn set_container_name(&mut self, container_name: String) {
        self.container_name = container_name;
    }

    pub fn with_container_name(
        mut self,
        container_name: String,
    ) -> ContainerDeactivatedEvent {
        self.container_name = container_name;
        self
    }

    pub fn container_name(&self) -> &String {
        &self.container_name
    }

    pub fn set_host_id(&mut self, host_id: String) {
        self.host_id = host_id;
    }

    pub fn with_host_id(
        mut self,
        host_id: String,
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
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
    ) -> ContainerDeactivatedEvent {
        self.start_time = start_time;
        self
    }

    pub fn start_time(&self) -> &String {
        &self.start_time
    }
}
