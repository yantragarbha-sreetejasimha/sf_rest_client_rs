/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ProvisionApplicationTypeKind : The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionApplicationTypeKind {}

impl Default for ProvisionApplicationTypeKind {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvisionApplicationTypeKind {
    /// The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision.
    pub fn new() -> ProvisionApplicationTypeKind {
        ProvisionApplicationTypeKind {}
    }
}

// TODO enum
// List of ProvisionApplicationTypeKind
//const (
//
//
//
//)
