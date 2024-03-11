/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FabricEventKind : The kind of FabricEvent.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricEventKind {}

impl Default for FabricEventKind {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricEventKind {
    /// The kind of FabricEvent.
    pub fn new() -> FabricEventKind {
        FabricEventKind {}
    }
}

// TODO enum
// List of FabricEventKind
//const (
//
//
//
//)
