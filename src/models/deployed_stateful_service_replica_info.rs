/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedStatefulServiceReplicaInfo : Information about a stateful service replica deployed on a node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedStatefulServiceReplicaInfo {
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// The full name of the service with 'fabric:' URI scheme.
    #[serde(rename = "ServiceName")]
    service_name: Option<::models::ServiceName>,
    /// Name of the service type as specified in the service manifest.
    #[serde(rename = "ServiceTypeName")]
    service_type_name: Option<::models::ServiceTypeName>,
    /// The name of the service manifest in which this service type is defined.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: Option<::models::ServiceManifestName>,
    /// The name of the code package that hosts this replica.
    #[serde(rename = "CodePackageName")]
    code_package_name: Option<::models::CodePackageName>,
    /// An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// The status of a replica of a service.
    #[serde(rename = "ReplicaStatus")]
    replica_status: Option<::models::ReplicaStatus>,
    /// The last address returned by the replica in Open or ChangeRole.
    #[serde(rename = "Address")]
    address: Option<String>,
    /// The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId is always an empty string.
    #[serde(rename = "ServicePackageActivationId")]
    service_package_activation_id: Option<::models::ServicePackageActivationId>,
    /// Host process ID of the process that is hosting the replica. This will be zero if the replica is down. In hyper-v containers this host process ID will be from different kernel.
    #[serde(rename = "HostProcessId")]
    host_process_id: Option<String>,
    /// Id of a stateful service replica. ReplicaId is used by Service Fabric to uniquely identify a replica of a partition. It is unique within a partition and does not change for the lifetime of the replica. If a replica gets dropped and another replica gets created on the same node for the same partition, it will get a different value for the id. Sometimes the id of a stateless service instance is also referred as a replica id.
    #[serde(rename = "ReplicaId")]
    replica_id: Option<::models::ReplicaId>,
    /// The role of a replica of a stateful service.
    #[serde(rename = "ReplicaRole")]
    replica_role: Option<::models::ReplicaRole>,
    /// Information about current reconfiguration like phase, type, previous configuration role of replica and reconfiguration start date time.
    #[serde(rename = "ReconfigurationInformation")]
    reconfiguration_information: Option<::models::ReconfigurationInformation>,
}

