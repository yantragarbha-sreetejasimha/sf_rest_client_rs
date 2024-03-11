/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeExtensionDescription : Describes extension of a service type defined in the service manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeExtensionDescription {
    /// The name of the extension.
    #[serde(rename = "Key")]
    key: Option<String>,
    /// The extension value.
    #[serde(rename = "Value")]
    value: Option<String>,
}

impl Default for ServiceTypeExtensionDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeExtensionDescription {
    /// Describes extension of a service type defined in the service manifest.
    pub fn new() -> ServiceTypeExtensionDescription {
        ServiceTypeExtensionDescription {
            key: None,
            value: None,
        }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }

    pub fn with_key(mut self, key: String) -> ServiceTypeExtensionDescription {
        self.key = Some(key);
        self
    }

    pub fn key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    pub fn reset_key(&mut self) {
        self.key = None;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn with_value(
        mut self,
        value: String,
    ) -> ServiceTypeExtensionDescription {
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