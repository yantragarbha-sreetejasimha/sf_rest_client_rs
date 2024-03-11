/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerLabel : Describes a container label.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerLabel {
    /// The name of the container label.
    #[serde(rename = "name")]
    name: String,
    /// The value of the container label.
    #[serde(rename = "value")]
    value: String,
}

impl ContainerLabel {
    /// Describes a container label.
    pub fn new(name: String, value: String) -> ContainerLabel {
        ContainerLabel { name, value }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> ContainerLabel {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn with_value(mut self, value: String) -> ContainerLabel {
        self.value = value;
        self
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
