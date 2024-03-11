/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UnplacedReplicaInformation : Contains information for an unplaced replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnplacedReplicaInformation {
    /// The name of the service.
    #[serde(rename = "ServiceName")]
    service_name: Option<::models::ServiceName>,
    /// The ID of the partition.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// List of reasons due to which a replica cannot be placed.
    #[serde(rename = "UnplacedReplicaDetails")]
    unplaced_replica_details: Option<Vec<String>>,
}

impl Default for UnplacedReplicaInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl UnplacedReplicaInformation {
    /// Contains information for an unplaced replica.
    pub fn new() -> UnplacedReplicaInformation {
        UnplacedReplicaInformation {
            service_name: None,
            partition_id: None,
            unplaced_replica_details: None,
        }
    }

    pub fn set_service_name(&mut self, service_name: ::models::ServiceName) {
        self.service_name = Some(service_name);
    }

    pub fn with_service_name(
        mut self,
        service_name: ::models::ServiceName,
    ) -> UnplacedReplicaInformation {
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
    ) -> UnplacedReplicaInformation {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_unplaced_replica_details(
        &mut self,
        unplaced_replica_details: Vec<String>,
    ) {
        self.unplaced_replica_details = Some(unplaced_replica_details);
    }

    pub fn with_unplaced_replica_details(
        mut self,
        unplaced_replica_details: Vec<String>,
    ) -> UnplacedReplicaInformation {
        self.unplaced_replica_details = Some(unplaced_replica_details);
        self
    }

    pub fn unplaced_replica_details(&self) -> Option<&Vec<String>> {
        self.unplaced_replica_details.as_ref()
    }

    pub fn reset_unplaced_replica_details(&mut self) {
        self.unplaced_replica_details = None;
    }
}
