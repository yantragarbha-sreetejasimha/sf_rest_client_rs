/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterUpgradeStartedEvent : Cluster Upgrade Started event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterUpgradeStartedEvent {
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
    /// Current Cluster version.
    #[serde(rename = "CurrentClusterVersion")]
    current_cluster_version: String,
    /// Target Cluster version.
    #[serde(rename = "TargetClusterVersion")]
    target_cluster_version: String,
    /// Type of upgrade.
    #[serde(rename = "UpgradeType")]
    upgrade_type: String,
    /// Mode of upgrade.
    #[serde(rename = "RollingUpgradeMode")]
    rolling_upgrade_mode: String,
    /// Action if failed.
    #[serde(rename = "FailureAction")]
    failure_action: String,
}

impl ClusterUpgradeStartedEvent {
    /// Cluster Upgrade Started event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        current_cluster_version: String,
        target_cluster_version: String,
        upgrade_type: String,
        rolling_upgrade_mode: String,
        failure_action: String,
    ) -> ClusterUpgradeStartedEvent {
        ClusterUpgradeStartedEvent {
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            current_cluster_version,
            target_cluster_version,
            upgrade_type,
            rolling_upgrade_mode,
            failure_action,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ClusterUpgradeStartedEvent {
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
    ) -> ClusterUpgradeStartedEvent {
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
    ) -> ClusterUpgradeStartedEvent {
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
    ) -> ClusterUpgradeStartedEvent {
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
    ) -> ClusterUpgradeStartedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_current_cluster_version(
        &mut self,
        current_cluster_version: String,
    ) {
        self.current_cluster_version = current_cluster_version;
    }

    pub fn with_current_cluster_version(
        mut self,
        current_cluster_version: String,
    ) -> ClusterUpgradeStartedEvent {
        self.current_cluster_version = current_cluster_version;
        self
    }

    pub fn current_cluster_version(&self) -> &String {
        &self.current_cluster_version
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
    ) -> ClusterUpgradeStartedEvent {
        self.target_cluster_version = target_cluster_version;
        self
    }

    pub fn target_cluster_version(&self) -> &String {
        &self.target_cluster_version
    }

    pub fn set_upgrade_type(&mut self, upgrade_type: String) {
        self.upgrade_type = upgrade_type;
    }

    pub fn with_upgrade_type(
        mut self,
        upgrade_type: String,
    ) -> ClusterUpgradeStartedEvent {
        self.upgrade_type = upgrade_type;
        self
    }

    pub fn upgrade_type(&self) -> &String {
        &self.upgrade_type
    }

    pub fn set_rolling_upgrade_mode(&mut self, rolling_upgrade_mode: String) {
        self.rolling_upgrade_mode = rolling_upgrade_mode;
    }

    pub fn with_rolling_upgrade_mode(
        mut self,
        rolling_upgrade_mode: String,
    ) -> ClusterUpgradeStartedEvent {
        self.rolling_upgrade_mode = rolling_upgrade_mode;
        self
    }

    pub fn rolling_upgrade_mode(&self) -> &String {
        &self.rolling_upgrade_mode
    }

    pub fn set_failure_action(&mut self, failure_action: String) {
        self.failure_action = failure_action;
    }

    pub fn with_failure_action(
        mut self,
        failure_action: String,
    ) -> ClusterUpgradeStartedEvent {
        self.failure_action = failure_action;
        self
    }

    pub fn failure_action(&self) -> &String {
        &self.failure_action
    }
}