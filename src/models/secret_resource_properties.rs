/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecretResourceProperties : Describes the properties of a secret resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretResourceProperties {
    /// Describes the kind of secret.
    #[serde(rename = "kind")]
    kind: ::models::SecretKind,
    /// User readable description of the secret.
    #[serde(rename = "description")]
    description: Option<String>,
    /// Status of the resource.
    #[serde(rename = "status")]
    status: Option<::models::ResourceStatus>,
    /// Gives additional information about the current status of the secret.
    #[serde(rename = "statusDetails")]
    status_details: Option<String>,
    /// The type of the content stored in the secret value. The value of this property is opaque to Service Fabric. Once set, the value of this property cannot be changed.
    #[serde(rename = "contentType")]
    content_type: Option<String>,
}

impl SecretResourceProperties {
    /// Describes the properties of a secret resource.
    pub fn new(kind: ::models::SecretKind) -> SecretResourceProperties {
        SecretResourceProperties {
            kind,
            description: None,
            status: None,
            status_details: None,
            content_type: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::SecretKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::SecretKind,
    ) -> SecretResourceProperties {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::SecretKind {
        &self.kind
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> SecretResourceProperties {
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
    ) -> SecretResourceProperties {
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
    ) -> SecretResourceProperties {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type);
    }

    pub fn with_content_type(
        mut self,
        content_type: String,
    ) -> SecretResourceProperties {
        self.content_type = Some(content_type);
        self
    }

    pub fn content_type(&self) -> Option<&String> {
        self.content_type.as_ref()
    }

    pub fn reset_content_type(&mut self) {
        self.content_type = None;
    }
}
