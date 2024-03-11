/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicaStatusBase : Information about the replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaStatusBase {
    /// The role of a replica of a stateful service.
    #[serde(rename = "Kind")]
    kind: ::models::ReplicaKind,
}

impl ReplicaStatusBase {
    /// Information about the replica.
    pub fn new(kind: ::models::ReplicaKind) -> ReplicaStatusBase {
        ReplicaStatusBase { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::ReplicaKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ReplicaKind,
    ) -> ReplicaStatusBase {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ReplicaKind {
        &self.kind
    }
}