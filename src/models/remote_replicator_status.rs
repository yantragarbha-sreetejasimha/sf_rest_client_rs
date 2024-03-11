/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RemoteReplicatorStatus : Represents the state of the secondary replicator from the primary replicator’s point of view.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteReplicatorStatus {
    /// Represents the replica ID of the remote secondary replicator.
    #[serde(rename = "ReplicaId")]
    replica_id: Option<::models::ReplicaId>,
    /// The last timestamp (in UTC) when an acknowledgement from the secondary replicator was processed on the primary. UTC 0 represents an invalid value, indicating that no acknowledgement messages were ever processed.
    #[serde(rename = "LastAcknowledgementProcessedTimeUtc")]
    last_acknowledgement_processed_time_utc: Option<String>,
    /// The highest replication operation sequence number that the secondary has received from the primary.
    #[serde(rename = "LastReceivedReplicationSequenceNumber")]
    last_received_replication_sequence_number: Option<String>,
    /// The highest replication operation sequence number that the secondary has applied to its state.
    #[serde(rename = "LastAppliedReplicationSequenceNumber")]
    last_applied_replication_sequence_number: Option<String>,
    /// A value that indicates whether the secondary replica is in the process of being built.
    #[serde(rename = "IsInBuild")]
    is_in_build: Option<bool>,
    /// The highest copy operation sequence number that the secondary has received from the primary. A value of -1 implies that the secondary has received all copy operations.
    #[serde(rename = "LastReceivedCopySequenceNumber")]
    last_received_copy_sequence_number: Option<String>,
    /// The highest copy operation sequence number that the secondary has applied to its state. A value of -1 implies that the secondary has applied all copy operations and the copy process is complete.
    #[serde(rename = "LastAppliedCopySequenceNumber")]
    last_applied_copy_sequence_number: Option<String>,
    /// Represents the acknowledgment status for the remote secondary replicator.
    #[serde(rename = "RemoteReplicatorAcknowledgementStatus")]
    remote_replicator_acknowledgement_status:
        Option<::models::RemoteReplicatorAcknowledgementStatus>,
}

impl Default for RemoteReplicatorStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteReplicatorStatus {
    /// Represents the state of the secondary replicator from the primary replicator’s point of view.
    pub fn new() -> RemoteReplicatorStatus {
        RemoteReplicatorStatus {
            replica_id: None,
            last_acknowledgement_processed_time_utc: None,
            last_received_replication_sequence_number: None,
            last_applied_replication_sequence_number: None,
            is_in_build: None,
            last_received_copy_sequence_number: None,
            last_applied_copy_sequence_number: None,
            remote_replicator_acknowledgement_status: None,
        }
    }

    pub fn set_replica_id(&mut self, replica_id: ::models::ReplicaId) {
        self.replica_id = Some(replica_id);
    }

    pub fn with_replica_id(
        mut self,
        replica_id: ::models::ReplicaId,
    ) -> RemoteReplicatorStatus {
        self.replica_id = Some(replica_id);
        self
    }

    pub fn replica_id(&self) -> Option<&::models::ReplicaId> {
        self.replica_id.as_ref()
    }

    pub fn reset_replica_id(&mut self) {
        self.replica_id = None;
    }

    pub fn set_last_acknowledgement_processed_time_utc(
        &mut self,
        last_acknowledgement_processed_time_utc: String,
    ) {
        self.last_acknowledgement_processed_time_utc =
            Some(last_acknowledgement_processed_time_utc);
    }

    pub fn with_last_acknowledgement_processed_time_utc(
        mut self,
        last_acknowledgement_processed_time_utc: String,
    ) -> RemoteReplicatorStatus {
        self.last_acknowledgement_processed_time_utc =
            Some(last_acknowledgement_processed_time_utc);
        self
    }

    pub fn last_acknowledgement_processed_time_utc(&self) -> Option<&String> {
        self.last_acknowledgement_processed_time_utc.as_ref()
    }

    pub fn reset_last_acknowledgement_processed_time_utc(&mut self) {
        self.last_acknowledgement_processed_time_utc = None;
    }

    pub fn set_last_received_replication_sequence_number(
        &mut self,
        last_received_replication_sequence_number: String,
    ) {
        self.last_received_replication_sequence_number =
            Some(last_received_replication_sequence_number);
    }

