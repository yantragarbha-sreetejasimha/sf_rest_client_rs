/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServicePackageHealthStateList : List of health states for a service package deployed on a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServicePackageHealthStateList {}

impl Default for DeployedServicePackageHealthStateList {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedServicePackageHealthStateList {
    /// List of health states for a service package deployed on a Service Fabric node.
    pub fn new() -> DeployedServicePackageHealthStateList {
        DeployedServicePackageHealthStateList {}
    }
}
