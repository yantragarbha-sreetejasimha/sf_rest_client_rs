/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServiceTypeInfoList : List of information about service type deployed on a node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServiceTypeInfoList {}

impl Default for DeployedServiceTypeInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedServiceTypeInfoList {
    /// List of information about service type deployed on a node.
    pub fn new() -> DeployedServiceTypeInfoList {
        DeployedServiceTypeInfoList {}
    }
}
