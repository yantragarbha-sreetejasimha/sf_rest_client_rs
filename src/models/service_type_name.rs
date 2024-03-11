/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeName : Name of the service type as specified in the service manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeName {}

impl Default for ServiceTypeName {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeName {
    /// Name of the service type as specified in the service manifest.
    pub fn new() -> ServiceTypeName {
        ServiceTypeName {}
    }
}
