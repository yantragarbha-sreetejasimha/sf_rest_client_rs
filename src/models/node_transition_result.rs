/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeTransitionResult : Represents information about an operation in a terminal state (Completed or Faulted).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTransitionResult {
    /// If OperationState is Completed, this is 0.  If OperationState is Faulted, this is an error code indicating the reason.
    #[serde(rename = "ErrorCode")]
    error_code: Option<i32>,
    /// Contains information about a node that was targeted by a user-induced operation.
    #[serde(rename = "NodeResult")]
    node_result: Option<::models::NodeResult>,
}

impl Default for NodeTransitionResult {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeTransitionResult {
    /// Represents information about an operation in a terminal state (Completed or Faulted).
    pub fn new() -> NodeTransitionResult {
        NodeTransitionResult {
            error_code: None,
            node_result: None,
        }
    }

    pub fn set_error_code(&mut self, error_code: i32) {
        self.error_code = Some(error_code);
    }

    pub fn with_error_code(mut self, error_code: i32) -> NodeTransitionResult {
        self.error_code = Some(error_code);
        self
    }

    pub fn error_code(&self) -> Option<&i32> {
        self.error_code.as_ref()
    }

    pub fn reset_error_code(&mut self) {
        self.error_code = None;
    }

    pub fn set_node_result(&mut self, node_result: ::models::NodeResult) {
        self.node_result = Some(node_result);
    }

    pub fn with_node_result(
        mut self,
        node_result: ::models::NodeResult,
    ) -> NodeTransitionResult {
        self.node_result = Some(node_result);
        self
    }

    pub fn node_result(&self) -> Option<&::models::NodeResult> {
        self.node_result.as_ref()
    }

    pub fn reset_node_result(&mut self) {
        self.node_result = None;
    }
}
