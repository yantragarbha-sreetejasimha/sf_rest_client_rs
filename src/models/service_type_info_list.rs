/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeInfoList : List of service type information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeInfoList {}

impl Default for ServiceTypeInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeInfoList {
    /// List of service type information.
    pub fn new() -> ServiceTypeInfoList {
        ServiceTypeInfoList {}
    }
}
