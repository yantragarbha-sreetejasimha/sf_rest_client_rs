/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationUpgradeProgressInfo : Describes the parameters for an application upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationUpgradeProgressInfo {
    /// The name of the target application, including the 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: Option<::models::TargetApplicationName>,
    /// The application type name as defined in the application manifest.
    #[serde(rename = "TypeName")]
    type_name: Option<::models::ApplicationTypeName>,
    /// The target application type version (found in the application manifest) for the application upgrade.
    #[serde(rename = "TargetApplicationTypeVersion")]
    target_application_type_version:
        Option<::models::TargetApplicationTypeVersion>,
    /// List of upgrade domains and their statuses.
    #[serde(rename = "UpgradeDomains")]
    upgrade_domains: Option<::models::UpgradeDomainInfoList>,
    /// The state of the upgrade domain.
    #[serde(rename = "UpgradeState")]
    upgrade_state: Option<::models::UpgradeState>,
    /// The name of the next upgrade domain to be processed.
    #[serde(rename = "NextUpgradeDomain")]
    next_upgrade_domain: Option<::models::NextUpgradeDomain>,
    /// The mode used to monitor health during a rolling upgrade. The values are UnmonitoredAuto, UnmonitoredManual, and Monitored.
    #[serde(rename = "RollingUpgradeMode")]
    rolling_upgrade_mode: Option<::models::UpgradeMode>,
    /// Describes the parameters for an application upgrade. Note that upgrade description replaces the existing application description. This means that if the parameters are not specified, the existing parameters on the applications will be overwritten with the empty parameters list. This would result in the application using the default value of the parameters from the application manifest. If you do not want to change any existing parameter values, please get the application parameters first using the GetApplicationInfo query and then supply those values as Parameters in this ApplicationUpgradeDescription.
    #[serde(rename = "UpgradeDescription")]
    upgrade_description: Option<::models::ApplicationUpgradeDescription>,
    /// The estimated total amount of time spent processing the overall upgrade.
    #[serde(rename = "UpgradeDurationInMilliseconds")]
    upgrade_duration_in_milliseconds: Option<String>,
    /// The estimated total amount of time spent processing the current upgrade domain.
    #[serde(rename = "UpgradeDomainDurationInMilliseconds")]
    upgrade_domain_duration_in_milliseconds: Option<String>,
    /// List of health evaluations that resulted in the current aggregated health state.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
    /// Information about the current in-progress upgrade domain.
    #[serde(rename = "CurrentUpgradeDomainProgress")]
    current_upgrade_domain_progress:
        Option<::models::CurrentUpgradeDomainProgressInfo>,
    /// The estimated UTC datetime when the upgrade started.
    #[serde(rename = "StartTimestampUtc")]
    start_timestamp_utc: Option<String>,
    /// The estimated UTC datetime when the upgrade failed and FailureAction was executed.
    #[serde(rename = "FailureTimestampUtc")]
    failure_timestamp_utc: Option<String>,
    /// The cause of an upgrade failure that resulted in FailureAction being executed.
    #[serde(rename = "FailureReason")]
    failure_reason: Option<::models::FailureReason>,
    /// Information about the upgrade domain progress at the time of upgrade failure.
    #[serde(rename = "UpgradeDomainProgressAtFailure")]
    upgrade_domain_progress_at_failure:
        Option<::models::FailureUpgradeDomainProgressInfo>,
    /// Additional detailed information about the status of the pending upgrade.
    #[serde(rename = "UpgradeStatusDetails")]
    upgrade_status_details: Option<String>,
}

impl Default for ApplicationUpgradeProgressInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationUpgradeProgressInfo {
    /// Describes the parameters for an application upgrade.
    pub fn new() -> ApplicationUpgradeProgressInfo {
        ApplicationUpgradeProgressInfo {
            name: None,
            type_name: None,
            target_application_type_version: None,
            upgrade_domains: None,
            upgrade_state: None,
            next_upgrade_domain: None,
            rolling_upgrade_mode: None,
            upgrade_description: None,
            upgrade_duration_in_milliseconds: None,
            upgrade_domain_duration_in_milliseconds: None,
            unhealthy_evaluations: None,
            current_upgrade_domain_progress: None,
            start_timestamp_utc: None,
            failure_timestamp_utc: None,
            failure_reason: None,
            upgrade_domain_progress_at_failure: None,
            upgrade_status_details: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::TargetApplicationName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::TargetApplicationName,
    ) -> ApplicationUpgradeProgressInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::TargetApplicationName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_type_name(&mut self, type_name: ::models::ApplicationTypeName) {
        self.type_name = Some(type_name);
    }

    pub fn with_type_name(
        mut self,
        type_name: ::models::ApplicationTypeName,
    ) -> ApplicationUpgradeProgressInfo {
        self.type_name = Some(type_name);
        self
    }

    pub fn type_name(&self) -> Option<&::models::ApplicationTypeName> {
        self.type_name.as_ref()
    }

    pub fn reset_type_name(&mut self) {
        self.type_name = None;
    }

    pub fn set_target_application_type_version(
        &mut self,
        target_application_type_version: ::models::TargetApplicationTypeVersion,
    ) {
        self.target_application_type_version =
            Some(target_application_type_version);
    }

    pub fn with_target_application_type_version(
        mut self,
        target_application_type_version: ::models::TargetApplicationTypeVersion,
    ) -> ApplicationUpgradeProgressInfo {
        self.target_application_type_version =
            Some(target_application_type_version);
        self
    }

    pub fn target_application_type_version(
        &self,
    ) -> Option<&::models::TargetApplicationTypeVersion> {
        self.target_application_type_version.as_ref()
    }

    pub fn reset_target_application_type_version(&mut self) {
        self.target_application_type_version = None;
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
    ) -> ApplicationUpgradeProgressInfo {
        self.upgrade_domains = Some(upgrade_domains);
        self
    }

    pub fn upgrade_domains(&self) -> Option<&::models::UpgradeDomainInfoList> {
        self.upgrade_domains.as_ref()
    }

    pub fn reset_upgrade_domains(&mut self) {
        self.upgrade_domains = None;
    }

    pub fn set_upgrade_state(&mut self, upgrade_state: ::models::UpgradeState) {
        self.upgrade_state = Some(upgrade_state);
    }

    pub fn with_upgrade_state(
        mut self,
        upgrade_state: ::models::UpgradeState,
    ) -> ApplicationUpgradeProgressInfo {
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
    ) -> ApplicationUpgradeProgressInfo {
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
    ) -> ApplicationUpgradeProgressInfo {
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
        upgrade_description: ::models::ApplicationUpgradeDescription,
    ) {
        self.upgrade_description = Some(upgrade_description);
    }

    pub fn with_upgrade_description(
        mut self,
        upgrade_description: ::models::ApplicationUpgradeDescription,
    ) -> ApplicationUpgradeProgressInfo {
        self.upgrade_description = Some(upgrade_description);
        self
    }

    pub fn upgrade_description(
        &self,
    ) -> Option<&::models::ApplicationUpgradeDescription> {
        self.upgrade_description.as_ref()
    }

    pub fn reset_upgrade_description(&mut self) {
        self.upgrade_description = None;
    }

    pub fn set_upgrade_duration_in_milliseconds(
        &mut self,
        upgrade_duration_in_milliseconds: String,
    ) {
        self.upgrade_duration_in_milliseconds =
            Some(upgrade_duration_in_milliseconds);
    }

    pub fn with_upgrade_duration_in_milliseconds(
        mut self,
        upgrade_duration_in_milliseconds: String,
    ) -> ApplicationUpgradeProgressInfo {
        self.upgrade_duration_in_milliseconds =
            Some(upgrade_duration_in_milliseconds);
        self
    }

    pub fn upgrade_duration_in_milliseconds(&self) -> Option<&String> {
        self.upgrade_duration_in_milliseconds.as_ref()
    }

    pub fn reset_upgrade_duration_in_milliseconds(&mut self) {
        self.upgrade_duration_in_milliseconds = None;
    }

    pub fn set_upgrade_domain_duration_in_milliseconds(
        &mut self,
        upgrade_domain_duration_in_milliseconds: String,
    ) {
        self.upgrade_domain_duration_in_milliseconds =
            Some(upgrade_domain_duration_in_milliseconds);
    }

    pub fn with_upgrade_domain_duration_in_milliseconds(
        mut self,
        upgrade_domain_duration_in_milliseconds: String,
    ) -> ApplicationUpgradeProgressInfo {
        self.upgrade_domain_duration_in_milliseconds =
            Some(upgrade_domain_duration_in_milliseconds);
        self
    }

    pub fn upgrade_domain_duration_in_milliseconds(&self) -> Option<&String> {
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
    ) -> ApplicationUpgradeProgressInfo {
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
    ) -> ApplicationUpgradeProgressInfo {
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

    pub fn set_start_timestamp_utc(&mut self, start_timestamp_utc: String) {
        self.start_timestamp_utc = Some(start_timestamp_utc);
    }

    pub fn with_start_timestamp_utc(
        mut self,
        start_timestamp_utc: String,
    ) -> ApplicationUpgradeProgressInfo {
        self.start_timestamp_utc = Some(start_timestamp_utc);
        self
    }

    pub fn start_timestamp_utc(&self) -> Option<&String> {
        self.start_timestamp_utc.as_ref()
    }

    pub fn reset_start_timestamp_utc(&mut self) {
        self.start_timestamp_utc = None;
    }

    pub fn set_failure_timestamp_utc(&mut self, failure_timestamp_utc: String) {
        self.failure_timestamp_utc = Some(failure_timestamp_utc);
    }

    pub fn with_failure_timestamp_utc(
        mut self,
        failure_timestamp_utc: String,
    ) -> ApplicationUpgradeProgressInfo {
        self.failure_timestamp_utc = Some(failure_timestamp_utc);
        self
    }

    pub fn failure_timestamp_utc(&self) -> Option<&String> {
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
    ) -> ApplicationUpgradeProgressInfo {
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
        upgrade_domain_progress_at_failure: ::models::FailureUpgradeDomainProgressInfo,
    ) {
        self.upgrade_domain_progress_at_failure =
            Some(upgrade_domain_progress_at_failure);
    }

    pub fn with_upgrade_domain_progress_at_failure(
        mut self,
        upgrade_domain_progress_at_failure: ::models::FailureUpgradeDomainProgressInfo,
    ) -> ApplicationUpgradeProgressInfo {
        self.upgrade_domain_progress_at_failure =
            Some(upgrade_domain_progress_at_failure);
        self
    }

    pub fn upgrade_domain_progress_at_failure(
        &self,
    ) -> Option<&::models::FailureUpgradeDomainProgressInfo> {
        self.upgrade_domain_progress_at_failure.as_ref()
    }

    pub fn reset_upgrade_domain_progress_at_failure(&mut self) {
        self.upgrade_domain_progress_at_failure = None;
    }

    pub fn set_upgrade_status_details(
        &mut self,
        upgrade_status_details: String,
    ) {
        self.upgrade_status_details = Some(upgrade_status_details);
    }

    pub fn with_upgrade_status_details(
        mut self,
        upgrade_status_details: String,
    ) -> ApplicationUpgradeProgressInfo {
        self.upgrade_status_details = Some(upgrade_status_details);
        self
    }

    pub fn upgrade_status_details(&self) -> Option<&String> {
        self.upgrade_status_details.as_ref()
    }

    pub fn reset_upgrade_status_details(&mut self) {
        self.upgrade_status_details = None;
    }
}
