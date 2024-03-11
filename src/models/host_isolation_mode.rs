/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HostIsolationMode : Specifies the isolation mode of main entry point of a code package when it's host type is ContainerHost. This is specified as part of container host policies in application manifest while importing service manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HostIsolationMode {}

impl Default for HostIsolationMode {
    fn default() -> Self {
        Self::new()
    }
}

impl HostIsolationMode {
    /// Specifies the isolation mode of main entry point of a code package when it's host type is ContainerHost. This is specified as part of container host policies in application manifest while importing service manifest.
    pub fn new() -> HostIsolationMode {
        HostIsolationMode {}
    }
}

// TODO enum
// List of HostIsolationMode
//const (
//
//
//
//)
