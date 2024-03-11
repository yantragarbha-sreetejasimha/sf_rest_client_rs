/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ComposeDeploymentUpgradeDescription : Describes the parameters for a compose deployment upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComposeDeploymentUpgradeDescription {
    /// The name of the deployment.
    #[serde(rename = "DeploymentName")]
    deployment_name: ::models::DeploymentName,
    /// The content of the compose file that describes the deployment to create.
    #[serde(rename = "ComposeFileContent")]
    compose_file_content: String,
    /// Credential information to connect to container registry.
    #[serde(rename = "RegistryCredential")]
    registry_credential: Option<::models::RegistryCredential>,
    /// The kind of upgrade out of the following possible values.
    #[serde(rename = "UpgradeKind")]
    upgrade_kind: ::models::UpgradeKind,
    /// The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, Monitored, and UnmonitoredDeferred.
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
    /// Defines a health policy used to evaluate the health of an application or one of its children entities.
    #[serde(rename = "ApplicationHealthPolicy")]
    application_health_policy: Option<::models::ApplicationHealthPolicy>,
}

impl ComposeDeploymentUpgradeDescription {
    /// Describes the parameters for a compose deployment upgrade.
    pub fn new(
        deployment_name: ::models::DeploymentName,
        compose_file_content: String,
        upgrade_kind: ::models::UpgradeKind,
    ) -> ComposeDeploymentUpgradeDescription {
        ComposeDeploymentUpgradeDescription {
            deployment_name,
            compose_file_content,
            registry_credential: None,
            upgrade_kind,
            rolling_upgrade_mode: None,
            upgrade_replica_set_check_timeout_in_seconds: None,
            force_restart: None,
            monitoring_policy: None,
            application_health_policy: None,
        }
    }

    pub fn set_deployment_name(
        &mut self,
        deployment_name: ::models::DeploymentName,
    ) {
        self.deployment_name = deployment_name;
    }

    pub fn with_deployment_name(
        mut self,
        deployment_name: ::models::DeploymentName,
    ) -> ComposeDeploymentUpgradeDescription {
        self.deployment_name = deployment_name;
        self
    }

    pub fn deployment_name(&self) -> &::models::DeploymentName {
        &self.deployment_name
    }

    pub fn set_compose_file_content(&mut self, compose_file_content: String) {
        self.compose_file_content = compose_file_content;
    }

    pub fn with_compose_file_content(
        mut self,
        compose_file_content: String,
    ) -> ComposeDeploymentUpgradeDescription {
        self.compose_file_content = compose_file_content;
        self
    }

    pub fn compose_file_content(&self) -> &String {
        &self.compose_file_content
    }

    pub fn set_registry_credential(
        &mut self,
        registry_credential: ::models::RegistryCredential,
    ) {
        self.registry_credential = Some(registry_credential);
    }

    pub fn with_registry_credential(
        mut self,
        registry_credential: ::models::RegistryCredential,
    ) -> ComposeDeploymentUpgradeDescription {
        self.registry_credential = Some(registry_credential);
        self
    }

    pub fn registry_credential(&self) -> Option<&::models::RegistryCredential> {
        self.registry_credential.as_ref()
    }

    pub fn reset_registry_credential(&mut self) {
        self.registry_credential = None;
    }

    pub fn set_upgrade_kind(&mut self, upgrade_kind: ::models::UpgradeKind) {
        self.upgrade_kind = upgrade_kind;
    }

    pub fn with_upgrade_kind(
        mut self,
        upgrade_kind: ::models::UpgradeKind,
    ) -> ComposeDeploymentUpgradeDescription {
        self.upgrade_kind = upgrade_kind;
        self
    }

    pub fn upgrade_kind(&self) -> &::models::UpgradeKind {
        &self.upgrade_kind
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
    ) -> ComposeDeploymentUpgradeDescription {
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
    ) -> ComposeDeploymentUpgradeDescription {
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
    ) -> ComposeDeploymentUpgradeDescription {
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
    ) -> ComposeDeploymentUpgradeDescription {
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

    pub fn set_application_health_policy(
        &mut self,
        application_health_policy: ::models::ApplicationHealthPolicy,
    ) {
        self.application_health_policy = Some(application_health_policy);
    }

    pub fn with_application_health_policy(
        mut self,
        application_health_policy: ::models::ApplicationHealthPolicy,
    ) -> ComposeDeploymentUpgradeDescription {
        self.application_health_policy = Some(application_health_policy);
        self
    }

    pub fn application_health_policy(
        &self,
    ) -> Option<&::models::ApplicationHealthPolicy> {
        self.application_health_policy.as_ref()
    }

    pub fn reset_application_health_policy(&mut self) {
        self.application_health_policy = None;
    }
}
