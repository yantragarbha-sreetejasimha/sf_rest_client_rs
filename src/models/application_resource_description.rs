/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationResourceDescription : This type describes a application resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationResourceDescription {
    /// Name of the Application resource.
    #[serde(rename = "name")]
    name: ::models::ApplicationResourceName,
    /// Describes properties of a application resource.
    #[serde(rename = "properties")]
    properties: ::models::ApplicationProperties,
    /// Describes the identity of the application.
    #[serde(rename = "identity")]
    identity: Option<::models::IdentityDescription>,
}

impl ApplicationResourceDescription {
    /// This type describes a application resource.
    pub fn new(
        name: ::models::ApplicationResourceName,
        properties: ::models::ApplicationProperties,
    ) -> ApplicationResourceDescription {
        ApplicationResourceDescription {
            name,
            properties,
            identity: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::ApplicationResourceName) {
        self.name = name;
    }

    pub fn with_name(
        mut self,
        name: ::models::ApplicationResourceName,
    ) -> ApplicationResourceDescription {
        self.name = name;
        self
    }

    pub fn name(&self) -> &::models::ApplicationResourceName {
        &self.name
    }

    pub fn set_properties(
        &mut self,
        properties: ::models::ApplicationProperties,
    ) {
        self.properties = properties;
    }

    pub fn with_properties(
        mut self,
        properties: ::models::ApplicationProperties,
    ) -> ApplicationResourceDescription {
        self.properties = properties;
        self
    }

    pub fn properties(&self) -> &::models::ApplicationProperties {
        &self.properties
    }

    pub fn set_identity(&mut self, identity: ::models::IdentityDescription) {
        self.identity = Some(identity);
    }

    pub fn with_identity(
        mut self,
        identity: ::models::IdentityDescription,
    ) -> ApplicationResourceDescription {
        self.identity = Some(identity);
        self
    }

    pub fn identity(&self) -> Option<&::models::IdentityDescription> {
        self.identity.as_ref()
    }

    pub fn reset_identity(&mut self) {
        self.identity = None;
    }
}
