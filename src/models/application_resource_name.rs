/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationResourceName : Application resource name.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationResourceName {}

impl Default for ApplicationResourceName {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationResourceName {
    /// Application resource name.
    pub fn new() -> ApplicationResourceName {
        ApplicationResourceName {}
    }
}
