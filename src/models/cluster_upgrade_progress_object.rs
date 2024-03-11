/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterUpgradeProgressObject : Information about a cluster upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterUpgradeProgressObject {
    /// The ServiceFabric code version of the cluster.
    #[serde(rename = "CodeVersion")]
    code_version: Option<::models::ClusterFabricCodeVersionString>,
    /// The cluster configuration version (specified in the cluster manifest).
    #[serde(rename = "ConfigVersion")]
    config_version: Option<::models::ClusterFabricConfigVersionString>,
    /// List of upgrade domains and their statuses. Not applicable to node-by-node upgrades.
    #[serde(rename = "UpgradeDomains")]
    upgrade_domains: Option<::models::UpgradeDomainInfoList>,
    /// List of upgrade units and their statuses.
    #[serde(rename = "UpgradeUnits")]
    upgrade_units: Option<::models::UpgradeUnitInfoList>,
    /// The state of the upgrade domain.
    #[serde(rename = "UpgradeState")]
    upgrade_state: Option<::models::UpgradeState>,
    /// The name of the next upgrade domain to be processed. Not applicable to node-by-node upgrades.
    #[serde(rename = "NextUpgradeDomain")]
    next_upgrade_domain: Option<::models::NextUpgradeDomain>,
    /// The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, Monitored, and UnmonitoredDeferred.
    #[serde(rename = "RollingUpgradeMode")]
    rolling_upgrade_mode: Option<::models::UpgradeMode>,
    /// Represents a ServiceFabric cluster upgrade
    #[serde(rename = "UpgradeDescription")]
    upgrade_description: Option<::models::ClusterUpgradeDescriptionObject>,
    /// The estimated elapsed time spent processing the current overall upgrade.
    #[serde(rename = "UpgradeDurationInMilliseconds")]
    upgrade_duration_in_milliseconds: Option<::models::UpgradeDurationString>,
    /// The estimated elapsed time spent processing the current upgrade domain. Not applicable to node-by-node upgrades.
    #[serde(rename = "UpgradeDomainDurationInMilliseconds")]
    upgrade_domain_duration_in_milliseconds:
        Option<::models::UpgradeDomainDurationString>,
    /// List of health evaluations that resulted in the current aggregated health state.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
    /// Information about the current in-progress upgrade domain. Not applicable to node-by-node upgrades.
    #[serde(rename = "CurrentUpgradeDomainProgress")]
    current_upgrade_domain_progress:
        Option<::models::CurrentUpgradeDomainProgressInfo>,
    /// Information about the current in-progress upgrade units.
    #[serde(rename = "CurrentUpgradeUnitsProgress")]
    current_upgrade_units_progress:
        Option<::models::CurrentUpgradeUnitsProgressInfo>,
    /// The start time of the upgrade in UTC.
    #[serde(rename = "StartTimestampUtc")]
    start_timestamp_utc: Option<::models::UpgradeStartTimeUtcString>,
    /// The failure time of the upgrade in UTC.
    #[serde(rename = "FailureTimestampUtc")]
    failure_timestamp_utc: Option<::models::UpgradeFailureTimeUtcString>,
    /// The cause of an upgrade failure that resulted in FailureAction being executed.
    #[serde(rename = "FailureReason")]
    failure_reason: Option<::models::FailureReason>,
    /// The detailed upgrade progress for nodes in the current upgrade domain at the point of failure. Not applicable to node-by-node upgrades.
    #[serde(rename = "UpgradeDomainProgressAtFailure")]
    upgrade_domain_progress_at_failure:
        Option<::models::FailedUpgradeDomainProgressObject>,
    /// Indicates whether this upgrade is node-by-node.
    #[serde(rename = "IsNodeByNode")]
    is_node_by_node: Option<bool>,
}

impl Default for ClusterUpgradeProgressObject {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterUpgradeProgressObject {
    /// Information about a cluster upgrade.
    pub fn new() -> ClusterUpgradeProgressObject {
        ClusterUpgradeProgressObject {
            code_version: None,
            config_version: None,
            upgrade_domains: None,
            upgrade_units: None,
            upgrade_state: None,
            next_upgrade_domain: None,
            rolling_upgrade_mode: None,
            upgrade_description: None,
            upgrade_duration_in_milliseconds: None,
            upgrade_domain_duration_in_milliseconds: None,
            unhealthy_evaluations: None,
            current_upgrade_domain_progress: None,
            current_upgrade_units_progress: None,
            start_timestamp_utc: None,
            failure_timestamp_utc: None,
            failure_reason: None,
            upgrade_domain_progress_at_failure: None,
            is_node_by_node: None,
        }
    }

