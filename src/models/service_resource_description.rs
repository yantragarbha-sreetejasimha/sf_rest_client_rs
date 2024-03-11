/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceResourceDescription : This type describes a service resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceResourceDescription {
    /// Name of the Service resource.
    #[serde(rename = "name")]
    name: ::models::ServiceResourceName,
    /// This type describes properties of a service resource.
    #[serde(rename = "properties")]
    properties: ::models::ServiceResourceProperties,
}

impl ServiceResourceDescription {
    /// This type describes a service resource.
    pub fn new(
        name: ::models::ServiceResourceName,
        properties: ::models::ServiceResourceProperties,
    ) -> ServiceResourceDescription {
        ServiceResourceDescription { name, properties }
    }

    pub fn set_name(&mut self, name: ::models::ServiceResourceName) {
        self.name = name;
    }

    pub fn with_name(
        mut self,
        name: ::models::ServiceResourceName,
    ) -> ServiceResourceDescription {
        self.name = name;
        self
    }

    pub fn name(&self) -> &::models::ServiceResourceName {
        &self.name
    }

    pub fn set_properties(
        &mut self,
        properties: ::models::ServiceResourceProperties,
    ) {
        self.properties = properties;
    }

    pub fn with_properties(
        mut self,
        properties: ::models::ServiceResourceProperties,
    ) -> ServiceResourceDescription {
        self.properties = properties;
        self
    }

    pub fn properties(&self) -> &::models::ServiceResourceProperties {
        &self.properties
    }
}
