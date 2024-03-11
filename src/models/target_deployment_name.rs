/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TargetDeploymentName : The name of the target deployment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetDeploymentName {}

impl Default for TargetDeploymentName {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetDeploymentName {
    /// The name of the target deployment.
    pub fn new() -> TargetDeploymentName {
        TargetDeploymentName {}
    }
}
