/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PackageSharingPolicyScope : Represents the scope for PackageSharingPolicy. This is specified during DeployServicePackageToNode operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSharingPolicyScope {}

impl Default for PackageSharingPolicyScope {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageSharingPolicyScope {
    /// Represents the scope for PackageSharingPolicy. This is specified during DeployServicePackageToNode operation.
    pub fn new() -> PackageSharingPolicyScope {
        PackageSharingPolicyScope {}
    }
}

// TODO enum
// List of PackageSharingPolicyScope
//const (
//
//
//
//)
