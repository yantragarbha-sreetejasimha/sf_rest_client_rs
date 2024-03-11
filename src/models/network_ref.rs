/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NetworkRef : Describes a network reference in a service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRef {
    /// Name of the network
    #[serde(rename = "name")]
    name: Option<String>,
    /// A list of endpoints that are exposed on this network.
    #[serde(rename = "endpointRefs")]
    endpoint_refs: Option<Vec<::models::EndpointRef>>,
}

impl Default for NetworkRef {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkRef {
    /// Describes a network reference in a service.
    pub fn new() -> NetworkRef {
        NetworkRef {
            name: None,
            endpoint_refs: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> NetworkRef {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_endpoint_refs(
        &mut self,
        endpoint_refs: Vec<::models::EndpointRef>,
    ) {
        self.endpoint_refs = Some(endpoint_refs);
    }

    pub fn with_endpoint_refs(
        mut self,
        endpoint_refs: Vec<::models::EndpointRef>,
    ) -> NetworkRef {
        self.endpoint_refs = Some(endpoint_refs);
        self
    }

    pub fn endpoint_refs(&self) -> Option<&Vec<::models::EndpointRef>> {
        self.endpoint_refs.as_ref()
    }

    pub fn reset_endpoint_refs(&mut self) {
        self.endpoint_refs = None;
    }
}
