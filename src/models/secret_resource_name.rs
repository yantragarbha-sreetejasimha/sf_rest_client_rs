/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecretResourceName : Name of the Secret resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretResourceName {}

impl Default for SecretResourceName {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretResourceName {
    /// Name of the Secret resource.
    pub fn new() -> SecretResourceName {
        SecretResourceName {}
    }
}
