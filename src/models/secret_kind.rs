/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecretKind : Describes the kind of secret.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretKind {}

impl Default for SecretKind {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretKind {
    /// Describes the kind of secret.
    pub fn new() -> SecretKind {
        SecretKind {}
    }
}

// TODO enum
// List of SecretKind
//const (
//
//
//
//)
