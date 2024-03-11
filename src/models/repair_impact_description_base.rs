/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairImpactDescriptionBase : Describes the expected impact of executing a repair task.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairImpactDescriptionBase {
    /// The kind of repair impact represented by the current object.
    #[serde(rename = "Kind")]
    kind: ::models::RepairImpactKind,
}

impl RepairImpactDescriptionBase {
    /// Describes the expected impact of executing a repair task.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.
    pub fn new(
        kind: ::models::RepairImpactKind,
    ) -> RepairImpactDescriptionBase {
        RepairImpactDescriptionBase { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::RepairImpactKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::RepairImpactKind,
    ) -> RepairImpactDescriptionBase {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::RepairImpactKind {
        &self.kind
    }
}
