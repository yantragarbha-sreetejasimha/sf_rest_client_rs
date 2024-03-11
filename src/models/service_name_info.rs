/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceNameInfo : Information about the service name.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceNameInfo {
    /// The identity of the service. This ID is an encoded representation of the service name. This is used in the REST APIs to identify the service resource. Starting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the service name is \"fabric:/myapp/app1/svc1\", the service identity would be \"myapp~app1\\~svc1\" in 6.0+ and \"myapp/app1/svc1\" in previous versions.
    #[serde(rename = "Id")]
    id: Option<::models::ServiceId>,
    /// The full name of the service with 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: Option<::models::ServiceName>,
}

impl Default for ServiceNameInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceNameInfo {
    /// Information about the service name.
    pub fn new() -> ServiceNameInfo {
        ServiceNameInfo {
            id: None,
            name: None,
        }
    }

    pub fn set_id(&mut self, id: ::models::ServiceId) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: ::models::ServiceId) -> ServiceNameInfo {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&::models::ServiceId> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_name(&mut self, name: ::models::ServiceName) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: ::models::ServiceName) -> ServiceNameInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::ServiceName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }
}
