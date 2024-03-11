/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// OperationType : The type of the operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationType {}

impl Default for OperationType {
    fn default() -> Self {
        Self::new()
    }
}

impl OperationType {
    /// The type of the operation.
    pub fn new() -> OperationType {
        OperationType {}
    }
}

// TODO enum
// List of OperationType
//const (
//
//
//
//)
