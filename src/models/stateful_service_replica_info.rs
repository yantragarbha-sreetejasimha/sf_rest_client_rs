/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatefulServiceReplicaInfo : Represents a stateful service replica. This includes information about the identity, role, status, health, node name, uptime, and other details about the replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulServiceReplicaInfo {
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// The status of a replica of a service.
    #[serde(rename = "ReplicaStatus")]
    replica_status: Option<::models::ReplicaStatus>,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: Option<::models::NodeName>,
    /// The address the replica is listening on.
    #[serde(rename = "Address")]
    address: Option<String>,
    /// The last in build duration of the replica in seconds.
    #[serde(rename = "LastInBuildDurationInSeconds")]
    last_in_build_duration_in_seconds: Option<String>,
    /// The role of a replica of a stateful service.
    #[serde(rename = "ReplicaRole")]
    replica_role: Option<::models::ReplicaRole>,
    /// Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id.
    #[serde(rename = "ReplicaId")]
    replica_id: Option<::models::ReplicaId>,
}

impl StatefulServiceReplicaInfo {
    /// Represents a stateful service replica. This includes information about the identity, role, status, health, node name, uptime, and other details about the replica.
    pub fn new(
        service_kind: ::models::ServiceKind,
    ) -> StatefulServiceReplicaInfo {
        StatefulServiceReplicaInfo {
            service_kind,
            replica_status: None,
            health_state: None,
            node_name: None,
            address: None,
            last_in_build_duration_in_seconds: None,
            replica_role: None,
            replica_id: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatefulServiceReplicaInfo {
        self.service_kind = service_kind;
        self
    }

    pub fn service_kind(&self) -> &::models::ServiceKind {
        &self.service_kind
    }

    pub fn set_replica_status(
        &mut self,
        replica_status: ::models::ReplicaStatus,
    ) {
        self.replica_status = Some(replica_status);
    }

    pub fn with_replica_status(
        mut self,
        replica_status: ::models::ReplicaStatus,
    ) -> StatefulServiceReplicaInfo {
        self.replica_status = Some(replica_status);
        self
    }

    pub fn replica_status(&self) -> Option<&::models::ReplicaStatus> {
        self.replica_status.as_ref()
    }

    pub fn reset_replica_status(&mut self) {
        self.replica_status = None;
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> StatefulServiceReplicaInfo {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = Some(node_name);
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> StatefulServiceReplicaInfo {
        self.node_name = Some(node_name);
        self
    }

    pub fn node_name(&self) -> Option<&::models::NodeName> {
        self.node_name.as_ref()
    }

    pub fn reset_node_name(&mut self) {
        self.node_name = None;
    }

    pub fn set_address(&mut self, address: String) {
        self.address = Some(address);
    }

    pub fn with_address(
        mut self,
        address: String,
    ) -> StatefulServiceReplicaInfo {
        self.address = Some(address);
        self
    }

    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    pub fn reset_address(&mut self) {
        self.address = None;
    }

    pub fn set_last_in_build_duration_in_seconds(
        &mut self,
        last_in_build_duration_in_seconds: String,
    ) {
        self.last_in_build_duration_in_seconds =
            Some(last_in_build_duration_in_seconds);
    }

    pub fn with_last_in_build_duration_in_seconds(
        mut self,
        last_in_build_duration_in_seconds: String,
    ) -> StatefulServiceReplicaInfo {
        self.last_in_build_duration_in_seconds =
            Some(last_in_build_duration_in_seconds);
        self
    }

    pub fn last_in_build_duration_in_seconds(&self) -> Option<&String> {
        self.last_in_build_duration_in_seconds.as_ref()
    }

    pub fn reset_last_in_build_duration_in_seconds(&mut self) {
        self.last_in_build_duration_in_seconds = None;
    }

    pub fn set_replica_role(&mut self, replica_role: ::models::ReplicaRole) {
        self.replica_role = Some(replica_role);
    }

    pub fn with_replica_role(
        mut self,
        replica_role: ::models::ReplicaRole,
    ) -> StatefulServiceReplicaInfo {
        self.replica_role = Some(replica_role);
        self
    }

    pub fn replica_role(&self) -> Option<&::models::ReplicaRole> {
        self.replica_role.as_ref()
    }

    pub fn reset_replica_role(&mut self) {
        self.replica_role = None;
    }

    pub fn set_replica_id(&mut self, replica_id: ::models::ReplicaId) {
        self.replica_id = Some(replica_id);
    }

    pub fn with_replica_id(
        mut self,
        replica_id: ::models::ReplicaId,
    ) -> StatefulServiceReplicaInfo {
        self.replica_id = Some(replica_id);
        self
    }

    pub fn replica_id(&self) -> Option<&::models::ReplicaId> {
        self.replica_id.as_ref()
    }

    pub fn reset_replica_id(&mut self) {
        self.replica_id = None;
    }
}
