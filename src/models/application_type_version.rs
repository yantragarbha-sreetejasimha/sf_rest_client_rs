/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeVersion : The version of the application type as defined in the application manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeVersion {}

impl Default for ApplicationTypeVersion {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationTypeVersion {
    /// The version of the application type as defined in the application manifest.
    pub fn new() -> ApplicationTypeVersion {
        ApplicationTypeVersion {}
    }
}
