/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SafetyCheck : Represents a safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyCheck {
    /// The kind of safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state. Following are the kinds of safety checks.
    #[serde(rename = "Kind")]
    kind: ::models::SafetyCheckKind,
}

impl SafetyCheck {
    /// Represents a safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state.
    pub fn new(kind: ::models::SafetyCheckKind) -> SafetyCheck {
        SafetyCheck { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::SafetyCheckKind) {
        self.kind = kind;
    }

    pub fn with_kind(mut self, kind: ::models::SafetyCheckKind) -> SafetyCheck {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::SafetyCheckKind {
        &self.kind
    }
}
