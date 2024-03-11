/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeDefinitionKind : The mechanism used to define a Service Fabric application type.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeDefinitionKind {}

impl Default for ApplicationTypeDefinitionKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationTypeDefinitionKind {
    /// The mechanism used to define a Service Fabric application type.
    pub fn new() -> ApplicationTypeDefinitionKind {
        ApplicationTypeDefinitionKind {}
    }
}

// TODO enum
// List of ApplicationTypeDefinitionKind
//const (
//
//
//
//)
