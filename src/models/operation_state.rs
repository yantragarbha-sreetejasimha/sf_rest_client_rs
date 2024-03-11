/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// OperationState : The state of the operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationState {}

impl Default for OperationState {
    fn default() -> Self {
        Self::new()
    }
}

impl OperationState {
    /// The state of the operation.
    pub fn new() -> OperationState {
        OperationState {}
    }
}

// TODO enum
// List of OperationState
//const (
//
//
//
//)
