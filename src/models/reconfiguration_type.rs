/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReconfigurationType : The type of reconfiguration for replica of a stateful service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconfigurationType {}

impl Default for ReconfigurationType {
    fn default() -> Self {
        Self::new()
    }
}

impl ReconfigurationType {
    /// The type of reconfiguration for replica of a stateful service.
    pub fn new() -> ReconfigurationType {
        ReconfigurationType {}
    }
}

// TODO enum
// List of ReconfigurationType
//const (
//
//
//
//)
