/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// VolumeResourceDescription : This type describes a volume resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeResourceDescription {
    /// Name of the Volume resource.
    #[serde(rename = "name")]
    name: ::models::VolumeResourceName,
    /// Describes properties of a volume resource.
    #[serde(rename = "properties")]
    properties: ::models::VolumeProperties,
}

impl VolumeResourceDescription {
    /// This type describes a volume resource.
    pub fn new(
        name: ::models::VolumeResourceName,
        properties: ::models::VolumeProperties,
    ) -> VolumeResourceDescription {
        VolumeResourceDescription { name, properties }
    }

    pub fn set_name(&mut self, name: ::models::VolumeResourceName) {
        self.name = name;
    }

    pub fn with_name(
        mut self,
        name: ::models::VolumeResourceName,
    ) -> VolumeResourceDescription {
        self.name = name;
        self
    }

    pub fn name(&self) -> &::models::VolumeResourceName {
        &self.name
    }

    pub fn set_properties(&mut self, properties: ::models::VolumeProperties) {
        self.properties = properties;
    }

    pub fn with_properties(
        mut self,
        properties: ::models::VolumeProperties,
    ) -> VolumeResourceDescription {
        self.properties = properties;
        self
    }

    pub fn properties(&self) -> &::models::VolumeProperties {
        &self.properties
    }
}
