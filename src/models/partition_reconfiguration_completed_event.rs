/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionReconfigurationCompletedEvent : Partition Reconfiguration Completed event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionReconfigurationCompletedEvent {
    /// The kind of FabricEvent.
    #[serde(rename = "Kind")]
    kind: ::models::FabricEventKind,
    /// The identifier for the FabricEvent instance.
    #[serde(rename = "EventInstanceId")]
    event_instance_id: String,
    /// The time event was logged.
    #[serde(rename = "TimeStamp")]
    time_stamp: String,
    /// Shows there is existing related events available.
    #[serde(rename = "HasCorrelatedEvents")]
    has_correlated_events: Option<bool>,
    /// An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different.
    #[serde(rename = "PartitionId")]
    partition_id: ::models::PartitionId,
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: ::models::NodeName,
    /// Id of Node instance.
    #[serde(rename = "NodeInstanceId")]
    node_instance_id: String,
    /// Type of Service.
    #[serde(rename = "ServiceType")]
    service_type: String,
    /// CcEpochDataLoss version.
    #[serde(rename = "CcEpochDataLossVersion")]
    cc_epoch_data_loss_version: i64,
    /// CcEpochConfig version.
    #[serde(rename = "CcEpochConfigVersion")]
    cc_epoch_config_version: i64,
    /// Type of reconfiguration.
    #[serde(rename = "ReconfigType")]
    reconfig_type: String,
    /// Describes reconfiguration result.
    #[serde(rename = "Result")]
    result: String,
    /// Duration of Phase0 in milli-seconds.
    #[serde(rename = "Phase0DurationMs")]
    phase0_duration_ms: f64,
    /// Duration of Phase1 in milli-seconds.
    #[serde(rename = "Phase1DurationMs")]
    phase1_duration_ms: f64,
    /// Duration of Phase2 in milli-seconds.
    #[serde(rename = "Phase2DurationMs")]
    phase2_duration_ms: f64,
    /// Duration of Phase3 in milli-seconds.
    #[serde(rename = "Phase3DurationMs")]
    phase3_duration_ms: f64,
    /// Duration of Phase4 in milli-seconds.
    #[serde(rename = "Phase4DurationMs")]
    phase4_duration_ms: f64,
    /// Total duration in milli-seconds.
    #[serde(rename = "TotalDurationMs")]
    total_duration_ms: f64,
}

