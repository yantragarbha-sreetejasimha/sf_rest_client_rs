/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NetworkResourceProperties : Describes properties of a network resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkResourceProperties {
    /// The type of a Service Fabric container network.
    #[serde(rename = "kind")]
    kind: ::models::NetworkKind,
    /// User readable description of the network.
    #[serde(rename = "description")]
    description: Option<String>,
    /// Status of the network.
    #[serde(rename = "status")]
    status: Option<::models::ResourceStatus>,
    /// Gives additional information about the current status of the network.
    #[serde(rename = "statusDetails")]
    status_details: Option<String>,
}

impl NetworkResourceProperties {
    /// Describes properties of a network resource.
    pub fn new(kind: ::models::NetworkKind) -> NetworkResourceProperties {
        NetworkResourceProperties {
            kind,
            description: None,
            status: None,
            status_details: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::NetworkKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::NetworkKind,
    ) -> NetworkResourceProperties {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::NetworkKind {
        &self.kind
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> NetworkResourceProperties {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_status(&mut self, status: ::models::ResourceStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ResourceStatus,
    ) -> NetworkResourceProperties {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::ResourceStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_status_details(&mut self, status_details: String) {
        self.status_details = Some(status_details);
    }

    pub fn with_status_details(
        mut self,
        status_details: String,
    ) -> NetworkResourceProperties {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }
}