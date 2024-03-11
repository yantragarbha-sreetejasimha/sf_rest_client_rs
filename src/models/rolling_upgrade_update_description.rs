/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RollingUpgradeUpdateDescription : Describes the parameters for updating a rolling upgrade of application or cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RollingUpgradeUpdateDescription {
    /// The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored.
    #[serde(rename = "RollingUpgradeMode")]
    rolling_upgrade_mode: ::models::UpgradeMode,
    /// If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data).
    #[serde(rename = "ForceRestart")]
    force_restart: Option<::models::ForceRestart>,
    /// The maximum amount of time to block processing of an upgrade domain and prevent loss of availability when there are unexpected issues. When this timeout expires, processing of the upgrade domain will proceed regardless of availability loss issues. The timeout is reset at the start of each upgrade domain. Valid values are between 0 and 42949672925 inclusive. (unsigned 32-bit integer).
    #[serde(rename = "ReplicaSetCheckTimeoutInMilliseconds")]
    replica_set_check_timeout_in_milliseconds:
        Option<::models::UpgradeReplicaSetCheckTimeout>,
    /// The compensating action to perform when a Monitored upgrade encounters monitoring policy or health policy violations. Invalid indicates the failure action is invalid. Rollback specifies that the upgrade will start rolling back automatically. Manual indicates that the upgrade will switch to UnmonitoredManual upgrade mode.
    #[serde(rename = "FailureAction")]
    failure_action: Option<::models::FailureAction>,
    /// The amount of time to wait after completing an upgrade domain before applying health policies. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds.
    #[serde(rename = "HealthCheckWaitDurationInMilliseconds")]
    health_check_wait_duration_in_milliseconds:
        Option<::models::HealthCheckWaitDuration>,
    /// The amount of time that the application or cluster must remain healthy before the upgrade proceeds to the next upgrade domain. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds.
    #[serde(rename = "HealthCheckStableDurationInMilliseconds")]
    health_check_stable_duration_in_milliseconds:
        Option<::models::HealthCheckStableDuration>,
    /// The amount of time to retry health evaluation when the application or cluster is unhealthy before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds.
    #[serde(rename = "HealthCheckRetryTimeoutInMilliseconds")]
    health_check_retry_timeout_in_milliseconds:
        Option<::models::HealthCheckRetryTimeout>,
    /// The amount of time the overall upgrade has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds.
    #[serde(rename = "UpgradeTimeoutInMilliseconds")]
    upgrade_timeout_in_milliseconds: Option<::models::UpgradeTimeout>,
    /// The amount of time each upgrade domain has to complete before FailureAction is executed. It is first interpreted as a string representing an ISO 8601 duration. If that fails, then it is interpreted as a number representing the total number of milliseconds.
    #[serde(rename = "UpgradeDomainTimeoutInMilliseconds")]
    upgrade_domain_timeout_in_milliseconds:
        Option<::models::UpgradeDomainTimeout>,
}

impl RollingUpgradeUpdateDescription {
    /// Describes the parameters for updating a rolling upgrade of application or cluster.
    pub fn new(
        rolling_upgrade_mode: ::models::UpgradeMode,
    ) -> RollingUpgradeUpdateDescription {
        RollingUpgradeUpdateDescription {
            rolling_upgrade_mode,
            force_restart: None,
            replica_set_check_timeout_in_milliseconds: None,
            failure_action: None,
            health_check_wait_duration_in_milliseconds: None,
            health_check_stable_duration_in_milliseconds: None,
            health_check_retry_timeout_in_milliseconds: None,
            upgrade_timeout_in_milliseconds: None,
            upgrade_domain_timeout_in_milliseconds: None,
        }
    }

    pub fn set_rolling_upgrade_mode(
        &mut self,
        rolling_upgrade_mode: ::models::UpgradeMode,
    ) {
        self.rolling_upgrade_mode = rolling_upgrade_mode;
    }

