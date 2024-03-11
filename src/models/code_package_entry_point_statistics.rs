/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CodePackageEntryPointStatistics : Statistics about setup or main entry point  of a code package deployed on a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePackageEntryPointStatistics {
    /// The last exit code of the entry point.
    #[serde(rename = "LastExitCode")]
    last_exit_code: Option<String>,
    /// The last time (in UTC) when Service Fabric attempted to run the entry point.
    #[serde(rename = "LastActivationTime")]
    last_activation_time: Option<String>,
    /// The last time (in UTC) when the entry point finished running.
    #[serde(rename = "LastExitTime")]
    last_exit_time: Option<String>,
    /// The last time (in UTC) when the entry point ran successfully.
    #[serde(rename = "LastSuccessfulActivationTime")]
    last_successful_activation_time: Option<String>,
    /// The last time (in UTC) when the entry point finished running gracefully.
    #[serde(rename = "LastSuccessfulExitTime")]
    last_successful_exit_time: Option<String>,
    /// Number of times the entry point has run.
    #[serde(rename = "ActivationCount")]
    activation_count: Option<String>,
    /// Number of times the entry point failed to run.
    #[serde(rename = "ActivationFailureCount")]
    activation_failure_count: Option<String>,
    /// Number of times the entry point continuously failed to run.
    #[serde(rename = "ContinuousActivationFailureCount")]
    continuous_activation_failure_count: Option<String>,
    /// Number of times the entry point finished running.
    #[serde(rename = "ExitCount")]
    exit_count: Option<String>,
    /// Number of times the entry point failed to exit gracefully.
    #[serde(rename = "ExitFailureCount")]
    exit_failure_count: Option<String>,
    /// Number of times the entry point continuously failed to exit gracefully.
    #[serde(rename = "ContinuousExitFailureCount")]
    continuous_exit_failure_count: Option<String>,
}

impl Default for CodePackageEntryPointStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl CodePackageEntryPointStatistics {
    /// Statistics about setup or main entry point  of a code package deployed on a Service Fabric node.
    pub fn new() -> CodePackageEntryPointStatistics {
        CodePackageEntryPointStatistics {
            last_exit_code: None,
            last_activation_time: None,
            last_exit_time: None,
            last_successful_activation_time: None,
            last_successful_exit_time: None,
            activation_count: None,
            activation_failure_count: None,
            continuous_activation_failure_count: None,
            exit_count: None,
            exit_failure_count: None,
            continuous_exit_failure_count: None,
        }
    }

    pub fn set_last_exit_code(&mut self, last_exit_code: String) {
        self.last_exit_code = Some(last_exit_code);
    }

    pub fn with_last_exit_code(
        mut self,
        last_exit_code: String,
    ) -> CodePackageEntryPointStatistics {
        self.last_exit_code = Some(last_exit_code);
        self
    }

    pub fn last_exit_code(&self) -> Option<&String> {
        self.last_exit_code.as_ref()
    }

    pub fn reset_last_exit_code(&mut self) {
        self.last_exit_code = None;
    }

    pub fn set_last_activation_time(&mut self, last_activation_time: String) {
        self.last_activation_time = Some(last_activation_time);
    }

    pub fn with_last_activation_time(
        mut self,
        last_activation_time: String,
    ) -> CodePackageEntryPointStatistics {
        self.last_activation_time = Some(last_activation_time);
        self
    }

    pub fn last_activation_time(&self) -> Option<&String> {
        self.last_activation_time.as_ref()
    }

    pub fn reset_last_activation_time(&mut self) {
        self.last_activation_time = None;
    }

    pub fn set_last_exit_time(&mut self, last_exit_time: String) {
        self.last_exit_time = Some(last_exit_time);
    }

    pub fn with_last_exit_time(
        mut self,
        last_exit_time: String,
    ) -> CodePackageEntryPointStatistics {
        self.last_exit_time = Some(last_exit_time);
        self
    }

    pub fn last_exit_time(&self) -> Option<&String> {
        self.last_exit_time.as_ref()
    }

    pub fn reset_last_exit_time(&mut self) {
        self.last_exit_time = None;
    }

    pub fn set_last_successful_activation_time(
        &mut self,
        last_successful_activation_time: String,
    ) {
        self.last_successful_activation_time =
            Some(last_successful_activation_time);
    }

    pub fn with_last_successful_activation_time(
        mut self,
        last_successful_activation_time: String,
    ) -> CodePackageEntryPointStatistics {
        self.last_successful_activation_time =
            Some(last_successful_activation_time);
        self
    }

