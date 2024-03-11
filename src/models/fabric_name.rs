/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FabricName : The Service Fabric name, including the 'fabric:' URI scheme.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricName {}

impl Default for FabricName {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricName {
    /// The Service Fabric name, including the 'fabric:' URI scheme.
    pub fn new() -> FabricName {
        FabricName {}
    }
}
