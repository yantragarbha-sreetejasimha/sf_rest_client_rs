/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeExtensionDescriptionList : List of service type extensions.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeExtensionDescriptionList {}

impl Default for ServiceTypeExtensionDescriptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeExtensionDescriptionList {
    /// List of service type extensions.
    pub fn new() -> ServiceTypeExtensionDescriptionList {
        ServiceTypeExtensionDescriptionList {}
    }
}
