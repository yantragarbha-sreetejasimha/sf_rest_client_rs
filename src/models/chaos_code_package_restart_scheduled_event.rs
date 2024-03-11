/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosCodePackageRestartScheduledEvent : Chaos Restart Code Package Fault Scheduled event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosCodePackageRestartScheduledEvent {
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
    /// Id of fault group.
    #[serde(rename = "FaultGroupId")]
    fault_group_id: String,
    /// Id of fault.
    #[serde(rename = "FaultId")]
    fault_id: String,
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: ::models::NodeName,
    /// Service manifest name.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: String,
    /// Code package name.
    #[serde(rename = "CodePackageName")]
    code_package_name: String,
    /// Id of Service package activation.
    #[serde(rename = "ServicePackageActivationId")]
    service_package_activation_id: String,
}

impl ChaosCodePackageRestartScheduledEvent {
    /// Chaos Restart Code Package Fault Scheduled event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        fault_group_id: String,
        fault_id: String,
        node_name: ::models::NodeName,
        service_manifest_name: String,
        code_package_name: String,
        service_package_activation_id: String,
    ) -> ChaosCodePackageRestartScheduledEvent {
        ChaosCodePackageRestartScheduledEvent {
            application_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            fault_group_id,
            fault_id,
            node_name,
            service_manifest_name,
            code_package_name,
            service_package_activation_id,
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
    ) -> ChaosCodePackageRestartScheduledEvent {
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
    ) -> ChaosCodePackageRestartScheduledEvent {
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
    ) -> ChaosCodePackageRestartScheduledEvent {
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
    ) -> ChaosCodePackageRestartScheduledEvent {
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
    ) -> ChaosCodePackageRestartScheduledEvent {
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
    ) -> ChaosCodePackageRestartScheduledEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_fault_group_id(&mut self, fault_group_id: String) {
        self.fault_group_id = fault_group_id;
    }

    pub fn with_fault_group_id(
        mut self,
        fault_group_id: String,
    ) -> ChaosCodePackageRestartScheduledEvent {
        self.fault_group_id = fault_group_id;
        self
    }

    pub fn fault_group_id(&self) -> &String {
        &self.fault_group_id
    }

    pub fn set_fault_id(&mut self, fault_id: String) {
        self.fault_id = fault_id;
    }

    pub fn with_fault_id(
        mut self,
        fault_id: String,
    ) -> ChaosCodePackageRestartScheduledEvent {
        self.fault_id = fault_id;
        self
    }

    pub fn fault_id(&self) -> &String {
        &self.fault_id
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = node_name;
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> ChaosCodePackageRestartScheduledEvent {
        self.node_name = node_name;
        self
    }

    pub fn node_name(&self) -> &::models::NodeName {
        &self.node_name
    }

    pub fn set_service_manifest_name(&mut self, service_manifest_name: String) {
        self.service_manifest_name = service_manifest_name;
    }

    pub fn with_service_manifest_name(
        mut self,
        service_manifest_name: String,
    ) -> ChaosCodePackageRestartScheduledEvent {
        self.service_manifest_name = service_manifest_name;
        self
    }

    pub fn service_manifest_name(&self) -> &String {
        &self.service_manifest_name
    }

    pub fn set_code_package_name(&mut self, code_package_name: String) {
        self.code_package_name = code_package_name;
    }

    pub fn with_code_package_name(
        mut self,
        code_package_name: String,
    ) -> ChaosCodePackageRestartScheduledEvent {
        self.code_package_name = code_package_name;
        self
    }

    pub fn code_package_name(&self) -> &String {
        &self.code_package_name
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
    ) -> ChaosCodePackageRestartScheduledEvent {
        self.service_package_activation_id = service_package_activation_id;
        self
    }

    pub fn service_package_activation_id(&self) -> &String {
        &self.service_package_activation_id
    }
}
