/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RemoteReplicatorAcknowledgementStatus : Provides details about the remote replicators from the primary replicator's point of view.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteReplicatorAcknowledgementStatus {
    /// Details about the acknowledgements for operations that are part of the replication stream data.
    #[serde(rename = "ReplicationStreamAcknowledgementDetail")]
    replication_stream_acknowledgement_detail:
        Option<::models::RemoteReplicatorAcknowledgementDetail>,
    /// Details about the acknowledgements for operations that are part of the copy stream data.
    #[serde(rename = "CopyStreamAcknowledgementDetail")]
    copy_stream_acknowledgement_detail:
        Option<::models::RemoteReplicatorAcknowledgementDetail>,
}

impl Default for RemoteReplicatorAcknowledgementStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteReplicatorAcknowledgementStatus {
    /// Provides details about the remote replicators from the primary replicator's point of view.
    pub fn new() -> RemoteReplicatorAcknowledgementStatus {
        RemoteReplicatorAcknowledgementStatus {
            replication_stream_acknowledgement_detail: None,
            copy_stream_acknowledgement_detail: None,
        }
    }

    pub fn set_replication_stream_acknowledgement_detail(
        &mut self,
        replication_stream_acknowledgement_detail: ::models::RemoteReplicatorAcknowledgementDetail,
    ) {
        self.replication_stream_acknowledgement_detail =
            Some(replication_stream_acknowledgement_detail);
    }

    pub fn with_replication_stream_acknowledgement_detail(
        mut self,
        replication_stream_acknowledgement_detail: ::models::RemoteReplicatorAcknowledgementDetail,
    ) -> RemoteReplicatorAcknowledgementStatus {
        self.replication_stream_acknowledgement_detail =
            Some(replication_stream_acknowledgement_detail);
        self
    }

    pub fn replication_stream_acknowledgement_detail(
        &self,
    ) -> Option<&::models::RemoteReplicatorAcknowledgementDetail> {
        self.replication_stream_acknowledgement_detail.as_ref()
    }

    pub fn reset_replication_stream_acknowledgement_detail(&mut self) {
        self.replication_stream_acknowledgement_detail = None;
    }

    pub fn set_copy_stream_acknowledgement_detail(
        &mut self,
        copy_stream_acknowledgement_detail: ::models::RemoteReplicatorAcknowledgementDetail,
    ) {
        self.copy_stream_acknowledgement_detail =
            Some(copy_stream_acknowledgement_detail);
    }

    pub fn with_copy_stream_acknowledgement_detail(
        mut self,
        copy_stream_acknowledgement_detail: ::models::RemoteReplicatorAcknowledgementDetail,
    ) -> RemoteReplicatorAcknowledgementStatus {
        self.copy_stream_acknowledgement_detail =
            Some(copy_stream_acknowledgement_detail);
        self
    }

    pub fn copy_stream_acknowledgement_detail(
        &self,
    ) -> Option<&::models::RemoteReplicatorAcknowledgementDetail> {
        self.copy_stream_acknowledgement_detail.as_ref()
    }

    pub fn reset_copy_stream_acknowledgement_detail(&mut self) {
        self.copy_stream_acknowledgement_detail = None;
    }
}