impl PartitionReconfigurationCompletedEvent {
    /// Partition Reconfiguration Completed event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        partition_id: ::models::PartitionId,
        node_name: ::models::NodeName,
        node_instance_id: String,
        service_type: String,
        cc_epoch_data_loss_version: i64,
        cc_epoch_config_version: i64,
        reconfig_type: String,
        result: String,
        phase0_duration_ms: f64,
        phase1_duration_ms: f64,
        phase2_duration_ms: f64,
        phase3_duration_ms: f64,
        phase4_duration_ms: f64,
        total_duration_ms: f64,
    ) -> PartitionReconfigurationCompletedEvent {
        PartitionReconfigurationCompletedEvent {
            kind,
            event_instance_id,
            time_stamp,
            has_correlated_events: None,
            partition_id,
            node_name,
            node_instance_id,
            service_type,
            cc_epoch_data_loss_version,
            cc_epoch_config_version,
            reconfig_type,
            result,
            phase0_duration_ms,
            phase1_duration_ms,
            phase2_duration_ms,
            phase3_duration_ms,
            phase4_duration_ms,
            total_duration_ms,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> PartitionReconfigurationCompletedEvent {
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
    ) -> PartitionReconfigurationCompletedEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    pub fn with_time_stamp(
        mut self,
        time_stamp: String,
    ) -> PartitionReconfigurationCompletedEvent {
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
    ) -> PartitionReconfigurationCompletedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = partition_id;
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> PartitionReconfigurationCompletedEvent {
        self.partition_id = partition_id;
        self
    }

    pub fn partition_id(&self) -> &::models::PartitionId {
        &self.partition_id
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = node_name;
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> PartitionReconfigurationCompletedEvent {
        self.node_name = node_name;
        self
    }

    pub fn node_name(&self) -> &::models::NodeName {
        &self.node_name
    }

    pub fn set_node_instance_id(&mut self, node_instance_id: String) {
        self.node_instance_id = node_instance_id;
    }

    pub fn with_node_instance_id(
        mut self,
        node_instance_id: String,
    ) -> PartitionReconfigurationCompletedEvent {
        self.node_instance_id = node_instance_id;
        self
    }

    pub fn node_instance_id(&self) -> &String {
        &self.node_instance_id
    }

    pub fn set_service_type(&mut self, service_type: String) {
        self.service_type = service_type;
    }

    pub fn with_service_type(
        mut self,
        service_type: String,
    ) -> PartitionReconfigurationCompletedEvent {
        self.service_type = service_type;
        self
    }

    pub fn service_type(&self) -> &String {
        &self.service_type
    }

    pub fn set_cc_epoch_data_loss_version(
        &mut self,
        cc_epoch_data_loss_version: i64,
    ) {
        self.cc_epoch_data_loss_version = cc_epoch_data_loss_version;
    }

    pub fn with_cc_epoch_data_loss_version(
        mut self,
        cc_epoch_data_loss_version: i64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.cc_epoch_data_loss_version = cc_epoch_data_loss_version;
        self
    }

    pub fn cc_epoch_data_loss_version(&self) -> &i64 {
        &self.cc_epoch_data_loss_version
    }

    pub fn set_cc_epoch_config_version(
        &mut self,
        cc_epoch_config_version: i64,
    ) {
        self.cc_epoch_config_version = cc_epoch_config_version;
    }

    pub fn with_cc_epoch_config_version(
        mut self,
        cc_epoch_config_version: i64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.cc_epoch_config_version = cc_epoch_config_version;
        self
    }

    pub fn cc_epoch_config_version(&self) -> &i64 {
        &self.cc_epoch_config_version
    }

    pub fn set_reconfig_type(&mut self, reconfig_type: String) {
        self.reconfig_type = reconfig_type;
    }

    pub fn with_reconfig_type(
        mut self,
        reconfig_type: String,
    ) -> PartitionReconfigurationCompletedEvent {
        self.reconfig_type = reconfig_type;
        self
    }

    pub fn reconfig_type(&self) -> &String {
        &self.reconfig_type
    }

    pub fn set_result(&mut self, result: String) {
        self.result = result;
    }

    pub fn with_result(
        mut self,
        result: String,
    ) -> PartitionReconfigurationCompletedEvent {
        self.result = result;
        self
    }

    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn set_phase0_duration_ms(&mut self, phase0_duration_ms: f64) {
        self.phase0_duration_ms = phase0_duration_ms;
    }

    pub fn with_phase0_duration_ms(
        mut self,
        phase0_duration_ms: f64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.phase0_duration_ms = phase0_duration_ms;
        self
    }

    pub fn phase0_duration_ms(&self) -> &f64 {
        &self.phase0_duration_ms
    }

    pub fn set_phase1_duration_ms(&mut self, phase1_duration_ms: f64) {
        self.phase1_duration_ms = phase1_duration_ms;
    }

    pub fn with_phase1_duration_ms(
        mut self,
        phase1_duration_ms: f64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.phase1_duration_ms = phase1_duration_ms;
        self
    }

    pub fn phase1_duration_ms(&self) -> &f64 {
        &self.phase1_duration_ms
    }

    pub fn set_phase2_duration_ms(&mut self, phase2_duration_ms: f64) {
        self.phase2_duration_ms = phase2_duration_ms;
    }

    pub fn with_phase2_duration_ms(
        mut self,
        phase2_duration_ms: f64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.phase2_duration_ms = phase2_duration_ms;
        self
    }

    pub fn phase2_duration_ms(&self) -> &f64 {
        &self.phase2_duration_ms
    }

    pub fn set_phase3_duration_ms(&mut self, phase3_duration_ms: f64) {
        self.phase3_duration_ms = phase3_duration_ms;
    }

    pub fn with_phase3_duration_ms(
        mut self,
        phase3_duration_ms: f64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.phase3_duration_ms = phase3_duration_ms;
        self
    }

    pub fn phase3_duration_ms(&self) -> &f64 {
        &self.phase3_duration_ms
    }

    pub fn set_phase4_duration_ms(&mut self, phase4_duration_ms: f64) {
        self.phase4_duration_ms = phase4_duration_ms;
    }

    pub fn with_phase4_duration_ms(
        mut self,
        phase4_duration_ms: f64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.phase4_duration_ms = phase4_duration_ms;
        self
    }

    pub fn phase4_duration_ms(&self) -> &f64 {
        &self.phase4_duration_ms
    }

    pub fn set_total_duration_ms(&mut self, total_duration_ms: f64) {
        self.total_duration_ms = total_duration_ms;
    }

    pub fn with_total_duration_ms(
        mut self,
        total_duration_ms: f64,
    ) -> PartitionReconfigurationCompletedEvent {
        self.total_duration_ms = total_duration_ms;
        self
    }

    pub fn total_duration_ms(&self) -> &f64 {
        &self.total_duration_ms
    }
}
