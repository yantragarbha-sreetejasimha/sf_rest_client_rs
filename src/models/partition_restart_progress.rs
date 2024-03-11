/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionRestartProgress : Information about a partition restart user-induced operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionRestartProgress {
    /// The state of the operation.
    #[serde(rename = "State")]
    state: Option<::models::OperationState>,
    /// Represents information about an operation in a terminal state (Completed or Faulted).
    #[serde(rename = "RestartPartitionResult")]
    restart_partition_result: Option<::models::RestartPartitionResult>,
}

impl Default for PartitionRestartProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionRestartProgress {
    /// Information about a partition restart user-induced operation.
    pub fn new() -> PartitionRestartProgress {
        PartitionRestartProgress {
            state: None,
            restart_partition_result: None,
        }
    }

    pub fn set_state(&mut self, state: ::models::OperationState) {
        self.state = Some(state);
    }

    pub fn with_state(
        mut self,
        state: ::models::OperationState,
    ) -> PartitionRestartProgress {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&::models::OperationState> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }

    pub fn set_restart_partition_result(
        &mut self,
        restart_partition_result: ::models::RestartPartitionResult,
    ) {
        self.restart_partition_result = Some(restart_partition_result);
    }

    pub fn with_restart_partition_result(
        mut self,
        restart_partition_result: ::models::RestartPartitionResult,
    ) -> PartitionRestartProgress {
        self.restart_partition_result = Some(restart_partition_result);
        self
    }

    pub fn restart_partition_result(
        &self,
    ) -> Option<&::models::RestartPartitionResult> {
        self.restart_partition_result.as_ref()
    }

    pub fn reset_restart_partition_result(&mut self) {
        self.restart_partition_result = None;
    }
}
