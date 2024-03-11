/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServicePlacementPolicyType : The type of placement policy for a service fabric service. Following are the possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicePlacementPolicyType {}

impl Default for ServicePlacementPolicyType {
    fn default() -> Self {
        Self::new()
    }
}

impl ServicePlacementPolicyType {
    /// The type of placement policy for a service fabric service. Following are the possible values.
    pub fn new() -> ServicePlacementPolicyType {
        ServicePlacementPolicyType {}
    }
}

// TODO enum
// List of ServicePlacementPolicyType
//const (
//
//
//
//)
