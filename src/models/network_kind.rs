/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NetworkKind : The type of a Service Fabric container network.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkKind {}

impl Default for NetworkKind {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkKind {
    /// The type of a Service Fabric container network.
    pub fn new() -> NetworkKind {
        NetworkKind {}
    }
}

// TODO enum
// List of NetworkKind
//const (
//
//
//
//)