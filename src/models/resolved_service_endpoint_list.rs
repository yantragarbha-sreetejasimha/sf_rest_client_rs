/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResolvedServiceEndpointList : List of resolved service endpoints of a service partition.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolvedServiceEndpointList {}

impl Default for ResolvedServiceEndpointList {
    fn default() -> Self {
        Self::new()
    }
}

impl ResolvedServiceEndpointList {
    /// List of resolved service endpoints of a service partition.
    pub fn new() -> ResolvedServiceEndpointList {
        ResolvedServiceEndpointList {}
    }
}