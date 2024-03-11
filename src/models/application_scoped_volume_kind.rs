/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationScopedVolumeKind : Specifies the application-scoped volume kind.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationScopedVolumeKind {}

impl Default for ApplicationScopedVolumeKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationScopedVolumeKind {
    /// Specifies the application-scoped volume kind.
    pub fn new() -> ApplicationScopedVolumeKind {
        ApplicationScopedVolumeKind {}
    }
}

// TODO enum
// List of ApplicationScopedVolumeKind
//const (
//
//
//
//)
