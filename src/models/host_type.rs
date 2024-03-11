/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HostType : Specifies the type of host for main entry point of a code package as specified in service manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HostType {}

impl Default for HostType {
    fn default() -> Self {
        Self::new()
    }
}

impl HostType {
    /// Specifies the type of host for main entry point of a code package as specified in service manifest.
    pub fn new() -> HostType {
        HostType {}
    }
}

// TODO enum
// List of HostType
//const (
//
//
//
//)