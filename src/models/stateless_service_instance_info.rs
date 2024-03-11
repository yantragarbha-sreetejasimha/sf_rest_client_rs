/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatelessServiceInstanceInfo : Represents a stateless service instance. This includes information about the identity, status, health, node name, uptime, and other details about the instance.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatelessServiceInstanceInfo {
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
    /// Id of a stateless service instance. InstanceId is used by Service Fabric to uniquely identify an instance of a partition of a stateless service. It is unique within a partition and does not change for the lifetime of the instance. If the instance has failed over on the same or different node, it will get a different value for the InstanceId.
    #[serde(rename = "InstanceId")]
    instance_id: Option<::models::InstanceId>,
}

impl StatelessServiceInstanceInfo {
    /// Represents a stateless service instance. This includes information about the identity, status, health, node name, uptime, and other details about the instance.
    pub fn new(
        service_kind: ::models::ServiceKind,
    ) -> StatelessServiceInstanceInfo {
        StatelessServiceInstanceInfo {
            service_kind,
            replica_status: None,
            health_state: None,
            node_name: None,
            address: None,
            last_in_build_duration_in_seconds: None,
            instance_id: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatelessServiceInstanceInfo {
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
    ) -> StatelessServiceInstanceInfo {
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
    ) -> StatelessServiceInstanceInfo {
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
    ) -> StatelessServiceInstanceInfo {
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
    ) -> StatelessServiceInstanceInfo {
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
    ) -> StatelessServiceInstanceInfo {
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

    pub fn set_instance_id(&mut self, instance_id: ::models::InstanceId) {
        self.instance_id = Some(instance_id);
    }

    pub fn with_instance_id(
        mut self,
        instance_id: ::models::InstanceId,
    ) -> StatelessServiceInstanceInfo {
        self.instance_id = Some(instance_id);
        self
    }

    pub fn instance_id(&self) -> Option<&::models::InstanceId> {
        self.instance_id.as_ref()
    }

    pub fn reset_instance_id(&mut self) {
        self.instance_id = None;
    }
}
