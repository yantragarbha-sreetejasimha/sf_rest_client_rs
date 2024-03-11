/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecretValueProperties : This type describes properties of secret value resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretValueProperties {
    /// The actual value of the secret.
    #[serde(rename = "value")]
    value: Option<String>,
}

impl Default for SecretValueProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretValueProperties {
    /// This type describes properties of secret value resource.
    pub fn new() -> SecretValueProperties {
        SecretValueProperties { value: None }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn with_value(mut self, value: String) -> SecretValueProperties {
        self.value = Some(value);
        self
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn reset_value(&mut self) {
        self.value = None;
    }
}