    pub fn with_rolling_upgrade_mode(
        mut self,
        rolling_upgrade_mode: ::models::UpgradeMode,
    ) -> RollingUpgradeUpdateDescription {
        self.rolling_upgrade_mode = rolling_upgrade_mode;
        self
    }

    pub fn rolling_upgrade_mode(&self) -> &::models::UpgradeMode {
        &self.rolling_upgrade_mode
    }

    pub fn set_force_restart(&mut self, force_restart: ::models::ForceRestart) {
        self.force_restart = Some(force_restart);
    }

    pub fn with_force_restart(
        mut self,
        force_restart: ::models::ForceRestart,
    ) -> RollingUpgradeUpdateDescription {
        self.force_restart = Some(force_restart);
        self
    }

    pub fn force_restart(&self) -> Option<&::models::ForceRestart> {
        self.force_restart.as_ref()
    }

    pub fn reset_force_restart(&mut self) {
        self.force_restart = None;
    }

    pub fn set_replica_set_check_timeout_in_milliseconds(
        &mut self,
        replica_set_check_timeout_in_milliseconds: ::models::UpgradeReplicaSetCheckTimeout,
    ) {
        self.replica_set_check_timeout_in_milliseconds =
            Some(replica_set_check_timeout_in_milliseconds);
    }

    pub fn with_replica_set_check_timeout_in_milliseconds(
        mut self,
        replica_set_check_timeout_in_milliseconds: ::models::UpgradeReplicaSetCheckTimeout,
    ) -> RollingUpgradeUpdateDescription {
        self.replica_set_check_timeout_in_milliseconds =
            Some(replica_set_check_timeout_in_milliseconds);
        self
    }

    pub fn replica_set_check_timeout_in_milliseconds(
        &self,
    ) -> Option<&::models::UpgradeReplicaSetCheckTimeout> {
        self.replica_set_check_timeout_in_milliseconds.as_ref()
    }

    pub fn reset_replica_set_check_timeout_in_milliseconds(&mut self) {
        self.replica_set_check_timeout_in_milliseconds = None;
    }

    pub fn set_failure_action(
        &mut self,
        failure_action: ::models::FailureAction,
    ) {
        self.failure_action = Some(failure_action);
    }

    pub fn with_failure_action(
        mut self,
        failure_action: ::models::FailureAction,
    ) -> RollingUpgradeUpdateDescription {
        self.failure_action = Some(failure_action);
        self
    }

    pub fn failure_action(&self) -> Option<&::models::FailureAction> {
        self.failure_action.as_ref()
    }

    pub fn reset_failure_action(&mut self) {
        self.failure_action = None;
    }

    pub fn set_health_check_wait_duration_in_milliseconds(
        &mut self,
        health_check_wait_duration_in_milliseconds: ::models::HealthCheckWaitDuration,
    ) {
        self.health_check_wait_duration_in_milliseconds =
            Some(health_check_wait_duration_in_milliseconds);
    }

    pub fn with_health_check_wait_duration_in_milliseconds(
        mut self,
        health_check_wait_duration_in_milliseconds: ::models::HealthCheckWaitDuration,
    ) -> RollingUpgradeUpdateDescription {
        self.health_check_wait_duration_in_milliseconds =
            Some(health_check_wait_duration_in_milliseconds);
        self
    }

    pub fn health_check_wait_duration_in_milliseconds(
        &self,
    ) -> Option<&::models::HealthCheckWaitDuration> {
        self.health_check_wait_duration_in_milliseconds.as_ref()
    }

    pub fn reset_health_check_wait_duration_in_milliseconds(&mut self) {
        self.health_check_wait_duration_in_milliseconds = None;
    }

    pub fn set_health_check_stable_duration_in_milliseconds(
        &mut self,
        health_check_stable_duration_in_milliseconds: ::models::HealthCheckStableDuration,
    ) {
        self.health_check_stable_duration_in_milliseconds =
            Some(health_check_stable_duration_in_milliseconds);
    }

