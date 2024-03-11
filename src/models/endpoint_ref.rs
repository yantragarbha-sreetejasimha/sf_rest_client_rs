/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EndpointRef : Describes a reference to a service endpoint.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointRef {
    /// Name of the endpoint.
    #[serde(rename = "name")]
    name: Option<String>,
}

impl Default for EndpointRef {
    fn default() -> Self {
        Self::new()
    }
}

impl EndpointRef {
    /// Describes a reference to a service endpoint.
    pub fn new() -> EndpointRef {
        EndpointRef { name: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> EndpointRef {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }
}