    pub fn set_code_version(
        &mut self,
        code_version: ::models::ClusterFabricCodeVersionString,
    ) {
        self.code_version = Some(code_version);
    }

    pub fn with_code_version(
        mut self,
        code_version: ::models::ClusterFabricCodeVersionString,
    ) -> ClusterUpgradeProgressObject {
        self.code_version = Some(code_version);
        self
    }

    pub fn code_version(
        &self,
    ) -> Option<&::models::ClusterFabricCodeVersionString> {
        self.code_version.as_ref()
    }

    pub fn reset_code_version(&mut self) {
        self.code_version = None;
    }

    pub fn set_config_version(
        &mut self,
        config_version: ::models::ClusterFabricConfigVersionString,
    ) {
        self.config_version = Some(config_version);
    }

    pub fn with_config_version(
        mut self,
        config_version: ::models::ClusterFabricConfigVersionString,
    ) -> ClusterUpgradeProgressObject {
        self.config_version = Some(config_version);
        self
    }

    pub fn config_version(
        &self,
    ) -> Option<&::models::ClusterFabricConfigVersionString> {
        self.config_version.as_ref()
    }

    pub fn reset_config_version(&mut self) {
        self.config_version = None;
    }

    pub fn set_upgrade_domains(
        &mut self,
        upgrade_domains: ::models::UpgradeDomainInfoList,
    ) {
        self.upgrade_domains = Some(upgrade_domains);
    }

    pub fn with_upgrade_domains(
        mut self,
        upgrade_domains: ::models::UpgradeDomainInfoList,
    ) -> ClusterUpgradeProgressObject {
        self.upgrade_domains = Some(upgrade_domains);
        self
    }

    pub fn upgrade_domains(&self) -> Option<&::models::UpgradeDomainInfoList> {
        self.upgrade_domains.as_ref()
    }

    pub fn reset_upgrade_domains(&mut self) {
        self.upgrade_domains = None;
    }

    pub fn set_upgrade_units(
        &mut self,
        upgrade_units: ::models::UpgradeUnitInfoList,
    ) {
        self.upgrade_units = Some(upgrade_units);
    }

    pub fn with_upgrade_units(
        mut self,
        upgrade_units: ::models::UpgradeUnitInfoList,
    ) -> ClusterUpgradeProgressObject {
        self.upgrade_units = Some(upgrade_units);
        self
    }

    pub fn upgrade_units(&self) -> Option<&::models::UpgradeUnitInfoList> {
        self.upgrade_units.as_ref()
    }

    pub fn reset_upgrade_units(&mut self) {
        self.upgrade_units = None;
    }

    pub fn set_upgrade_state(&mut self, upgrade_state: ::models::UpgradeState) {
        self.upgrade_state = Some(upgrade_state);
    }

    pub fn with_upgrade_state(
        mut self,
        upgrade_state: ::models::UpgradeState,
    ) -> ClusterUpgradeProgressObject {
        self.upgrade_state = Some(upgrade_state);
        self
    }

    pub fn upgrade_state(&self) -> Option<&::models::UpgradeState> {
        self.upgrade_state.as_ref()
    }

    pub fn reset_upgrade_state(&mut self) {
        self.upgrade_state = None;
    }

    pub fn set_next_upgrade_domain(
        &mut self,
        next_upgrade_domain: ::models::NextUpgradeDomain,
    ) {
        self.next_upgrade_domain = Some(next_upgrade_domain);
    }

    pub fn with_next_upgrade_domain(
        mut self,
        next_upgrade_domain: ::models::NextUpgradeDomain,
    ) -> ClusterUpgradeProgressObject {
        self.next_upgrade_domain = Some(next_upgrade_domain);
        self
    }

    pub fn next_upgrade_domain(&self) -> Option<&::models::NextUpgradeDomain> {
        self.next_upgrade_domain.as_ref()
    }

    pub fn reset_next_upgrade_domain(&mut self) {
        self.next_upgrade_domain = None;
    }

    pub fn set_rolling_upgrade_mode(
        &mut self,
        rolling_upgrade_mode: ::models::UpgradeMode,
    ) {
        self.rolling_upgrade_mode = Some(rolling_upgrade_mode);
    }

    pub fn with_rolling_upgrade_mode(
        mut self,
        rolling_upgrade_mode: ::models::UpgradeMode,
    ) -> ClusterUpgradeProgressObject {
        self.rolling_upgrade_mode = Some(rolling_upgrade_mode);
        self
    }

