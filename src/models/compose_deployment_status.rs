/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ComposeDeploymentStatus : The status of the compose deployment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComposeDeploymentStatus {}

impl Default for ComposeDeploymentStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ComposeDeploymentStatus {
    /// The status of the compose deployment.
    pub fn new() -> ComposeDeploymentStatus {
        ComposeDeploymentStatus {}
    }
}

// TODO enum
// List of ComposeDeploymentStatus
//const (
//
//
//
//)
