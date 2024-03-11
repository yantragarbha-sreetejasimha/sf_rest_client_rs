/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeName : The application type name as defined in the application manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeName {}

impl Default for ApplicationTypeName {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationTypeName {
    /// The application type name as defined in the application manifest.
    pub fn new() -> ApplicationTypeName {
        ApplicationTypeName {}
    }
}
