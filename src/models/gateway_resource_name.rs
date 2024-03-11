/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GatewayResourceName : Name of the Gateway resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayResourceName {}

impl Default for GatewayResourceName {
    fn default() -> Self {
        Self::new()
    }
}

impl GatewayResourceName {
    /// Name of the Gateway resource.
    pub fn new() -> GatewayResourceName {
        GatewayResourceName {}
    }
}
