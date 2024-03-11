/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationParameter : Describes an application parameter override to be applied when creating or upgrading an application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationParameter {
    /// The name of the parameter.
    #[serde(rename = "Key")]
    key: String,
    /// The value of the parameter.
    #[serde(rename = "Value")]
    value: String,
}

impl ApplicationParameter {
    /// Describes an application parameter override to be applied when creating or upgrading an application.
    pub fn new(key: String, value: String) -> ApplicationParameter {
        ApplicationParameter { key, value }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn with_key(mut self, key: String) -> ApplicationParameter {
        self.key = key;
        self
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn with_value(mut self, value: String) -> ApplicationParameter {
        self.value = value;
        self
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
