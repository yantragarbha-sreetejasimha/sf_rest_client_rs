/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationHealthReportExpiredEvent : Deployed Application Health Report Expired event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationHealthReportExpiredEvent {
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
    /// Id of Application instance.
    #[serde(rename = "ApplicationInstanceId")]
    application_instance_id: i64,
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: ::models::NodeName,
    /// Id of report source.
    #[serde(rename = "SourceId")]
    source_id: String,
    /// Describes the property.
    #[serde(rename = "Property")]
    property: String,
    /// Describes the property health state.
    #[serde(rename = "HealthState")]
    health_state: String,
    /// Time to live in milli-seconds.
    #[serde(rename = "TimeToLiveMs")]
    time_to_live_ms: i64,
    /// Sequence number of report.
    #[serde(rename = "SequenceNumber")]
    sequence_number: i64,
    /// Description of report.
    #[serde(rename = "Description")]
    description: String,
    /// Indicates the removal when it expires.
    #[serde(rename = "RemoveWhenExpired")]
    remove_when_expired: bool,
    /// Source time.
    #[serde(rename = "SourceUtcTimestamp")]
    source_utc_timestamp: String,
}

impl DeployedApplicationHealthReportExpiredEvent {
    /// Deployed Application Health Report Expired event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        application_instance_id: i64,
        node_name: ::models::NodeName,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: String,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        DeployedApplicationHealthReportExpiredEvent {
            application_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            application_instance_id,
            node_name,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
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
    ) -> DeployedApplicationHealthReportExpiredEvent {
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
    ) -> DeployedApplicationHealthReportExpiredEvent {
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
    ) -> DeployedApplicationHealthReportExpiredEvent {
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
    ) -> DeployedApplicationHealthReportExpiredEvent {
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
    ) -> DeployedApplicationHealthReportExpiredEvent {
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
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_application_instance_id(
        &mut self,
        application_instance_id: i64,
    ) {
        self.application_instance_id = application_instance_id;
    }

    pub fn with_application_instance_id(
        mut self,
        application_instance_id: i64,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.application_instance_id = application_instance_id;
        self
    }

    pub fn application_instance_id(&self) -> &i64 {
        &self.application_instance_id
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = node_name;
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.node_name = node_name;
        self
    }

    pub fn node_name(&self) -> &::models::NodeName {
        &self.node_name
    }

    pub fn set_source_id(&mut self, source_id: String) {
        self.source_id = source_id;
    }

    pub fn with_source_id(
        mut self,
        source_id: String,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.source_id = source_id;
        self
    }

    pub fn source_id(&self) -> &String {
        &self.source_id
    }

    pub fn set_property(&mut self, property: String) {
        self.property = property;
    }

    pub fn with_property(
        mut self,
        property: String,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.property = property;
        self
    }

    pub fn property(&self) -> &String {
        &self.property
    }

    pub fn set_health_state(&mut self, health_state: String) {
        self.health_state = health_state;
    }

    pub fn with_health_state(
        mut self,
        health_state: String,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.health_state = health_state;
        self
    }

    pub fn health_state(&self) -> &String {
        &self.health_state
    }

    pub fn set_time_to_live_ms(&mut self, time_to_live_ms: i64) {
        self.time_to_live_ms = time_to_live_ms;
    }

    pub fn with_time_to_live_ms(
        mut self,
        time_to_live_ms: i64,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.time_to_live_ms = time_to_live_ms;
        self
    }

    pub fn time_to_live_ms(&self) -> &i64 {
        &self.time_to_live_ms
    }

    pub fn set_sequence_number(&mut self, sequence_number: i64) {
        self.sequence_number = sequence_number;
    }

    pub fn with_sequence_number(
        mut self,
        sequence_number: i64,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.sequence_number = sequence_number;
        self
    }

    pub fn sequence_number(&self) -> &i64 {
        &self.sequence_number
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.description = description;
        self
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn set_remove_when_expired(&mut self, remove_when_expired: bool) {
        self.remove_when_expired = remove_when_expired;
    }

    pub fn with_remove_when_expired(
        mut self,
        remove_when_expired: bool,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.remove_when_expired = remove_when_expired;
        self
    }

    pub fn remove_when_expired(&self) -> &bool {
        &self.remove_when_expired
    }

    pub fn set_source_utc_timestamp(&mut self, source_utc_timestamp: String) {
        self.source_utc_timestamp = source_utc_timestamp;
    }

    pub fn with_source_utc_timestamp(
        mut self,
        source_utc_timestamp: String,
    ) -> DeployedApplicationHealthReportExpiredEvent {
        self.source_utc_timestamp = source_utc_timestamp;
        self
    }

    pub fn source_utc_timestamp(&self) -> &String {
        &self.source_utc_timestamp
    }
}
