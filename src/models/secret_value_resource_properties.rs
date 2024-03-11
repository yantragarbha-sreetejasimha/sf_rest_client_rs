/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecretValueResourceProperties : This type describes properties of a secret value resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretValueResourceProperties {
    /// The actual value of the secret.
    #[serde(rename = "value")]
    value: Option<String>,
}

impl Default for SecretValueResourceProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretValueResourceProperties {
    /// This type describes properties of a secret value resource.
    pub fn new() -> SecretValueResourceProperties {
        SecretValueResourceProperties { value: None }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn with_value(
        mut self,
        value: String,
    ) -> SecretValueResourceProperties {
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
