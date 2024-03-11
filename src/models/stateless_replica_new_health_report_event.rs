/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatelessReplicaNewHealthReportEvent : Stateless Replica Health Report Created event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatelessReplicaNewHealthReportEvent {
    /// An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id.
    #[serde(rename = "ReplicaId")]
    replica_id: Option<::models::ReplicaIdInteger>,
    /// The kind of FabricEvent.
    #[serde(rename = "Kind")]
    kind: ::models::FabricEventKind,
    /// The identifier for the FabricEvent instance.
    #[serde(rename = "EventInstanceId")]
    event_instance_id: String,
    /// The category of event.
    #[serde(rename = "Category")]
    category: Option<String>,
    /// The time event was logged.
    #[serde(rename = "TimeStamp")]
    time_stamp: String,
    /// Shows there is existing related events available.
    #[serde(rename = "HasCorrelatedEvents")]
    has_correlated_events: Option<bool>,
    /// Id of report source.
    #[serde(rename = "SourceId")]
    source_id: String,
    /// Describes the property.
    #[serde(rename = "Property")]
    property: String,
    /// Describes the property health state.
    #[serde(rename = "HealthState")]
    health_state: String,
    /// Time to live in milli-seconds.
    #[serde(rename = "TimeToLiveMs")]
    time_to_live_ms: i64,
    /// Sequence number of report.
    #[serde(rename = "SequenceNumber")]
    sequence_number: i64,
    /// Description of report.
    #[serde(rename = "Description")]
    description: String,
    /// Indicates the removal when it expires.
    #[serde(rename = "RemoveWhenExpired")]
    remove_when_expired: bool,
    /// Source time.
    #[serde(rename = "SourceUtcTimestamp")]
    source_utc_timestamp: String,
}

impl StatelessReplicaNewHealthReportEvent {
    /// Stateless Replica Health Report Created event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        source_id: String,
        property: String,
        health_state: String,
        time_to_live_ms: i64,
        sequence_number: i64,
        description: String,
        remove_when_expired: bool,
        source_utc_timestamp: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        StatelessReplicaNewHealthReportEvent {
            partition_id: None,
            replica_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            source_id,
            property,
            health_state,
            time_to_live_ms,
            sequence_number,
            description,
            remove_when_expired,
            source_utc_timestamp,
        }
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_replica_id(&mut self, replica_id: ::models::ReplicaIdInteger) {
        self.replica_id = Some(replica_id);
    }

    pub fn with_replica_id(
        mut self,
        replica_id: ::models::ReplicaIdInteger,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.replica_id = Some(replica_id);
        self
    }

    pub fn replica_id(&self) -> Option<&::models::ReplicaIdInteger> {
        self.replica_id.as_ref()
    }

    pub fn reset_replica_id(&mut self) {
        self.replica_id = None;
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::FabricEventKind {
        &self.kind
    }

    pub fn set_event_instance_id(&mut self, event_instance_id: String) {
        self.event_instance_id = event_instance_id;
    }

    pub fn with_event_instance_id(
        mut self,
        event_instance_id: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn with_category(
        mut self,
        category: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.category = Some(category);
        self
    }

    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    pub fn reset_category(&mut self) {
        self.category = None;
    }

    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    pub fn with_time_stamp(
        mut self,
        time_stamp: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.time_stamp = time_stamp;
        self
    }

    pub fn time_stamp(&self) -> &String {
        &self.time_stamp
    }

    pub fn set_has_correlated_events(&mut self, has_correlated_events: bool) {
        self.has_correlated_events = Some(has_correlated_events);
    }

    pub fn with_has_correlated_events(
        mut self,
        has_correlated_events: bool,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_source_id(&mut self, source_id: String) {
        self.source_id = source_id;
    }

    pub fn with_source_id(
        mut self,
        source_id: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.source_id = source_id;
        self
    }

    pub fn source_id(&self) -> &String {
        &self.source_id
    }

    pub fn set_property(&mut self, property: String) {
        self.property = property;
    }

    pub fn with_property(
        mut self,
        property: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.property = property;
        self
    }

    pub fn property(&self) -> &String {
        &self.property
    }

    pub fn set_health_state(&mut self, health_state: String) {
        self.health_state = health_state;
    }

    pub fn with_health_state(
        mut self,
        health_state: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.health_state = health_state;
        self
    }

    pub fn health_state(&self) -> &String {
        &self.health_state
    }

    pub fn set_time_to_live_ms(&mut self, time_to_live_ms: i64) {
        self.time_to_live_ms = time_to_live_ms;
    }

    pub fn with_time_to_live_ms(
        mut self,
        time_to_live_ms: i64,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.time_to_live_ms = time_to_live_ms;
        self
    }

    pub fn time_to_live_ms(&self) -> &i64 {
        &self.time_to_live_ms
    }

    pub fn set_sequence_number(&mut self, sequence_number: i64) {
        self.sequence_number = sequence_number;
    }

    pub fn with_sequence_number(
        mut self,
        sequence_number: i64,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.sequence_number = sequence_number;
        self
    }

    pub fn sequence_number(&self) -> &i64 {
        &self.sequence_number
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.description = description;
        self
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn set_remove_when_expired(&mut self, remove_when_expired: bool) {
        self.remove_when_expired = remove_when_expired;
    }

    pub fn with_remove_when_expired(
        mut self,
        remove_when_expired: bool,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.remove_when_expired = remove_when_expired;
        self
    }

    pub fn remove_when_expired(&self) -> &bool {
        &self.remove_when_expired
    }

    pub fn set_source_utc_timestamp(&mut self, source_utc_timestamp: String) {
        self.source_utc_timestamp = source_utc_timestamp;
    }

    pub fn with_source_utc_timestamp(
        mut self,
        source_utc_timestamp: String,
    ) -> StatelessReplicaNewHealthReportEvent {
        self.source_utc_timestamp = source_utc_timestamp;
        self
    }

    pub fn source_utc_timestamp(&self) -> &String {
        &self.source_utc_timestamp
    }
}