    pub fn rolling_upgrade_mode(&self) -> Option<&::models::UpgradeMode> {
        self.rolling_upgrade_mode.as_ref()
    }

    pub fn reset_rolling_upgrade_mode(&mut self) {
        self.rolling_upgrade_mode = None;
    }

    pub fn set_upgrade_description(
        &mut self,
        upgrade_description: ::models::ClusterUpgradeDescriptionObject,
    ) {
        self.upgrade_description = Some(upgrade_description);
    }

    pub fn with_upgrade_description(
        mut self,
        upgrade_description: ::models::ClusterUpgradeDescriptionObject,
    ) -> ClusterUpgradeProgressObject {
        self.upgrade_description = Some(upgrade_description);
        self
    }

    pub fn upgrade_description(
        &self,
    ) -> Option<&::models::ClusterUpgradeDescriptionObject> {
        self.upgrade_description.as_ref()
    }

    pub fn reset_upgrade_description(&mut self) {
        self.upgrade_description = None;
    }

    pub fn set_upgrade_duration_in_milliseconds(
        &mut self,
        upgrade_duration_in_milliseconds: ::models::UpgradeDurationString,
    ) {
        self.upgrade_duration_in_milliseconds =
            Some(upgrade_duration_in_milliseconds);
    }

    pub fn with_upgrade_duration_in_milliseconds(
        mut self,
        upgrade_duration_in_milliseconds: ::models::UpgradeDurationString,
    ) -> ClusterUpgradeProgressObject {
        self.upgrade_duration_in_milliseconds =
            Some(upgrade_duration_in_milliseconds);
        self
    }

    pub fn upgrade_duration_in_milliseconds(
        &self,
    ) -> Option<&::models::UpgradeDurationString> {
        self.upgrade_duration_in_milliseconds.as_ref()
    }

    pub fn reset_upgrade_duration_in_milliseconds(&mut self) {
        self.upgrade_duration_in_milliseconds = None;
    }

    pub fn set_upgrade_domain_duration_in_milliseconds(
        &mut self,
        upgrade_domain_duration_in_milliseconds: ::models::UpgradeDomainDurationString,
    ) {
        self.upgrade_domain_duration_in_milliseconds =
            Some(upgrade_domain_duration_in_milliseconds);
    }

    pub fn with_upgrade_domain_duration_in_milliseconds(
        mut self,
        upgrade_domain_duration_in_milliseconds: ::models::UpgradeDomainDurationString,
    ) -> ClusterUpgradeProgressObject {
        self.upgrade_domain_duration_in_milliseconds =
            Some(upgrade_domain_duration_in_milliseconds);
        self
    }

    pub fn upgrade_domain_duration_in_milliseconds(
        &self,
    ) -> Option<&::models::UpgradeDomainDurationString> {
        self.upgrade_domain_duration_in_milliseconds.as_ref()
    }

    pub fn reset_upgrade_domain_duration_in_milliseconds(&mut self) {
        self.upgrade_domain_duration_in_milliseconds = None;
    }

    pub fn set_unhealthy_evaluations(
        &mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
    }

    pub fn with_unhealthy_evaluations(
        mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) -> ClusterUpgradeProgressObject {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
        self
    }

    pub fn unhealthy_evaluations(
        &self,
    ) -> Option<&::models::UnhealthyEvaluations> {
        self.unhealthy_evaluations.as_ref()
    }

    pub fn reset_unhealthy_evaluations(&mut self) {
        self.unhealthy_evaluations = None;
    }

    pub fn set_current_upgrade_domain_progress(
        &mut self,
        current_upgrade_domain_progress: ::models::CurrentUpgradeDomainProgressInfo,
    ) {
        self.current_upgrade_domain_progress =
            Some(current_upgrade_domain_progress);
    }

    pub fn with_current_upgrade_domain_progress(
        mut self,
        current_upgrade_domain_progress: ::models::CurrentUpgradeDomainProgressInfo,
    ) -> ClusterUpgradeProgressObject {
        self.current_upgrade_domain_progress =
            Some(current_upgrade_domain_progress);
        self
    }

    pub fn current_upgrade_domain_progress(
        &self,
    ) -> Option<&::models::CurrentUpgradeDomainProgressInfo> {
        self.current_upgrade_domain_progress.as_ref()
    }

    pub fn reset_current_upgrade_domain_progress(&mut self) {
        self.current_upgrade_domain_progress = None;
    }

    pub fn set_current_upgrade_units_progress(
        &mut self,
        current_upgrade_units_progress: ::models::CurrentUpgradeUnitsProgressInfo,
    ) {
        self.current_upgrade_units_progress =
            Some(current_upgrade_units_progress);
    }

