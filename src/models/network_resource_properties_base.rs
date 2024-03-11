/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NetworkResourcePropertiesBase : This type describes the properties of a network resource, including its kind.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkResourcePropertiesBase {
    /// The type of a Service Fabric container network.
    #[serde(rename = "kind")]
    kind: ::models::NetworkKind,
}

impl NetworkResourcePropertiesBase {
    /// This type describes the properties of a network resource, including its kind.
    pub fn new(kind: ::models::NetworkKind) -> NetworkResourcePropertiesBase {
        NetworkResourcePropertiesBase { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::NetworkKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::NetworkKind,
    ) -> NetworkResourcePropertiesBase {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::NetworkKind {
        &self.kind
    }
}
