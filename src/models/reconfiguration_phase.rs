/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReconfigurationPhase : The reconfiguration phase of a replica of a stateful service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconfigurationPhase {}

impl Default for ReconfigurationPhase {
    fn default() -> Self {
        Self::new()
    }
}

impl ReconfigurationPhase {
    /// The reconfiguration phase of a replica of a stateful service.
    pub fn new() -> ReconfigurationPhase {
        ReconfigurationPhase {}
    }
}

// TODO enum
// List of ReconfigurationPhase
//const (
//
//
//
//)
