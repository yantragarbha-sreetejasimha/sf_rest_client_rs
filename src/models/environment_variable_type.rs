/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EnvironmentVariableType : The type of the environment variable being given in value

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentVariableType {}

impl Default for EnvironmentVariableType {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentVariableType {
    /// The type of the environment variable being given in value
    pub fn new() -> EnvironmentVariableType {
        EnvironmentVariableType {}
    }
}

// TODO enum
// List of EnvironmentVariableType
//const (
//
//
//
//)
