/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScalingTriggerKind : Enumerates the ways that a service can be scaled.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalingTriggerKind {}

impl Default for ScalingTriggerKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ScalingTriggerKind {
    /// Enumerates the ways that a service can be scaled.
    pub fn new() -> ScalingTriggerKind {
        ScalingTriggerKind {}
    }
}

// TODO enum
// List of ScalingTriggerKind
//const (
//
//
//
//)
