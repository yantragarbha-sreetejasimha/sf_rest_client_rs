/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FabricConfigVersionInfoList : List of all Service Fabric config versions.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricConfigVersionInfoList {}

impl Default for FabricConfigVersionInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricConfigVersionInfoList {
    /// List of all Service Fabric config versions.
    pub fn new() -> FabricConfigVersionInfoList {
        FabricConfigVersionInfoList {}
    }
}
