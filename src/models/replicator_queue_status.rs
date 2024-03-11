/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicatorQueueStatus : Provides various statistics of the queue used in the service fabric replicator. Contains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc. Depending on the role of the replicator, the properties in this type imply different meanings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicatorQueueStatus {
    /// Represents the utilization of the queue. A value of 0 indicates that the queue is empty and a value of 100 indicates the queue is full.
    #[serde(rename = "QueueUtilizationPercentage")]
    queue_utilization_percentage: Option<i32>,
    /// Represents the virtual memory consumed by the queue in bytes.
    #[serde(rename = "QueueMemorySize")]
    queue_memory_size: Option<String>,
    /// On a primary replicator, this is semantically the sequence number of the operation for which all the secondary replicas have sent an acknowledgement. On a secondary replicator, this is the smallest sequence number of the operation that is present in the queue.
    #[serde(rename = "FirstSequenceNumber")]
    first_sequence_number: Option<String>,
    /// On a primary replicator, this is semantically the highest sequence number of the operation for which all the secondary replicas have sent an acknowledgement. On a secondary replicator, this is semantically the highest sequence number that has been applied to the persistent state.
    #[serde(rename = "CompletedSequenceNumber")]
    completed_sequence_number: Option<String>,
    /// On a primary replicator, this is semantically the highest sequence number of the operation for which a write quorum of the secondary replicas have sent an acknowledgement. On a secondary replicator, this is semantically the highest sequence number of the in-order operation received from the primary.
    #[serde(rename = "CommittedSequenceNumber")]
    committed_sequence_number: Option<String>,
    /// Represents the latest sequence number of the operation that is available in the queue.
    #[serde(rename = "LastSequenceNumber")]
    last_sequence_number: Option<String>,
}

impl Default for ReplicatorQueueStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplicatorQueueStatus {
    /// Provides various statistics of the queue used in the service fabric replicator. Contains information about the service fabric replicator like the replication/copy queue utilization, last acknowledgement received timestamp, etc. Depending on the role of the replicator, the properties in this type imply different meanings.
    pub fn new() -> ReplicatorQueueStatus {
        ReplicatorQueueStatus {
            queue_utilization_percentage: None,
            queue_memory_size: None,
            first_sequence_number: None,
            completed_sequence_number: None,
            committed_sequence_number: None,
            last_sequence_number: None,
        }
    }

    pub fn set_queue_utilization_percentage(
        &mut self,
        queue_utilization_percentage: i32,
    ) {
        self.queue_utilization_percentage = Some(queue_utilization_percentage);
    }

    pub fn with_queue_utilization_percentage(
        mut self,
        queue_utilization_percentage: i32,
    ) -> ReplicatorQueueStatus {
        self.queue_utilization_percentage = Some(queue_utilization_percentage);
        self
    }

    pub fn queue_utilization_percentage(&self) -> Option<&i32> {
        self.queue_utilization_percentage.as_ref()
    }

    pub fn reset_queue_utilization_percentage(&mut self) {
        self.queue_utilization_percentage = None;
    }

    pub fn set_queue_memory_size(&mut self, queue_memory_size: String) {
        self.queue_memory_size = Some(queue_memory_size);
    }

    pub fn with_queue_memory_size(
        mut self,
        queue_memory_size: String,
    ) -> ReplicatorQueueStatus {
        self.queue_memory_size = Some(queue_memory_size);
        self
    }

    pub fn queue_memory_size(&self) -> Option<&String> {
        self.queue_memory_size.as_ref()
    }

    pub fn reset_queue_memory_size(&mut self) {
        self.queue_memory_size = None;
    }

    pub fn set_first_sequence_number(&mut self, first_sequence_number: String) {
        self.first_sequence_number = Some(first_sequence_number);
    }

    pub fn with_first_sequence_number(
        mut self,
        first_sequence_number: String,
    ) -> ReplicatorQueueStatus {
        self.first_sequence_number = Some(first_sequence_number);
        self
    }

    pub fn first_sequence_number(&self) -> Option<&String> {
        self.first_sequence_number.as_ref()
    }

    pub fn reset_first_sequence_number(&mut self) {
        self.first_sequence_number = None;
    }

    pub fn set_completed_sequence_number(
        &mut self,
        completed_sequence_number: String,
    ) {
        self.completed_sequence_number = Some(completed_sequence_number);
    }

    pub fn with_completed_sequence_number(
        mut self,
        completed_sequence_number: String,
    ) -> ReplicatorQueueStatus {
        self.completed_sequence_number = Some(completed_sequence_number);
        self
    }

    pub fn completed_sequence_number(&self) -> Option<&String> {
        self.completed_sequence_number.as_ref()
    }

    pub fn reset_completed_sequence_number(&mut self) {
        self.completed_sequence_number = None;
    }

    pub fn set_committed_sequence_number(
        &mut self,
        committed_sequence_number: String,
    ) {
        self.committed_sequence_number = Some(committed_sequence_number);
    }

    pub fn with_committed_sequence_number(
        mut self,
        committed_sequence_number: String,
    ) -> ReplicatorQueueStatus {
        self.committed_sequence_number = Some(committed_sequence_number);
        self
    }

    pub fn committed_sequence_number(&self) -> Option<&String> {
        self.committed_sequence_number.as_ref()
    }

    pub fn reset_committed_sequence_number(&mut self) {
        self.committed_sequence_number = None;
    }

    pub fn set_last_sequence_number(&mut self, last_sequence_number: String) {
        self.last_sequence_number = Some(last_sequence_number);
    }

    pub fn with_last_sequence_number(
        mut self,
        last_sequence_number: String,
    ) -> ReplicatorQueueStatus {
        self.last_sequence_number = Some(last_sequence_number);
        self
    }

    pub fn last_sequence_number(&self) -> Option<&String> {
        self.last_sequence_number.as_ref()
    }

    pub fn reset_last_sequence_number(&mut self) {
        self.last_sequence_number = None;
    }
}
