/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceName : The full name of the service with 'fabric:' URI scheme.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceName {}

impl Default for ServiceName {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceName {
    /// The full name of the service with 'fabric:' URI scheme.
    pub fn new() -> ServiceName {
        ServiceName {}
    }
}
