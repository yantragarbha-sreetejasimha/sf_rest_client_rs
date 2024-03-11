/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationDefinitionKind : The mechanism used to define a Service Fabric application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationDefinitionKind {}

impl Default for ApplicationDefinitionKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationDefinitionKind {
    /// The mechanism used to define a Service Fabric application.
    pub fn new() -> ApplicationDefinitionKind {
        ApplicationDefinitionKind {}
    }
}

// TODO enum
// List of ApplicationDefinitionKind
//const (
//
//
//
//)