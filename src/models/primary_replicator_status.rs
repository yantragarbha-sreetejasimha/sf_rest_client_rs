/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PrimaryReplicatorStatus : Provides statistics about the Service Fabric Replicator, when it is functioning in a Primary role.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryReplicatorStatus {
    /// The role of a replica of a stateful service.
    #[serde(rename = "Kind")]
    kind: ::models::ReplicaRole,
    /// Details about the replication queue on the primary replicator.
    #[serde(rename = "ReplicationQueueStatus")]
    replication_queue_status: Option<::models::ReplicatorQueueStatus>,
    /// The status of all the active and idle secondary replicators that the primary is aware of.
    #[serde(rename = "RemoteReplicators")]
    remote_replicators: Option<::models::RemoteReplicatorStatusList>,
}

impl PrimaryReplicatorStatus {
    /// Provides statistics about the Service Fabric Replicator, when it is functioning in a Primary role.
    pub fn new(kind: ::models::ReplicaRole) -> PrimaryReplicatorStatus {
        PrimaryReplicatorStatus {
            kind,
            replication_queue_status: None,
            remote_replicators: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ReplicaRole) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ReplicaRole,
    ) -> PrimaryReplicatorStatus {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ReplicaRole {
        &self.kind
    }

    pub fn set_replication_queue_status(
        &mut self,
        replication_queue_status: ::models::ReplicatorQueueStatus,
    ) {
        self.replication_queue_status = Some(replication_queue_status);
    }

    pub fn with_replication_queue_status(
        mut self,
        replication_queue_status: ::models::ReplicatorQueueStatus,
    ) -> PrimaryReplicatorStatus {
        self.replication_queue_status = Some(replication_queue_status);
        self
    }

    pub fn replication_queue_status(
        &self,
    ) -> Option<&::models::ReplicatorQueueStatus> {
        self.replication_queue_status.as_ref()
    }

    pub fn reset_replication_queue_status(&mut self) {
        self.replication_queue_status = None;
    }

    pub fn set_remote_replicators(
        &mut self,
        remote_replicators: ::models::RemoteReplicatorStatusList,
    ) {
        self.remote_replicators = Some(remote_replicators);
    }

    pub fn with_remote_replicators(
        mut self,
        remote_replicators: ::models::RemoteReplicatorStatusList,
    ) -> PrimaryReplicatorStatus {
        self.remote_replicators = Some(remote_replicators);
        self
    }

    pub fn remote_replicators(
        &self,
    ) -> Option<&::models::RemoteReplicatorStatusList> {
        self.remote_replicators.as_ref()
    }

    pub fn reset_remote_replicators(&mut self) {
        self.remote_replicators = None;
    }
}
