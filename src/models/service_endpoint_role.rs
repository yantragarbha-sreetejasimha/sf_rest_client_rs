/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceEndpointRole : The role of the replica where the endpoint is reported.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceEndpointRole {}

impl Default for ServiceEndpointRole {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceEndpointRole {
    /// The role of the replica where the endpoint is reported.
    pub fn new() -> ServiceEndpointRole {
        ServiceEndpointRole {}
    }
}

// TODO enum
// List of ServiceEndpointRole
//const (
//
//
//
//)
