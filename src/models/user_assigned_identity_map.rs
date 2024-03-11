/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UserAssignedIdentityMap : Defines a map that contains user assigned identities.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignedIdentityMap {}

impl Default for UserAssignedIdentityMap {
    fn default() -> Self {
        Self::new()
    }
}

impl UserAssignedIdentityMap {
    /// Defines a map that contains user assigned identities.
    pub fn new() -> UserAssignedIdentityMap {
        UserAssignedIdentityMap {}
    }
}
