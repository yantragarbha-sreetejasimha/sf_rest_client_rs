/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FabricCodeVersionInfoList : List of all Service Fabric code versions.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricCodeVersionInfoList {}

impl Default for FabricCodeVersionInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl FabricCodeVersionInfoList {
    /// List of all Service Fabric code versions.
    pub fn new() -> FabricCodeVersionInfoList {
        FabricCodeVersionInfoList {}
    }
}
