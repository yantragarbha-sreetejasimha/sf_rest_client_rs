/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterConfigurationUpgradeDescription : Describes the parameters for a standalone cluster configuration upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfigurationUpgradeDescription {
    /// The cluster configuration.
    #[serde(rename = "ClusterConfig")]
    cluster_config: String,
    /// The length of time between attempts to perform a health checks if the application or cluster is not healthy.
    #[serde(rename = "HealthCheckRetryTimeout")]
    health_check_retry_timeout: Option<String>,
    /// The length of time to wait after completing an upgrade domain before starting the health checks process.
    #[serde(rename = "HealthCheckWaitDurationInSeconds")]
    health_check_wait_duration_in_seconds: Option<String>,
    /// The length of time that the application or cluster must remain healthy.
    #[serde(rename = "HealthCheckStableDurationInSeconds")]
    health_check_stable_duration_in_seconds: Option<String>,
    /// The timeout for the upgrade domain.
    #[serde(rename = "UpgradeDomainTimeoutInSeconds")]
    upgrade_domain_timeout_in_seconds: Option<String>,
    /// The upgrade timeout.
    #[serde(rename = "UpgradeTimeoutInSeconds")]
    upgrade_timeout_in_seconds: Option<String>,
    /// The maximum allowed percentage of unhealthy applications during the upgrade. Allowed values are integer values from zero to 100.
    #[serde(rename = "MaxPercentUnhealthyApplications")]
    max_percent_unhealthy_applications: Option<i32>,
    /// The maximum allowed percentage of unhealthy nodes during the upgrade. Allowed values are integer values from zero to 100.
    #[serde(rename = "MaxPercentUnhealthyNodes")]
    max_percent_unhealthy_nodes: Option<i32>,
    /// The maximum allowed percentage of delta health degradation during the upgrade. Allowed values are integer values from zero to 100.
    #[serde(rename = "MaxPercentDeltaUnhealthyNodes")]
    max_percent_delta_unhealthy_nodes: Option<i32>,
    /// The maximum allowed percentage of upgrade domain delta health degradation during the upgrade. Allowed values are integer values from zero to 100.
    #[serde(rename = "MaxPercentUpgradeDomainDeltaUnhealthyNodes")]
    max_percent_upgrade_domain_delta_unhealthy_nodes: Option<i32>,
    /// Defines the application health policy map used to evaluate the health of an application or one of its children entities.
    #[serde(rename = "ApplicationHealthPolicies")]
    application_health_policies: Option<::models::ApplicationHealthPolicies>,
}

impl ClusterConfigurationUpgradeDescription {
    /// Describes the parameters for a standalone cluster configuration upgrade.
    pub fn new(
        cluster_config: String,
    ) -> ClusterConfigurationUpgradeDescription {
        ClusterConfigurationUpgradeDescription {
            cluster_config,
            health_check_retry_timeout: None,
            health_check_wait_duration_in_seconds: None,
            health_check_stable_duration_in_seconds: None,
            upgrade_domain_timeout_in_seconds: None,
            upgrade_timeout_in_seconds: None,
            max_percent_unhealthy_applications: None,
            max_percent_unhealthy_nodes: None,
            max_percent_delta_unhealthy_nodes: None,
            max_percent_upgrade_domain_delta_unhealthy_nodes: None,
            application_health_policies: None,
        }
    }

    pub fn set_cluster_config(&mut self, cluster_config: String) {
        self.cluster_config = cluster_config;
    }

    pub fn with_cluster_config(
        mut self,
        cluster_config: String,
    ) -> ClusterConfigurationUpgradeDescription {
        self.cluster_config = cluster_config;
        self
    }

    pub fn cluster_config(&self) -> &String {
        &self.cluster_config
    }

    pub fn set_health_check_retry_timeout(
        &mut self,
        health_check_retry_timeout: String,
    ) {
        self.health_check_retry_timeout = Some(health_check_retry_timeout);
    }

    pub fn with_health_check_retry_timeout(
        mut self,
        health_check_retry_timeout: String,
    ) -> ClusterConfigurationUpgradeDescription {
        self.health_check_retry_timeout = Some(health_check_retry_timeout);
        self
    }

