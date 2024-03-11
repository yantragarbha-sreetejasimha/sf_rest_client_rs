/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceKind : The kind of service (Stateless or Stateful).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceKind {}

impl Default for ServiceKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceKind {
    /// The kind of service (Stateless or Stateful).
    pub fn new() -> ServiceKind {
        ServiceKind {}
    }
}

// TODO enum
// List of ServiceKind
//const (
//
//
//
//)
