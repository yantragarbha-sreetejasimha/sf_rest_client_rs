/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServicePackageInfoList : List of deployed service package information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServicePackageInfoList {}

impl Default for DeployedServicePackageInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedServicePackageInfoList {
    /// List of deployed service package information.
    pub fn new() -> DeployedServicePackageInfoList {
        DeployedServicePackageInfoList {}
    }
}
