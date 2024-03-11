/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicatorStatus : Represents a base class for primary or secondary replicator status. Contains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicatorStatus {
    /// The role of a replica of a stateful service.
    #[serde(rename = "Kind")]
    kind: ::models::ReplicaRole,
}

impl ReplicatorStatus {
    /// Represents a base class for primary or secondary replicator status. Contains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc.
    pub fn new(kind: ::models::ReplicaRole) -> ReplicatorStatus {
        ReplicatorStatus { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::ReplicaRole) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ReplicaRole,
    ) -> ReplicatorStatus {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ReplicaRole {
        &self.kind
    }
}