/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// VolumeResourceDescription : Describes a service fabric volume resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeResourceDescription {
    /// This type describes properties of a volume resource.
    #[serde(rename = "properties")]
    properties: ::models::VolumeProperties,
    /// Volume resource name.
    #[serde(rename = "name")]
    name: ::models::VolumeResourceName,
}

impl VolumeResourceDescription {
    /// Describes a service fabric volume resource.
    pub fn new(
        properties: ::models::VolumeProperties,
        name: ::models::VolumeResourceName,
    ) -> VolumeResourceDescription {
        VolumeResourceDescription { properties, name }
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
}
