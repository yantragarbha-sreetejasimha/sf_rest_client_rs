/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoScalingTriggerKind : Enumerates the triggers for auto scaling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoScalingTriggerKind {}

impl Default for AutoScalingTriggerKind {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoScalingTriggerKind {
    /// Enumerates the triggers for auto scaling.
    pub fn new() -> AutoScalingTriggerKind {
        AutoScalingTriggerKind {}
    }
}

// TODO enum
// List of AutoScalingTriggerKind
//const (
//
//
//
//)
