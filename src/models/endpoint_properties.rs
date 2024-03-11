/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EndpointProperties : Describes a container endpoint.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointProperties {
    /// The name of the endpoint.
    #[serde(rename = "name")]
    name: String,
    /// Port used by the container.
    #[serde(rename = "port")]
    port: Option<i32>,
}

impl EndpointProperties {
    /// Describes a container endpoint.
    pub fn new(name: String) -> EndpointProperties {
        EndpointProperties { name, port: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> EndpointProperties {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_port(&mut self, port: i32) {
        self.port = Some(port);
    }

    pub fn with_port(mut self, port: i32) -> EndpointProperties {
        self.port = Some(port);
        self
    }

    pub fn port(&self) -> Option<&i32> {
        self.port.as_ref()
    }

    pub fn reset_port(&mut self) {
        self.port = None;
    }
}