    pub fn last_successful_activation_time(&self) -> Option<&String> {
        self.last_successful_activation_time.as_ref()
    }

    pub fn reset_last_successful_activation_time(&mut self) {
        self.last_successful_activation_time = None;
    }

    pub fn set_last_successful_exit_time(
        &mut self,
        last_successful_exit_time: String,
    ) {
        self.last_successful_exit_time = Some(last_successful_exit_time);
    }

    pub fn with_last_successful_exit_time(
        mut self,
        last_successful_exit_time: String,
    ) -> CodePackageEntryPointStatistics {
        self.last_successful_exit_time = Some(last_successful_exit_time);
        self
    }

    pub fn last_successful_exit_time(&self) -> Option<&String> {
        self.last_successful_exit_time.as_ref()
    }

    pub fn reset_last_successful_exit_time(&mut self) {
        self.last_successful_exit_time = None;
    }

    pub fn set_activation_count(&mut self, activation_count: String) {
        self.activation_count = Some(activation_count);
    }

    pub fn with_activation_count(
        mut self,
        activation_count: String,
    ) -> CodePackageEntryPointStatistics {
        self.activation_count = Some(activation_count);
        self
    }

    pub fn activation_count(&self) -> Option<&String> {
        self.activation_count.as_ref()
    }

    pub fn reset_activation_count(&mut self) {
        self.activation_count = None;
    }

    pub fn set_activation_failure_count(
        &mut self,
        activation_failure_count: String,
    ) {
        self.activation_failure_count = Some(activation_failure_count);
    }

    pub fn with_activation_failure_count(
        mut self,
        activation_failure_count: String,
    ) -> CodePackageEntryPointStatistics {
        self.activation_failure_count = Some(activation_failure_count);
        self
    }

    pub fn activation_failure_count(&self) -> Option<&String> {
        self.activation_failure_count.as_ref()
    }

    pub fn reset_activation_failure_count(&mut self) {
        self.activation_failure_count = None;
    }

    pub fn set_continuous_activation_failure_count(
        &mut self,
        continuous_activation_failure_count: String,
    ) {
        self.continuous_activation_failure_count =
            Some(continuous_activation_failure_count);
    }

    pub fn with_continuous_activation_failure_count(
        mut self,
        continuous_activation_failure_count: String,
    ) -> CodePackageEntryPointStatistics {
        self.continuous_activation_failure_count =
            Some(continuous_activation_failure_count);
        self
    }

    pub fn continuous_activation_failure_count(&self) -> Option<&String> {
        self.continuous_activation_failure_count.as_ref()
    }

    pub fn reset_continuous_activation_failure_count(&mut self) {
        self.continuous_activation_failure_count = None;
    }

    pub fn set_exit_count(&mut self, exit_count: String) {
        self.exit_count = Some(exit_count);
    }

    pub fn with_exit_count(
        mut self,
        exit_count: String,
    ) -> CodePackageEntryPointStatistics {
        self.exit_count = Some(exit_count);
        self
    }

    pub fn exit_count(&self) -> Option<&String> {
        self.exit_count.as_ref()
    }

    pub fn reset_exit_count(&mut self) {
        self.exit_count = None;
    }

    pub fn set_exit_failure_count(&mut self, exit_failure_count: String) {
        self.exit_failure_count = Some(exit_failure_count);
    }

    pub fn with_exit_failure_count(
        mut self,
        exit_failure_count: String,
    ) -> CodePackageEntryPointStatistics {
        self.exit_failure_count = Some(exit_failure_count);
        self
    }

    pub fn exit_failure_count(&self) -> Option<&String> {
        self.exit_failure_count.as_ref()
    }

    pub fn reset_exit_failure_count(&mut self) {
        self.exit_failure_count = None;
    }

    pub fn set_continuous_exit_failure_count(
        &mut self,
        continuous_exit_failure_count: String,
    ) {
        self.continuous_exit_failure_count =
            Some(continuous_exit_failure_count);
    }

    pub fn with_continuous_exit_failure_count(
        mut self,
        continuous_exit_failure_count: String,
    ) -> CodePackageEntryPointStatistics {
        self.continuous_exit_failure_count =
            Some(continuous_exit_failure_count);
        self
    }

    pub fn continuous_exit_failure_count(&self) -> Option<&String> {
        self.continuous_exit_failure_count.as_ref()
    }

    pub fn reset_continuous_exit_failure_count(&mut self) {
        self.continuous_exit_failure_count = None;
    }
}
