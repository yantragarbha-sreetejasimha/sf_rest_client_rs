/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RestartPartitionResult : Represents information about an operation in a terminal state (Completed or Faulted).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RestartPartitionResult {
    /// If OperationState is Completed, this is 0.  If OperationState is Faulted, this is an error code indicating the reason.
    #[serde(rename = "ErrorCode")]
    error_code: Option<i32>,
    /// This class returns information about the partition that the user-induced operation acted upon.
    #[serde(rename = "SelectedPartition")]
    selected_partition: Option<::models::SelectedPartition>,
}

impl Default for RestartPartitionResult {
    fn default() -> Self {
        Self::new()
    }
}

impl RestartPartitionResult {
    /// Represents information about an operation in a terminal state (Completed or Faulted).
    pub fn new() -> RestartPartitionResult {
        RestartPartitionResult {
            error_code: None,
            selected_partition: None,
        }
    }

    pub fn set_error_code(&mut self, error_code: i32) {
        self.error_code = Some(error_code);
    }

    pub fn with_error_code(
        mut self,
        error_code: i32,
    ) -> RestartPartitionResult {
        self.error_code = Some(error_code);
        self
    }

    pub fn error_code(&self) -> Option<&i32> {
        self.error_code.as_ref()
    }

    pub fn reset_error_code(&mut self) {
        self.error_code = None;
    }

    pub fn set_selected_partition(
        &mut self,
        selected_partition: ::models::SelectedPartition,
    ) {
        self.selected_partition = Some(selected_partition);
    }

    pub fn with_selected_partition(
        mut self,
        selected_partition: ::models::SelectedPartition,
    ) -> RestartPartitionResult {
        self.selected_partition = Some(selected_partition);
        self
    }

    pub fn selected_partition(&self) -> Option<&::models::SelectedPartition> {
        self.selected_partition.as_ref()
    }

    pub fn reset_selected_partition(&mut self) {
        self.selected_partition = None;
    }
}
