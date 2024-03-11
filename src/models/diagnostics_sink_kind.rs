/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DiagnosticsSinkKind : The kind of DiagnosticsSink.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsSinkKind {}

impl Default for DiagnosticsSinkKind {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsSinkKind {
    /// The kind of DiagnosticsSink.
    pub fn new() -> DiagnosticsSinkKind {
        DiagnosticsSinkKind {}
    }
}

// TODO enum
// List of DiagnosticsSinkKind
//const (
//
//
//
//)