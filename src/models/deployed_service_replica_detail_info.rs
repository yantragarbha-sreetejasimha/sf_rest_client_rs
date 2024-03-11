/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServiceReplicaDetailInfo : Information about a Service Fabric service replica deployed on a node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServiceReplicaDetailInfo {
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// Full hierarchical name of the service in URI format starting with `fabric:`.
    #[serde(rename = "ServiceName")]
    service_name: Option<::models::ServiceName>,
    /// An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// Specifies the current active life-cycle operation on a stateful service replica or stateless service instance.
    #[serde(rename = "CurrentServiceOperation")]
    current_service_operation: Option<::models::ServiceOperationName>,
    /// The start time of the current service operation in UTC format.
    #[serde(rename = "CurrentServiceOperationStartTimeUtc")]
    current_service_operation_start_time_utc: Option<String>,
    /// List of load reported by replica.
    #[serde(rename = "ReportedLoad")]
    reported_load: Option<::models::LoadMetricReportInfoList>,
}

impl DeployedServiceReplicaDetailInfo {
    /// Information about a Service Fabric service replica deployed on a node.
    pub fn new(
        service_kind: ::models::ServiceKind,
    ) -> DeployedServiceReplicaDetailInfo {
        DeployedServiceReplicaDetailInfo {
            service_kind,
            service_name: None,
            partition_id: None,
            current_service_operation: None,
            current_service_operation_start_time_utc: None,
            reported_load: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> DeployedServiceReplicaDetailInfo {
        self.service_kind = service_kind;
        self
    }

    pub fn service_kind(&self) -> &::models::ServiceKind {
        &self.service_kind
    }

    pub fn set_service_name(&mut self, service_name: ::models::ServiceName) {
        self.service_name = Some(service_name);
    }

    pub fn with_service_name(
        mut self,
        service_name: ::models::ServiceName,
    ) -> DeployedServiceReplicaDetailInfo {
        self.service_name = Some(service_name);
        self
    }

    pub fn service_name(&self) -> Option<&::models::ServiceName> {
        self.service_name.as_ref()
    }

    pub fn reset_service_name(&mut self) {
        self.service_name = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> DeployedServiceReplicaDetailInfo {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_current_service_operation(
        &mut self,
        current_service_operation: ::models::ServiceOperationName,
    ) {
        self.current_service_operation = Some(current_service_operation);
    }

    pub fn with_current_service_operation(
        mut self,
        current_service_operation: ::models::ServiceOperationName,
    ) -> DeployedServiceReplicaDetailInfo {
        self.current_service_operation = Some(current_service_operation);
        self
    }

    pub fn current_service_operation(
        &self,
    ) -> Option<&::models::ServiceOperationName> {
        self.current_service_operation.as_ref()
    }

    pub fn reset_current_service_operation(&mut self) {
        self.current_service_operation = None;
    }

    pub fn set_current_service_operation_start_time_utc(
        &mut self,
        current_service_operation_start_time_utc: String,
    ) {
        self.current_service_operation_start_time_utc =
            Some(current_service_operation_start_time_utc);
    }

    pub fn with_current_service_operation_start_time_utc(
        mut self,
        current_service_operation_start_time_utc: String,
    ) -> DeployedServiceReplicaDetailInfo {
        self.current_service_operation_start_time_utc =
            Some(current_service_operation_start_time_utc);
        self
    }

    pub fn current_service_operation_start_time_utc(&self) -> Option<&String> {
        self.current_service_operation_start_time_utc.as_ref()
    }

    pub fn reset_current_service_operation_start_time_utc(&mut self) {
        self.current_service_operation_start_time_utc = None;
    }

    pub fn set_reported_load(
        &mut self,
        reported_load: ::models::LoadMetricReportInfoList,
    ) {
        self.reported_load = Some(reported_load);
    }

    pub fn with_reported_load(
        mut self,
        reported_load: ::models::LoadMetricReportInfoList,
    ) -> DeployedServiceReplicaDetailInfo {
        self.reported_load = Some(reported_load);
        self
    }

    pub fn reported_load(&self) -> Option<&::models::LoadMetricReportInfoList> {
        self.reported_load.as_ref()
    }

    pub fn reset_reported_load(&mut self) {
        self.reported_load = None;
    }
}