impl DeployedStatefulServiceReplicaInfo {
    /// Information about a stateful service replica deployed on a node.
    pub fn new(
        service_kind: ::models::ServiceKind,
    ) -> DeployedStatefulServiceReplicaInfo {
        DeployedStatefulServiceReplicaInfo {
            service_kind,
            service_name: None,
            service_type_name: None,
            service_manifest_name: None,
            code_package_name: None,
            partition_id: None,
            replica_status: None,
            address: None,
            service_package_activation_id: None,
            host_process_id: None,
            replica_id: None,
            replica_role: None,
            reconfiguration_information: None,
        }
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> DeployedStatefulServiceReplicaInfo {
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
    ) -> DeployedStatefulServiceReplicaInfo {
        self.service_name = Some(service_name);
        self
    }

    pub fn service_name(&self) -> Option<&::models::ServiceName> {
        self.service_name.as_ref()
    }

    pub fn reset_service_name(&mut self) {
        self.service_name = None;
    }

    pub fn set_service_type_name(
        &mut self,
        service_type_name: ::models::ServiceTypeName,
    ) {
        self.service_type_name = Some(service_type_name);
    }

    pub fn with_service_type_name(
        mut self,
        service_type_name: ::models::ServiceTypeName,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.service_type_name = Some(service_type_name);
        self
    }

    pub fn service_type_name(&self) -> Option<&::models::ServiceTypeName> {
        self.service_type_name.as_ref()
    }

    pub fn reset_service_type_name(&mut self) {
        self.service_type_name = None;
    }

    pub fn set_service_manifest_name(
        &mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) {
        self.service_manifest_name = Some(service_manifest_name);
    }

    pub fn with_service_manifest_name(
        mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.service_manifest_name = Some(service_manifest_name);
        self
    }

    pub fn service_manifest_name(
        &self,
    ) -> Option<&::models::ServiceManifestName> {
        self.service_manifest_name.as_ref()
    }

    pub fn reset_service_manifest_name(&mut self) {
        self.service_manifest_name = None;
    }

    pub fn set_code_package_name(
        &mut self,
        code_package_name: ::models::CodePackageName,
    ) {
        self.code_package_name = Some(code_package_name);
    }

    pub fn with_code_package_name(
        mut self,
        code_package_name: ::models::CodePackageName,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.code_package_name = Some(code_package_name);
        self
    }

    pub fn code_package_name(&self) -> Option<&::models::CodePackageName> {
        self.code_package_name.as_ref()
    }

    pub fn reset_code_package_name(&mut self) {
        self.code_package_name = None;
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
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
    ) -> DeployedStatefulServiceReplicaInfo {
        self.replica_status = Some(replica_status);
        self
    }

    pub fn replica_status(&self) -> Option<&::models::ReplicaStatus> {
        self.replica_status.as_ref()
    }

    pub fn reset_replica_status(&mut self) {
        self.replica_status = None;
    }

    pub fn set_address(&mut self, address: String) {
        self.address = Some(address);
    }

    pub fn with_address(
        mut self,
        address: String,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.address = Some(address);
        self
    }

    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    pub fn reset_address(&mut self) {
        self.address = None;
    }

    pub fn set_service_package_activation_id(
        &mut self,
        service_package_activation_id: ::models::ServicePackageActivationId,
    ) {
        self.service_package_activation_id =
            Some(service_package_activation_id);
    }

    pub fn with_service_package_activation_id(
        mut self,
        service_package_activation_id: ::models::ServicePackageActivationId,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.service_package_activation_id =
            Some(service_package_activation_id);
        self
    }

    pub fn service_package_activation_id(
        &self,
    ) -> Option<&::models::ServicePackageActivationId> {
        self.service_package_activation_id.as_ref()
    }

    pub fn reset_service_package_activation_id(&mut self) {
        self.service_package_activation_id = None;
    }

    pub fn set_host_process_id(&mut self, host_process_id: String) {
        self.host_process_id = Some(host_process_id);
    }

    pub fn with_host_process_id(
        mut self,
        host_process_id: String,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.host_process_id = Some(host_process_id);
        self
    }

    pub fn host_process_id(&self) -> Option<&String> {
        self.host_process_id.as_ref()
    }

    pub fn reset_host_process_id(&mut self) {
        self.host_process_id = None;
    }

    pub fn set_replica_id(&mut self, replica_id: ::models::ReplicaId) {
        self.replica_id = Some(replica_id);
    }

    pub fn with_replica_id(
        mut self,
        replica_id: ::models::ReplicaId,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.replica_id = Some(replica_id);
        self
    }

    pub fn replica_id(&self) -> Option<&::models::ReplicaId> {
        self.replica_id.as_ref()
    }

    pub fn reset_replica_id(&mut self) {
        self.replica_id = None;
    }

    pub fn set_replica_role(&mut self, replica_role: ::models::ReplicaRole) {
        self.replica_role = Some(replica_role);
    }

    pub fn with_replica_role(
        mut self,
        replica_role: ::models::ReplicaRole,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.replica_role = Some(replica_role);
        self
    }

    pub fn replica_role(&self) -> Option<&::models::ReplicaRole> {
        self.replica_role.as_ref()
    }

    pub fn reset_replica_role(&mut self) {
        self.replica_role = None;
    }

    pub fn set_reconfiguration_information(
        &mut self,
        reconfiguration_information: ::models::ReconfigurationInformation,
    ) {
        self.reconfiguration_information = Some(reconfiguration_information);
    }

    pub fn with_reconfiguration_information(
        mut self,
        reconfiguration_information: ::models::ReconfigurationInformation,
    ) -> DeployedStatefulServiceReplicaInfo {
        self.reconfiguration_information = Some(reconfiguration_information);
        self
    }

    pub fn reconfiguration_information(
        &self,
    ) -> Option<&::models::ReconfigurationInformation> {
        self.reconfiguration_information.as_ref()
    }

    pub fn reset_reconfiguration_information(&mut self) {
        self.reconfiguration_information = None;
    }
}