    pub fn with_health_check_stable_duration_in_milliseconds(
        mut self,
        health_check_stable_duration_in_milliseconds: ::models::HealthCheckStableDuration,
    ) -> RollingUpgradeUpdateDescription {
        self.health_check_stable_duration_in_milliseconds =
            Some(health_check_stable_duration_in_milliseconds);
        self
    }

    pub fn health_check_stable_duration_in_milliseconds(
        &self,
    ) -> Option<&::models::HealthCheckStableDuration> {
        self.health_check_stable_duration_in_milliseconds.as_ref()
    }

    pub fn reset_health_check_stable_duration_in_milliseconds(&mut self) {
        self.health_check_stable_duration_in_milliseconds = None;
    }

    pub fn set_health_check_retry_timeout_in_milliseconds(
        &mut self,
        health_check_retry_timeout_in_milliseconds: ::models::HealthCheckRetryTimeout,
    ) {
        self.health_check_retry_timeout_in_milliseconds =
            Some(health_check_retry_timeout_in_milliseconds);
    }

    pub fn with_health_check_retry_timeout_in_milliseconds(
        mut self,
        health_check_retry_timeout_in_milliseconds: ::models::HealthCheckRetryTimeout,
    ) -> RollingUpgradeUpdateDescription {
        self.health_check_retry_timeout_in_milliseconds =
            Some(health_check_retry_timeout_in_milliseconds);
        self
    }

    pub fn health_check_retry_timeout_in_milliseconds(
        &self,
    ) -> Option<&::models::HealthCheckRetryTimeout> {
        self.health_check_retry_timeout_in_milliseconds.as_ref()
    }

    pub fn reset_health_check_retry_timeout_in_milliseconds(&mut self) {
        self.health_check_retry_timeout_in_milliseconds = None;
    }

    pub fn set_upgrade_timeout_in_milliseconds(
        &mut self,
        upgrade_timeout_in_milliseconds: ::models::UpgradeTimeout,
    ) {
        self.upgrade_timeout_in_milliseconds =
            Some(upgrade_timeout_in_milliseconds);
    }

    pub fn with_upgrade_timeout_in_milliseconds(
        mut self,
        upgrade_timeout_in_milliseconds: ::models::UpgradeTimeout,
    ) -> RollingUpgradeUpdateDescription {
        self.upgrade_timeout_in_milliseconds =
            Some(upgrade_timeout_in_milliseconds);
        self
    }

    pub fn upgrade_timeout_in_milliseconds(
        &self,
    ) -> Option<&::models::UpgradeTimeout> {
        self.upgrade_timeout_in_milliseconds.as_ref()
    }

    pub fn reset_upgrade_timeout_in_milliseconds(&mut self) {
        self.upgrade_timeout_in_milliseconds = None;
    }

    pub fn set_upgrade_domain_timeout_in_milliseconds(
        &mut self,
        upgrade_domain_timeout_in_milliseconds: ::models::UpgradeDomainTimeout,
    ) {
        self.upgrade_domain_timeout_in_milliseconds =
            Some(upgrade_domain_timeout_in_milliseconds);
    }

    pub fn with_upgrade_domain_timeout_in_milliseconds(
        mut self,
        upgrade_domain_timeout_in_milliseconds: ::models::UpgradeDomainTimeout,
    ) -> RollingUpgradeUpdateDescription {
        self.upgrade_domain_timeout_in_milliseconds =
            Some(upgrade_domain_timeout_in_milliseconds);
        self
    }

    pub fn upgrade_domain_timeout_in_milliseconds(
        &self,
    ) -> Option<&::models::UpgradeDomainTimeout> {
        self.upgrade_domain_timeout_in_milliseconds.as_ref()
    }

    pub fn reset_upgrade_domain_timeout_in_milliseconds(&mut self) {
        self.upgrade_domain_timeout_in_milliseconds = None;
    }
}