    pub fn health_check_retry_timeout(&self) -> Option<&String> {
        self.health_check_retry_timeout.as_ref()
    }

    pub fn reset_health_check_retry_timeout(&mut self) {
        self.health_check_retry_timeout = None;
    }

    pub fn set_health_check_wait_duration_in_seconds(
        &mut self,
        health_check_wait_duration_in_seconds: String,
    ) {
        self.health_check_wait_duration_in_seconds =
            Some(health_check_wait_duration_in_seconds);
    }

    pub fn with_health_check_wait_duration_in_seconds(
        mut self,
        health_check_wait_duration_in_seconds: String,
    ) -> ClusterConfigurationUpgradeDescription {
        self.health_check_wait_duration_in_seconds =
            Some(health_check_wait_duration_in_seconds);
        self
    }

    pub fn health_check_wait_duration_in_seconds(&self) -> Option<&String> {
        self.health_check_wait_duration_in_seconds.as_ref()
    }

    pub fn reset_health_check_wait_duration_in_seconds(&mut self) {
        self.health_check_wait_duration_in_seconds = None;
    }

    pub fn set_health_check_stable_duration_in_seconds(
        &mut self,
        health_check_stable_duration_in_seconds: String,
    ) {
        self.health_check_stable_duration_in_seconds =
            Some(health_check_stable_duration_in_seconds);
    }

    pub fn with_health_check_stable_duration_in_seconds(
        mut self,
        health_check_stable_duration_in_seconds: String,
    ) -> ClusterConfigurationUpgradeDescription {
        self.health_check_stable_duration_in_seconds =
            Some(health_check_stable_duration_in_seconds);
        self
    }

    pub fn health_check_stable_duration_in_seconds(&self) -> Option<&String> {
        self.health_check_stable_duration_in_seconds.as_ref()
    }

    pub fn reset_health_check_stable_duration_in_seconds(&mut self) {
        self.health_check_stable_duration_in_seconds = None;
    }

    pub fn set_upgrade_domain_timeout_in_seconds(
        &mut self,
        upgrade_domain_timeout_in_seconds: String,
    ) {
        self.upgrade_domain_timeout_in_seconds =
            Some(upgrade_domain_timeout_in_seconds);
    }

    pub fn with_upgrade_domain_timeout_in_seconds(
        mut self,
        upgrade_domain_timeout_in_seconds: String,
    ) -> ClusterConfigurationUpgradeDescription {
        self.upgrade_domain_timeout_in_seconds =
            Some(upgrade_domain_timeout_in_seconds);
        self
    }

    pub fn upgrade_domain_timeout_in_seconds(&self) -> Option<&String> {
        self.upgrade_domain_timeout_in_seconds.as_ref()
    }

    pub fn reset_upgrade_domain_timeout_in_seconds(&mut self) {
        self.upgrade_domain_timeout_in_seconds = None;
    }

    pub fn set_upgrade_timeout_in_seconds(
        &mut self,
        upgrade_timeout_in_seconds: String,
    ) {
        self.upgrade_timeout_in_seconds = Some(upgrade_timeout_in_seconds);
    }

    pub fn with_upgrade_timeout_in_seconds(
        mut self,
        upgrade_timeout_in_seconds: String,
    ) -> ClusterConfigurationUpgradeDescription {
        self.upgrade_timeout_in_seconds = Some(upgrade_timeout_in_seconds);
        self
    }

    pub fn upgrade_timeout_in_seconds(&self) -> Option<&String> {
        self.upgrade_timeout_in_seconds.as_ref()
    }

    pub fn reset_upgrade_timeout_in_seconds(&mut self) {
        self.upgrade_timeout_in_seconds = None;
    }

    pub fn set_max_percent_unhealthy_applications(
        &mut self,
        max_percent_unhealthy_applications: i32,
    ) {
        self.max_percent_unhealthy_applications =
            Some(max_percent_unhealthy_applications);
    }

