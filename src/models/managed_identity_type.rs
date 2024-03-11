/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ManagedIdentityType : The type of managed identity to be used to connect to Azure Blob Store via Managed Identity.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedIdentityType {}

impl Default for ManagedIdentityType {
    fn default() -> Self {
        Self::new()
    }
}

impl ManagedIdentityType {
    /// The type of managed identity to be used to connect to Azure Blob Store via Managed Identity.
    pub fn new() -> ManagedIdentityType {
        ManagedIdentityType {}
    }
}

// TODO enum
// List of ManagedIdentityType
//const (
//
//
//
//)
