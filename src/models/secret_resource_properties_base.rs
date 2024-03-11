/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecretResourcePropertiesBase : This type describes the properties of a secret resource, including its kind.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretResourcePropertiesBase {
    /// Describes the kind of secret.
    #[serde(rename = "kind")]
    kind: ::models::SecretKind,
}

impl SecretResourcePropertiesBase {
    /// This type describes the properties of a secret resource, including its kind.
    pub fn new(kind: ::models::SecretKind) -> SecretResourcePropertiesBase {
        SecretResourcePropertiesBase { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::SecretKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::SecretKind,
    ) -> SecretResourcePropertiesBase {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::SecretKind {
        &self.kind
    }
}
