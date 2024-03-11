/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PackageSharingPolicyInfoList : List of package sharing policy information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSharingPolicyInfoList {}

impl Default for PackageSharingPolicyInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageSharingPolicyInfoList {
    /// List of package sharing policy information.
    pub fn new() -> PackageSharingPolicyInfoList {
        PackageSharingPolicyInfoList {}
    }
}
