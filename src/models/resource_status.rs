/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResourceStatus : Status of the resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceStatus {}

impl Default for ResourceStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceStatus {
    /// Status of the resource.
    pub fn new() -> ResourceStatus {
        ResourceStatus {}
    }
}

// TODO enum
// List of ResourceStatus
//const (
//
//
//
//)
