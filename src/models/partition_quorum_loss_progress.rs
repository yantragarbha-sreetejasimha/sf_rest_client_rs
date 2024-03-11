/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionQuorumLossProgress : Information about a partition quorum loss user-induced operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionQuorumLossProgress {
    /// The state of the operation.
    #[serde(rename = "State")]
    state: Option<::models::OperationState>,
    /// Represents information about an operation in a terminal state (Completed or Faulted).
    #[serde(rename = "InvokeQuorumLossResult")]
    invoke_quorum_loss_result: Option<::models::InvokeQuorumLossResult>,
}

impl Default for PartitionQuorumLossProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionQuorumLossProgress {
    /// Information about a partition quorum loss user-induced operation.
    pub fn new() -> PartitionQuorumLossProgress {
        PartitionQuorumLossProgress {
            state: None,
            invoke_quorum_loss_result: None,
        }
    }

    pub fn set_state(&mut self, state: ::models::OperationState) {
        self.state = Some(state);
    }

    pub fn with_state(
        mut self,
        state: ::models::OperationState,
    ) -> PartitionQuorumLossProgress {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&::models::OperationState> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }

    pub fn set_invoke_quorum_loss_result(
        &mut self,
        invoke_quorum_loss_result: ::models::InvokeQuorumLossResult,
    ) {
        self.invoke_quorum_loss_result = Some(invoke_quorum_loss_result);
    }

    pub fn with_invoke_quorum_loss_result(
        mut self,
        invoke_quorum_loss_result: ::models::InvokeQuorumLossResult,
    ) -> PartitionQuorumLossProgress {
        self.invoke_quorum_loss_result = Some(invoke_quorum_loss_result);
        self
    }

    pub fn invoke_quorum_loss_result(
        &self,
    ) -> Option<&::models::InvokeQuorumLossResult> {
        self.invoke_quorum_loss_result.as_ref()
    }

    pub fn reset_invoke_quorum_loss_result(&mut self) {
        self.invoke_quorum_loss_result = None;
    }
}
