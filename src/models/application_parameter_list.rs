/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationParameterList : List of application parameters with overridden values from their default values specified in the application manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationParameterList {}

impl Default for ApplicationParameterList {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationParameterList {
    /// List of application parameters with overridden values from their default values specified in the application manifest.
    pub fn new() -> ApplicationParameterList {
        ApplicationParameterList {}
    }
}
