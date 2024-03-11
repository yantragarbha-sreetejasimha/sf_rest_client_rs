/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FailureReason : The cause of an upgrade failure that resulted in FailureAction being executed.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FailureReason {}

impl Default for FailureReason {
    fn default() -> Self {
        Self::new()
    }
}

impl FailureReason {
    /// The cause of an upgrade failure that resulted in FailureAction being executed.
    pub fn new() -> FailureReason {
        FailureReason {}
    }
}

// TODO enum
// List of FailureReason
//const (
//
//
//
//)
