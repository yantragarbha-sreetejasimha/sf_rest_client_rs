/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CorrelationSchemeList : A list that describes the correlation of the service with other services.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrelationSchemeList {}

impl Default for CorrelationSchemeList {
    fn default() -> Self {
        Self::new()
    }
}

impl CorrelationSchemeList {
    /// A list that describes the correlation of the service with other services.
    pub fn new() -> CorrelationSchemeList {
        CorrelationSchemeList {}
    }
}