    pub fn with_max_percent_unhealthy_applications(
        mut self,
        max_percent_unhealthy_applications: i32,
    ) -> ClusterConfigurationUpgradeDescription {
        self.max_percent_unhealthy_applications =
            Some(max_percent_unhealthy_applications);
        self
    }

    pub fn max_percent_unhealthy_applications(&self) -> Option<&i32> {
        self.max_percent_unhealthy_applications.as_ref()
    }

    pub fn reset_max_percent_unhealthy_applications(&mut self) {
        self.max_percent_unhealthy_applications = None;
    }

    pub fn set_max_percent_unhealthy_nodes(
        &mut self,
        max_percent_unhealthy_nodes: i32,
    ) {
        self.max_percent_unhealthy_nodes = Some(max_percent_unhealthy_nodes);
    }

    pub fn with_max_percent_unhealthy_nodes(
        mut self,
        max_percent_unhealthy_nodes: i32,
    ) -> ClusterConfigurationUpgradeDescription {
        self.max_percent_unhealthy_nodes = Some(max_percent_unhealthy_nodes);
        self
    }

    pub fn max_percent_unhealthy_nodes(&self) -> Option<&i32> {
        self.max_percent_unhealthy_nodes.as_ref()
    }

    pub fn reset_max_percent_unhealthy_nodes(&mut self) {
        self.max_percent_unhealthy_nodes = None;
    }

    pub fn set_max_percent_delta_unhealthy_nodes(
        &mut self,
        max_percent_delta_unhealthy_nodes: i32,
    ) {
        self.max_percent_delta_unhealthy_nodes =
            Some(max_percent_delta_unhealthy_nodes);
    }

    pub fn with_max_percent_delta_unhealthy_nodes(
        mut self,
        max_percent_delta_unhealthy_nodes: i32,
    ) -> ClusterConfigurationUpgradeDescription {
        self.max_percent_delta_unhealthy_nodes =
            Some(max_percent_delta_unhealthy_nodes);
        self
    }

    pub fn max_percent_delta_unhealthy_nodes(&self) -> Option<&i32> {
        self.max_percent_delta_unhealthy_nodes.as_ref()
    }

    pub fn reset_max_percent_delta_unhealthy_nodes(&mut self) {
        self.max_percent_delta_unhealthy_nodes = None;
    }

    pub fn set_max_percent_upgrade_domain_delta_unhealthy_nodes(
        &mut self,
        max_percent_upgrade_domain_delta_unhealthy_nodes: i32,
    ) {
        self.max_percent_upgrade_domain_delta_unhealthy_nodes =
            Some(max_percent_upgrade_domain_delta_unhealthy_nodes);
    }

    pub fn with_max_percent_upgrade_domain_delta_unhealthy_nodes(
        mut self,
        max_percent_upgrade_domain_delta_unhealthy_nodes: i32,
    ) -> ClusterConfigurationUpgradeDescription {
        self.max_percent_upgrade_domain_delta_unhealthy_nodes =
            Some(max_percent_upgrade_domain_delta_unhealthy_nodes);
        self
    }

    pub fn max_percent_upgrade_domain_delta_unhealthy_nodes(
        &self,
    ) -> Option<&i32> {
        self.max_percent_upgrade_domain_delta_unhealthy_nodes
            .as_ref()
    }

    pub fn reset_max_percent_upgrade_domain_delta_unhealthy_nodes(&mut self) {
        self.max_percent_upgrade_domain_delta_unhealthy_nodes = None;
    }

    pub fn set_application_health_policies(
        &mut self,
        application_health_policies: ::models::ApplicationHealthPolicies,
    ) {
        self.application_health_policies = Some(application_health_policies);
    }

    pub fn with_application_health_policies(
        mut self,
        application_health_policies: ::models::ApplicationHealthPolicies,
    ) -> ClusterConfigurationUpgradeDescription {
        self.application_health_policies = Some(application_health_policies);
        self
    }

    pub fn application_health_policies(
        &self,
    ) -> Option<&::models::ApplicationHealthPolicies> {
        self.application_health_policies.as_ref()
    }

    pub fn reset_application_health_policies(&mut self) {
        self.application_health_policies = None;
    }
}
