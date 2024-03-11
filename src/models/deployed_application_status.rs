/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationStatus : The status of the application deployed on the node. Following are the possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationStatus {}

impl Default for DeployedApplicationStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedApplicationStatus {
    /// The status of the application deployed on the node. Following are the possible values.
    pub fn new() -> DeployedApplicationStatus {
        DeployedApplicationStatus {}
    }
}

// TODO enum
// List of DeployedApplicationStatus
//const (
//
//
//
//)
