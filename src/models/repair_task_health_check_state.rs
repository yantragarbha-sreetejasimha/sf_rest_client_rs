/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairTaskHealthCheckState : Specifies the workflow state of a repair task's health check. This type supports the Service Fabric platform; it is not meant to be used directly from your code.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairTaskHealthCheckState {}

impl Default for RepairTaskHealthCheckState {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairTaskHealthCheckState {
    /// Specifies the workflow state of a repair task's health check. This type supports the Service Fabric platform; it is not meant to be used directly from your code.
    pub fn new() -> RepairTaskHealthCheckState {
        RepairTaskHealthCheckState {}
    }
}

// TODO enum
// List of RepairTaskHealthCheckState
//const (
//
//
//
//)
