/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PropertyName : The name of the Service Fabric property.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyName {}

impl Default for PropertyName {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyName {
    /// The name of the Service Fabric property.
    pub fn new() -> PropertyName {
        PropertyName {}
    }
}