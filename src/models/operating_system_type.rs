/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// OperatingSystemType : The operation system required by the code in service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatingSystemType {}

impl Default for OperatingSystemType {
    fn default() -> Self {
        Self::new()
    }
}

impl OperatingSystemType {
    /// The operation system required by the code in service.
    pub fn new() -> OperatingSystemType {
        OperatingSystemType {}
    }
}

// TODO enum
// List of OperatingSystemType
//const (
//
//
//
//)