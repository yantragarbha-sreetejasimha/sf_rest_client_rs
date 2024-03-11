/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StartClusterUpgradeDescription : Describes the parameters for starting a cluster upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StartClusterUpgradeDescription {
    /// The cluster code version.
    #[serde(rename = "CodeVersion")]
    code_version: Option<String>,
    /// The cluster configuration version.
    #[serde(rename = "ConfigVersion")]
    config_version: Option<String>,
    /// The kind of upgrade out of the following possible values.
    #[serde(rename = "UpgradeKind")]
    upgrade_kind: Option<::models::UpgradeKind>,
    /// The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored.
    #[serde(rename = "RollingUpgradeMode")]
    rolling_upgrade_mode: Option<::models::UpgradeMode>,
    /// The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer).
    #[serde(rename = "UpgradeReplicaSetCheckTimeoutInSeconds")]
    upgrade_replica_set_check_timeout_in_seconds:
        Option<::models::UpgradeReplicaSetCheckTimeout>,
    /// If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data).
    #[serde(rename = "ForceRestart")]
    force_restart: Option<::models::ForceRestart>,
    /// Describes the parameters for monitoring an upgrade in Monitored mode.
    #[serde(rename = "MonitoringPolicy")]
    monitoring_policy: Option<::models::MonitoringPolicyDescription>,
    /// Defines a health policy used to evaluate the health of the cluster or of a cluster node.
    #[serde(rename = "ClusterHealthPolicy")]
    cluster_health_policy: Option<::models::ClusterHealthPolicy>,
    /// When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain.
    #[serde(rename = "EnableDeltaHealthEvaluation")]
    enable_delta_health_evaluation: Option<bool>,
    /// Defines a health policy used to evaluate the health of the cluster during a cluster upgrade.
    #[serde(rename = "ClusterUpgradeHealthPolicy")]
    cluster_upgrade_health_policy:
        Option<::models::ClusterUpgradeHealthPolicyObject>,
    /// Defines the application health policy map used to evaluate the health of an application or one of its children entities.
    #[serde(rename = "ApplicationHealthPolicyMap")]
    application_health_policy_map: Option<::models::ApplicationHealthPolicies>,
}

impl Default for StartClusterUpgradeDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl StartClusterUpgradeDescription {
    /// Describes the parameters for starting a cluster upgrade.
    pub fn new() -> StartClusterUpgradeDescription {
        StartClusterUpgradeDescription {
            code_version: None,
            config_version: None,
            upgrade_kind: None,
            rolling_upgrade_mode: None,
            upgrade_replica_set_check_timeout_in_seconds: None,
            force_restart: None,
            monitoring_policy: None,
            cluster_health_policy: None,
            enable_delta_health_evaluation: None,
            cluster_upgrade_health_policy: None,
            application_health_policy_map: None,
        }
    }

    pub fn set_code_version(&mut self, code_version: String) {
        self.code_version = Some(code_version);
    }

    pub fn with_code_version(
        mut self,
        code_version: String,
    ) -> StartClusterUpgradeDescription {
        self.code_version = Some(code_version);
        self
    }

    pub fn code_version(&self) -> Option<&String> {
        self.code_version.as_ref()
    }

    pub fn reset_code_version(&mut self) {
        self.code_version = None;
    }

    pub fn set_config_version(&mut self, config_version: String) {
        self.config_version = Some(config_version);
    }

    pub fn with_config_version(
        mut self,
        config_version: String,
    ) -> StartClusterUpgradeDescription {
        self.config_version = Some(config_version);
        self
    }

    pub fn config_version(&self) -> Option<&String> {
        self.config_version.as_ref()
    }

    pub fn reset_config_version(&mut self) {
        self.config_version = None;
    }

    pub fn set_upgrade_kind(&mut self, upgrade_kind: ::models::UpgradeKind) {
        self.upgrade_kind = Some(upgrade_kind);
    }

    pub fn with_upgrade_kind(
        mut self,
        upgrade_kind: ::models::UpgradeKind,
    ) -> StartClusterUpgradeDescription {
        self.upgrade_kind = Some(upgrade_kind);
        self
    }

    pub fn upgrade_kind(&self) -> Option<&::models::UpgradeKind> {
        self.upgrade_kind.as_ref()
    }

    pub fn reset_upgrade_kind(&mut self) {
        self.upgrade_kind = None;
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
    ) -> StartClusterUpgradeDescription {
        self.rolling_upgrade_mode = Some(rolling_upgrade_mode);
        self
    }

    pub fn rolling_upgrade_mode(&self) -> Option<&::models::UpgradeMode> {
        self.rolling_upgrade_mode.as_ref()
    }

    pub fn reset_rolling_upgrade_mode(&mut self) {
        self.rolling_upgrade_mode = None;
    }

    pub fn set_upgrade_replica_set_check_timeout_in_seconds(
        &mut self,
        upgrade_replica_set_check_timeout_in_seconds: ::models::UpgradeReplicaSetCheckTimeout,
    ) {
        self.upgrade_replica_set_check_timeout_in_seconds =
            Some(upgrade_replica_set_check_timeout_in_seconds);
    }

