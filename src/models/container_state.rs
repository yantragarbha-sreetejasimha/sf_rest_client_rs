/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerState : The container state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerState {
    /// The state of this container
    #[serde(rename = "state")]
    state: Option<String>,
    /// Date/time when the container state started.
    #[serde(rename = "startTime")]
    start_time: Option<String>,
    /// The container exit code.
    #[serde(rename = "exitCode")]
    exit_code: Option<String>,
    /// Date/time when the container state finished.
    #[serde(rename = "finishTime")]
    finish_time: Option<String>,
    /// Human-readable status of this state.
    #[serde(rename = "detailStatus")]
    detail_status: Option<String>,
}

impl Default for ContainerState {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerState {
    /// The container state.
    pub fn new() -> ContainerState {
        ContainerState {
            state: None,
            start_time: None,
            exit_code: None,
            finish_time: None,
            detail_status: None,
        }
    }

    pub fn set_state(&mut self, state: String) {
        self.state = Some(state);
    }

    pub fn with_state(mut self, state: String) -> ContainerState {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&String> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }

    pub fn set_start_time(&mut self, start_time: String) {
        self.start_time = Some(start_time);
    }

    pub fn with_start_time(mut self, start_time: String) -> ContainerState {
        self.start_time = Some(start_time);
        self
    }

    pub fn start_time(&self) -> Option<&String> {
        self.start_time.as_ref()
    }

    pub fn reset_start_time(&mut self) {
        self.start_time = None;
    }

    pub fn set_exit_code(&mut self, exit_code: String) {
        self.exit_code = Some(exit_code);
    }

    pub fn with_exit_code(mut self, exit_code: String) -> ContainerState {
        self.exit_code = Some(exit_code);
        self
    }

    pub fn exit_code(&self) -> Option<&String> {
        self.exit_code.as_ref()
    }

    pub fn reset_exit_code(&mut self) {
        self.exit_code = None;
    }

    pub fn set_finish_time(&mut self, finish_time: String) {
        self.finish_time = Some(finish_time);
    }

    pub fn with_finish_time(mut self, finish_time: String) -> ContainerState {
        self.finish_time = Some(finish_time);
        self
    }

    pub fn finish_time(&self) -> Option<&String> {
        self.finish_time.as_ref()
    }

    pub fn reset_finish_time(&mut self) {
        self.finish_time = None;
    }

    pub fn set_detail_status(&mut self, detail_status: String) {
        self.detail_status = Some(detail_status);
    }

    pub fn with_detail_status(
        mut self,
        detail_status: String,
    ) -> ContainerState {
        self.detail_status = Some(detail_status);
        self
    }

    pub fn detail_status(&self) -> Option<&String> {
        self.detail_status.as_ref()
    }

    pub fn reset_detail_status(&mut self) {
        self.detail_status = None;
    }
}
