/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosEventKind : The kind of Chaos event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosEventKind {}

impl Default for ChaosEventKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosEventKind {
    /// The kind of Chaos event.
    pub fn new() -> ChaosEventKind {
        ChaosEventKind {}
    }
}

// TODO enum
// List of ChaosEventKind
//const (
//
//
//
//)
