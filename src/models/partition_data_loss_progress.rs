/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionDataLossProgress : Information about a partition data loss user-induced operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionDataLossProgress {
    /// The state of the operation.
    #[serde(rename = "State")]
    state: Option<::models::OperationState>,
    /// Represents information about an operation in a terminal state (Completed or Faulted).
    #[serde(rename = "InvokeDataLossResult")]
    invoke_data_loss_result: Option<::models::InvokeDataLossResult>,
}

impl Default for PartitionDataLossProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionDataLossProgress {
    /// Information about a partition data loss user-induced operation.
    pub fn new() -> PartitionDataLossProgress {
        PartitionDataLossProgress {
            state: None,
            invoke_data_loss_result: None,
        }
    }

    pub fn set_state(&mut self, state: ::models::OperationState) {
        self.state = Some(state);
    }

    pub fn with_state(
        mut self,
        state: ::models::OperationState,
    ) -> PartitionDataLossProgress {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&::models::OperationState> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }

    pub fn set_invoke_data_loss_result(
        &mut self,
        invoke_data_loss_result: ::models::InvokeDataLossResult,
    ) {
        self.invoke_data_loss_result = Some(invoke_data_loss_result);
    }

    pub fn with_invoke_data_loss_result(
        mut self,
        invoke_data_loss_result: ::models::InvokeDataLossResult,
    ) -> PartitionDataLossProgress {
        self.invoke_data_loss_result = Some(invoke_data_loss_result);
        self
    }

    pub fn invoke_data_loss_result(
        &self,
    ) -> Option<&::models::InvokeDataLossResult> {
        self.invoke_data_loss_result.as_ref()
    }

    pub fn reset_invoke_data_loss_result(&mut self) {
        self.invoke_data_loss_result = None;
    }
}