    pub fn with_upgrade_replica_set_check_timeout_in_seconds(
        mut self,
        upgrade_replica_set_check_timeout_in_seconds: ::models::UpgradeReplicaSetCheckTimeout,
    ) -> StartClusterUpgradeDescription {
        self.upgrade_replica_set_check_timeout_in_seconds =
            Some(upgrade_replica_set_check_timeout_in_seconds);
        self
    }

    pub fn upgrade_replica_set_check_timeout_in_seconds(
        &self,
    ) -> Option<&::models::UpgradeReplicaSetCheckTimeout> {
        self.upgrade_replica_set_check_timeout_in_seconds.as_ref()
    }

    pub fn reset_upgrade_replica_set_check_timeout_in_seconds(&mut self) {
        self.upgrade_replica_set_check_timeout_in_seconds = None;
    }

    pub fn set_force_restart(&mut self, force_restart: ::models::ForceRestart) {
        self.force_restart = Some(force_restart);
    }

    pub fn with_force_restart(
        mut self,
        force_restart: ::models::ForceRestart,
    ) -> StartClusterUpgradeDescription {
        self.force_restart = Some(force_restart);
        self
    }

    pub fn force_restart(&self) -> Option<&::models::ForceRestart> {
        self.force_restart.as_ref()
    }

    pub fn reset_force_restart(&mut self) {
        self.force_restart = None;
    }

    pub fn set_monitoring_policy(
        &mut self,
        monitoring_policy: ::models::MonitoringPolicyDescription,
    ) {
        self.monitoring_policy = Some(monitoring_policy);
    }

    pub fn with_monitoring_policy(
        mut self,
        monitoring_policy: ::models::MonitoringPolicyDescription,
    ) -> StartClusterUpgradeDescription {
        self.monitoring_policy = Some(monitoring_policy);
        self
    }

    pub fn monitoring_policy(
        &self,
    ) -> Option<&::models::MonitoringPolicyDescription> {
        self.monitoring_policy.as_ref()
    }

    pub fn reset_monitoring_policy(&mut self) {
        self.monitoring_policy = None;
    }

    pub fn set_cluster_health_policy(
        &mut self,
        cluster_health_policy: ::models::ClusterHealthPolicy,
    ) {
        self.cluster_health_policy = Some(cluster_health_policy);
    }

    pub fn with_cluster_health_policy(
        mut self,
        cluster_health_policy: ::models::ClusterHealthPolicy,
    ) -> StartClusterUpgradeDescription {
        self.cluster_health_policy = Some(cluster_health_policy);
        self
    }

    pub fn cluster_health_policy(
        &self,
    ) -> Option<&::models::ClusterHealthPolicy> {
        self.cluster_health_policy.as_ref()
    }

    pub fn reset_cluster_health_policy(&mut self) {
        self.cluster_health_policy = None;
    }

    pub fn set_enable_delta_health_evaluation(
        &mut self,
        enable_delta_health_evaluation: bool,
    ) {
        self.enable_delta_health_evaluation =
            Some(enable_delta_health_evaluation);
    }

    pub fn with_enable_delta_health_evaluation(
        mut self,
        enable_delta_health_evaluation: bool,
    ) -> StartClusterUpgradeDescription {
        self.enable_delta_health_evaluation =
            Some(enable_delta_health_evaluation);
        self
    }

    pub fn enable_delta_health_evaluation(&self) -> Option<&bool> {
        self.enable_delta_health_evaluation.as_ref()
    }

    pub fn reset_enable_delta_health_evaluation(&mut self) {
        self.enable_delta_health_evaluation = None;
    }

    pub fn set_cluster_upgrade_health_policy(
        &mut self,
        cluster_upgrade_health_policy: ::models::ClusterUpgradeHealthPolicyObject,
    ) {
        self.cluster_upgrade_health_policy =
            Some(cluster_upgrade_health_policy);
    }

    pub fn with_cluster_upgrade_health_policy(
        mut self,
        cluster_upgrade_health_policy: ::models::ClusterUpgradeHealthPolicyObject,
    ) -> StartClusterUpgradeDescription {
        self.cluster_upgrade_health_policy =
            Some(cluster_upgrade_health_policy);
        self
    }

    pub fn cluster_upgrade_health_policy(
        &self,
    ) -> Option<&::models::ClusterUpgradeHealthPolicyObject> {
        self.cluster_upgrade_health_policy.as_ref()
    }

    pub fn reset_cluster_upgrade_health_policy(&mut self) {
        self.cluster_upgrade_health_policy = None;
    }

    pub fn set_application_health_policy_map(
        &mut self,
        application_health_policy_map: ::models::ApplicationHealthPolicies,
    ) {
        self.application_health_policy_map =
            Some(application_health_policy_map);
    }

    pub fn with_application_health_policy_map(
        mut self,
        application_health_policy_map: ::models::ApplicationHealthPolicies,
    ) -> StartClusterUpgradeDescription {
        self.application_health_policy_map =
            Some(application_health_policy_map);
        self
    }

    pub fn application_health_policy_map(
        &self,
    ) -> Option<&::models::ApplicationHealthPolicies> {
        self.application_health_policy_map.as_ref()
    }

    pub fn reset_application_health_policy_map(&mut self) {
        self.application_health_policy_map = None;
    }
}
