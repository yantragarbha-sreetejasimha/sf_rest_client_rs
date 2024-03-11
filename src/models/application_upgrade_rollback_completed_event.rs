/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationUpgradeRollbackCompletedEvent : Application Upgrade Rollback Completed event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationUpgradeRollbackCompletedEvent {
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
    /// Application type name.
    #[serde(rename = "ApplicationTypeName")]
    application_type_name: String,
    /// Application type version.
    #[serde(rename = "ApplicationTypeVersion")]
    application_type_version: String,
    /// Describes reason of failure.
    #[serde(rename = "FailureReason")]
    failure_reason: String,
    /// Overall upgrade time in milli-seconds.
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    overall_upgrade_elapsed_time_in_ms: f64,
}

impl ApplicationUpgradeRollbackCompletedEvent {
    /// Application Upgrade Rollback Completed event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        application_type_name: String,
        application_type_version: String,
        failure_reason: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> ApplicationUpgradeRollbackCompletedEvent {
        ApplicationUpgradeRollbackCompletedEvent {
            application_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            application_type_name,
            application_type_version,
            failure_reason,
            overall_upgrade_elapsed_time_in_ms,
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
    ) -> ApplicationUpgradeRollbackCompletedEvent {
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
    ) -> ApplicationUpgradeRollbackCompletedEvent {
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
    ) -> ApplicationUpgradeRollbackCompletedEvent {
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
    ) -> ApplicationUpgradeRollbackCompletedEvent {
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
    ) -> ApplicationUpgradeRollbackCompletedEvent {
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
    ) -> ApplicationUpgradeRollbackCompletedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_application_type_name(&mut self, application_type_name: String) {
        self.application_type_name = application_type_name;
    }

    pub fn with_application_type_name(
        mut self,
        application_type_name: String,
    ) -> ApplicationUpgradeRollbackCompletedEvent {
        self.application_type_name = application_type_name;
        self
    }

    pub fn application_type_name(&self) -> &String {
        &self.application_type_name
    }

    pub fn set_application_type_version(
        &mut self,
        application_type_version: String,
    ) {
        self.application_type_version = application_type_version;
    }

    pub fn with_application_type_version(
        mut self,
        application_type_version: String,
    ) -> ApplicationUpgradeRollbackCompletedEvent {
        self.application_type_version = application_type_version;
        self
    }

    pub fn application_type_version(&self) -> &String {
        &self.application_type_version
    }

    pub fn set_failure_reason(&mut self, failure_reason: String) {
        self.failure_reason = failure_reason;
    }

    pub fn with_failure_reason(
        mut self,
        failure_reason: String,
    ) -> ApplicationUpgradeRollbackCompletedEvent {
        self.failure_reason = failure_reason;
        self
    }

    pub fn failure_reason(&self) -> &String {
        &self.failure_reason
    }

    pub fn set_overall_upgrade_elapsed_time_in_ms(
        &mut self,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) {
        self.overall_upgrade_elapsed_time_in_ms =
            overall_upgrade_elapsed_time_in_ms;
    }

    pub fn with_overall_upgrade_elapsed_time_in_ms(
        mut self,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> ApplicationUpgradeRollbackCompletedEvent {
        self.overall_upgrade_elapsed_time_in_ms =
            overall_upgrade_elapsed_time_in_ms;
        self
    }

    pub fn overall_upgrade_elapsed_time_in_ms(&self) -> &f64 {
        &self.overall_upgrade_elapsed_time_in_ms
    }
}
