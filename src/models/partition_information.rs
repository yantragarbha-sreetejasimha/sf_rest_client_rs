/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionInformation : Information about the partition identity, partitioning scheme and keys supported by it.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionInformation {
    /// The kind of partitioning scheme used to partition the service.
    #[serde(rename = "ServicePartitionKind")]
    service_partition_kind: ::models::ServicePartitionKind,
    /// An internal ID used by Service Fabric to uniquely identify a partition. This is a randomly generated GUID when the service was created. The partition ID is unique and does not change for the lifetime of the service. If the same service was deleted and recreated the IDs of its partitions would be different.
    #[serde(rename = "Id")]
    id: Option<::models::PartitionId>,
}

impl PartitionInformation {
    /// Information about the partition identity, partitioning scheme and keys supported by it.
    pub fn new(
        service_partition_kind: ::models::ServicePartitionKind,
    ) -> PartitionInformation {
        PartitionInformation {
            service_partition_kind,
            id: None,
        }
    }

    pub fn set_service_partition_kind(
        &mut self,
        service_partition_kind: ::models::ServicePartitionKind,
    ) {
        self.service_partition_kind = service_partition_kind;
    }

    pub fn with_service_partition_kind(
        mut self,
        service_partition_kind: ::models::ServicePartitionKind,
    ) -> PartitionInformation {
        self.service_partition_kind = service_partition_kind;
        self
    }

    pub fn service_partition_kind(&self) -> &::models::ServicePartitionKind {
        &self.service_partition_kind
    }

    pub fn set_id(&mut self, id: ::models::PartitionId) {
        self.id = Some(id);
    }

    pub fn with_id(
        mut self,
        id: ::models::PartitionId,
    ) -> PartitionInformation {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&::models::PartitionId> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }
}
