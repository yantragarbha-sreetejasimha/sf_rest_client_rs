/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeTransitionProgress : Information about an NodeTransition operation.  This class contains an OperationState and a NodeTransitionResult.  The NodeTransitionResult is not valid until OperationState is Completed or Faulted.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTransitionProgress {
    /// The state of the operation.
    #[serde(rename = "State")]
    state: Option<::models::OperationState>,
    /// Represents information about an operation in a terminal state (Completed or Faulted).
    #[serde(rename = "NodeTransitionResult")]
    node_transition_result: Option<::models::NodeTransitionResult>,
}

impl Default for NodeTransitionProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeTransitionProgress {
    /// Information about an NodeTransition operation.  This class contains an OperationState and a NodeTransitionResult.  The NodeTransitionResult is not valid until OperationState is Completed or Faulted.
    pub fn new() -> NodeTransitionProgress {
        NodeTransitionProgress {
            state: None,
            node_transition_result: None,
        }
    }

    pub fn set_state(&mut self, state: ::models::OperationState) {
        self.state = Some(state);
    }

    pub fn with_state(
        mut self,
        state: ::models::OperationState,
    ) -> NodeTransitionProgress {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&::models::OperationState> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }

    pub fn set_node_transition_result(
        &mut self,
        node_transition_result: ::models::NodeTransitionResult,
    ) {
        self.node_transition_result = Some(node_transition_result);
    }

    pub fn with_node_transition_result(
        mut self,
        node_transition_result: ::models::NodeTransitionResult,
    ) -> NodeTransitionProgress {
        self.node_transition_result = Some(node_transition_result);
        self
    }

    pub fn node_transition_result(
        &self,
    ) -> Option<&::models::NodeTransitionResult> {
        self.node_transition_result.as_ref()
    }

    pub fn reset_node_transition_result(&mut self) {
        self.node_transition_result = None;
    }
}