    pub fn with_last_received_replication_sequence_number(
        mut self,
        last_received_replication_sequence_number: String,
    ) -> RemoteReplicatorStatus {
        self.last_received_replication_sequence_number =
            Some(last_received_replication_sequence_number);
        self
    }

    pub fn last_received_replication_sequence_number(&self) -> Option<&String> {
        self.last_received_replication_sequence_number.as_ref()
    }

    pub fn reset_last_received_replication_sequence_number(&mut self) {
        self.last_received_replication_sequence_number = None;
    }

    pub fn set_last_applied_replication_sequence_number(
        &mut self,
        last_applied_replication_sequence_number: String,
    ) {
        self.last_applied_replication_sequence_number =
            Some(last_applied_replication_sequence_number);
    }

    pub fn with_last_applied_replication_sequence_number(
        mut self,
        last_applied_replication_sequence_number: String,
    ) -> RemoteReplicatorStatus {
        self.last_applied_replication_sequence_number =
            Some(last_applied_replication_sequence_number);
        self
    }

    pub fn last_applied_replication_sequence_number(&self) -> Option<&String> {
        self.last_applied_replication_sequence_number.as_ref()
    }

    pub fn reset_last_applied_replication_sequence_number(&mut self) {
        self.last_applied_replication_sequence_number = None;
    }

    pub fn set_is_in_build(&mut self, is_in_build: bool) {
        self.is_in_build = Some(is_in_build);
    }

    pub fn with_is_in_build(
        mut self,
        is_in_build: bool,
    ) -> RemoteReplicatorStatus {
        self.is_in_build = Some(is_in_build);
        self
    }

    pub fn is_in_build(&self) -> Option<&bool> {
        self.is_in_build.as_ref()
    }

    pub fn reset_is_in_build(&mut self) {
        self.is_in_build = None;
    }

    pub fn set_last_received_copy_sequence_number(
        &mut self,
        last_received_copy_sequence_number: String,
    ) {
        self.last_received_copy_sequence_number =
            Some(last_received_copy_sequence_number);
    }

    pub fn with_last_received_copy_sequence_number(
        mut self,
        last_received_copy_sequence_number: String,
    ) -> RemoteReplicatorStatus {
        self.last_received_copy_sequence_number =
            Some(last_received_copy_sequence_number);
        self
    }

    pub fn last_received_copy_sequence_number(&self) -> Option<&String> {
        self.last_received_copy_sequence_number.as_ref()
    }

    pub fn reset_last_received_copy_sequence_number(&mut self) {
        self.last_received_copy_sequence_number = None;
    }

    pub fn set_last_applied_copy_sequence_number(
        &mut self,
        last_applied_copy_sequence_number: String,
    ) {
        self.last_applied_copy_sequence_number =
            Some(last_applied_copy_sequence_number);
    }

    pub fn with_last_applied_copy_sequence_number(
        mut self,
        last_applied_copy_sequence_number: String,
    ) -> RemoteReplicatorStatus {
        self.last_applied_copy_sequence_number =
            Some(last_applied_copy_sequence_number);
        self
    }

    pub fn last_applied_copy_sequence_number(&self) -> Option<&String> {
        self.last_applied_copy_sequence_number.as_ref()
    }

    pub fn reset_last_applied_copy_sequence_number(&mut self) {
        self.last_applied_copy_sequence_number = None;
    }

    pub fn set_remote_replicator_acknowledgement_status(
        &mut self,
        remote_replicator_acknowledgement_status: ::models::RemoteReplicatorAcknowledgementStatus,
    ) {
        self.remote_replicator_acknowledgement_status =
            Some(remote_replicator_acknowledgement_status);
    }

    pub fn with_remote_replicator_acknowledgement_status(
        mut self,
        remote_replicator_acknowledgement_status: ::models::RemoteReplicatorAcknowledgementStatus,
    ) -> RemoteReplicatorStatus {
        self.remote_replicator_acknowledgement_status =
            Some(remote_replicator_acknowledgement_status);
        self
    }

    pub fn remote_replicator_acknowledgement_status(
        &self,
    ) -> Option<&::models::RemoteReplicatorAcknowledgementStatus> {
        self.remote_replicator_acknowledgement_status.as_ref()
    }

    pub fn reset_remote_replicator_acknowledgement_status(&mut self) {
        self.remote_replicator_acknowledgement_status = None;
    }
}
