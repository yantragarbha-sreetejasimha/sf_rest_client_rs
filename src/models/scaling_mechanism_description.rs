/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScalingMechanismDescription : Describes the mechanism for performing a scaling operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalingMechanismDescription {
    /// Specifies the kind of scaling mechanism
    #[serde(rename = "Kind")]
    kind: ::models::ScalingMechanismKind,
}

impl ScalingMechanismDescription {
    /// Describes the mechanism for performing a scaling operation.
    pub fn new(
        kind: ::models::ScalingMechanismKind,
    ) -> ScalingMechanismDescription {
        ScalingMechanismDescription { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::ScalingMechanismKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ScalingMechanismKind,
    ) -> ScalingMechanismDescription {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ScalingMechanismKind {
        &self.kind
    }
}
