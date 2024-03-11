/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterUpgradeRollbackStartedEvent : Cluster Upgrade Rollback Started event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterUpgradeRollbackStartedEvent {
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
    /// Describes failure.
    #[serde(rename = "FailureReason")]
    failure_reason: String,
    /// Overall duration of upgrade in milli-seconds.
    #[serde(rename = "OverallUpgradeElapsedTimeInMs")]
    overall_upgrade_elapsed_time_in_ms: f64,
}

impl ClusterUpgradeRollbackStartedEvent {
    /// Cluster Upgrade Rollback Started event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        target_cluster_version: String,
        failure_reason: String,
        overall_upgrade_elapsed_time_in_ms: f64,
    ) -> ClusterUpgradeRollbackStartedEvent {
        ClusterUpgradeRollbackStartedEvent {
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            target_cluster_version,
            failure_reason,
            overall_upgrade_elapsed_time_in_ms,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ClusterUpgradeRollbackStartedEvent {
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
    ) -> ClusterUpgradeRollbackStartedEvent {
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
    ) -> ClusterUpgradeRollbackStartedEvent {
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
    ) -> ClusterUpgradeRollbackStartedEvent {
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
    ) -> ClusterUpgradeRollbackStartedEvent {
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
    ) -> ClusterUpgradeRollbackStartedEvent {
        self.target_cluster_version = target_cluster_version;
        self
    }

    pub fn target_cluster_version(&self) -> &String {
        &self.target_cluster_version
    }

    pub fn set_failure_reason(&mut self, failure_reason: String) {
        self.failure_reason = failure_reason;
    }

    pub fn with_failure_reason(
        mut self,
        failure_reason: String,
    ) -> ClusterUpgradeRollbackStartedEvent {
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
    ) -> ClusterUpgradeRollbackStartedEvent {
        self.overall_upgrade_elapsed_time_in_ms =
            overall_upgrade_elapsed_time_in_ms;
        self
    }

    pub fn overall_upgrade_elapsed_time_in_ms(&self) -> &f64 {
        &self.overall_upgrade_elapsed_time_in_ms
    }
}
