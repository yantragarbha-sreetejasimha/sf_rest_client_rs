/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterUpgradeCompletedEvent : Cluster Upgrade Completed event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterUpgradeCompletedEvent {
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
    /// Target Cluster version.
    #[serde(rename = "TargetClusterVersion")]
    target_cluster_version: String,
    /// Overall duration of upgrade in milli-seconds.
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    overall_upgrade_elapsed_time_in_ms: f64,
}

impl ClusterUpgradeCompletedEvent {
    /// Cluster Upgrade Completed event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        target_cluster_version: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> ClusterUpgradeCompletedEvent {
        ClusterUpgradeCompletedEvent {
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            target_cluster_version,
            overall_upgrade_elapsed_time_in_ms,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ClusterUpgradeCompletedEvent {
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
    ) -> ClusterUpgradeCompletedEvent {
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
    ) -> ClusterUpgradeCompletedEvent {
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
    ) -> ClusterUpgradeCompletedEvent {
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
    ) -> ClusterUpgradeCompletedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_target_cluster_version(
        &mut self,
        target_cluster_version: String,
    ) {
        self.target_cluster_version = target_cluster_version;
    }

    pub fn with_target_cluster_version(
        mut self,
        target_cluster_version: String,
    ) -> ClusterUpgradeCompletedEvent {
        self.target_cluster_version = target_cluster_version;
        self
    }

    pub fn target_cluster_version(&self) -> &String {
        &self.target_cluster_version
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
    ) -> ClusterUpgradeCompletedEvent {
        self.overall_upgrade_elapsed_time_in_ms =
            overall_upgrade_elapsed_time_in_ms;
        self
    }

    pub fn overall_upgrade_elapsed_time_in_ms(&self) -> &f64 {
        &self.overall_upgrade_elapsed_time_in_ms
    }
}