    pub fn with_current_upgrade_units_progress(
        mut self,
        current_upgrade_units_progress: ::models::CurrentUpgradeUnitsProgressInfo,
    ) -> ClusterUpgradeProgressObject {
        self.current_upgrade_units_progress =
            Some(current_upgrade_units_progress);
        self
    }

    pub fn current_upgrade_units_progress(
        &self,
    ) -> Option<&::models::CurrentUpgradeUnitsProgressInfo> {
        self.current_upgrade_units_progress.as_ref()
    }

    pub fn reset_current_upgrade_units_progress(&mut self) {
        self.current_upgrade_units_progress = None;
    }

    pub fn set_start_timestamp_utc(
        &mut self,
        start_timestamp_utc: ::models::UpgradeStartTimeUtcString,
    ) {
        self.start_timestamp_utc = Some(start_timestamp_utc);
    }

    pub fn with_start_timestamp_utc(
        mut self,
        start_timestamp_utc: ::models::UpgradeStartTimeUtcString,
    ) -> ClusterUpgradeProgressObject {
        self.start_timestamp_utc = Some(start_timestamp_utc);
        self
    }

    pub fn start_timestamp_utc(
        &self,
    ) -> Option<&::models::UpgradeStartTimeUtcString> {
        self.start_timestamp_utc.as_ref()
    }

    pub fn reset_start_timestamp_utc(&mut self) {
        self.start_timestamp_utc = None;
    }

    pub fn set_failure_timestamp_utc(
        &mut self,
        failure_timestamp_utc: ::models::UpgradeFailureTimeUtcString,
    ) {
        self.failure_timestamp_utc = Some(failure_timestamp_utc);
    }

    pub fn with_failure_timestamp_utc(
        mut self,
        failure_timestamp_utc: ::models::UpgradeFailureTimeUtcString,
    ) -> ClusterUpgradeProgressObject {
        self.failure_timestamp_utc = Some(failure_timestamp_utc);
        self
    }

    pub fn failure_timestamp_utc(
        &self,
    ) -> Option<&::models::UpgradeFailureTimeUtcString> {
        self.failure_timestamp_utc.as_ref()
    }

    pub fn reset_failure_timestamp_utc(&mut self) {
        self.failure_timestamp_utc = None;
    }

    pub fn set_failure_reason(
        &mut self,
        failure_reason: ::models::FailureReason,
    ) {
        self.failure_reason = Some(failure_reason);
    }

    pub fn with_failure_reason(
        mut self,
        failure_reason: ::models::FailureReason,
    ) -> ClusterUpgradeProgressObject {
        self.failure_reason = Some(failure_reason);
        self
    }

    pub fn failure_reason(&self) -> Option<&::models::FailureReason> {
        self.failure_reason.as_ref()
    }

    pub fn reset_failure_reason(&mut self) {
        self.failure_reason = None;
    }

    pub fn set_upgrade_domain_progress_at_failure(
        &mut self,
        upgrade_domain_progress_at_failure: ::models::FailedUpgradeDomainProgressObject,
    ) {
        self.upgrade_domain_progress_at_failure =
            Some(upgrade_domain_progress_at_failure);
    }

    pub fn with_upgrade_domain_progress_at_failure(
        mut self,
        upgrade_domain_progress_at_failure: ::models::FailedUpgradeDomainProgressObject,
    ) -> ClusterUpgradeProgressObject {
        self.upgrade_domain_progress_at_failure =
            Some(upgrade_domain_progress_at_failure);
        self
    }

    pub fn upgrade_domain_progress_at_failure(
        &self,
    ) -> Option<&::models::FailedUpgradeDomainProgressObject> {
        self.upgrade_domain_progress_at_failure.as_ref()
    }

    pub fn reset_upgrade_domain_progress_at_failure(&mut self) {
        self.upgrade_domain_progress_at_failure = None;
    }

    pub fn set_is_node_by_node(&mut self, is_node_by_node: bool) {
        self.is_node_by_node = Some(is_node_by_node);
    }

    pub fn with_is_node_by_node(
        mut self,
        is_node_by_node: bool,
    ) -> ClusterUpgradeProgressObject {
        self.is_node_by_node = Some(is_node_by_node);
        self
    }

    pub fn is_node_by_node(&self) -> Option<&bool> {
        self.is_node_by_node.as_ref()
    }

    pub fn reset_is_node_by_node(&mut self) {
        self.is_node_by_node = None;
    }
}
