/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExecutionPolicyType : Enumerates the execution policy types for services.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionPolicyType {}

impl Default for ExecutionPolicyType {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionPolicyType {
    /// Enumerates the execution policy types for services.
    pub fn new() -> ExecutionPolicyType {
        ExecutionPolicyType {}
    }
}

// TODO enum
// List of ExecutionPolicyType
//const (
//
//
//
//)
