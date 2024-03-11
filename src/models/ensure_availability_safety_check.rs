/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EnsureAvailabilitySafetyCheck : Safety check that waits to ensure the availability of the partition. It waits until there are replicas available such that bringing down this replica will not cause availability loss for the partition.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnsureAvailabilitySafetyCheck {
    /// Id of the partition which is undergoing the safety check.
    #[serde(rename = "PartitionId")]
    partition_id: Option<::models::PartitionId>,
    /// The kind of safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state. Following are the kinds of safety checks.
    #[serde(rename = "Kind")]
    kind: ::models::SafetyCheckKind,
}

impl EnsureAvailabilitySafetyCheck {
    /// Safety check that waits to ensure the availability of the partition. It waits until there are replicas available such that bringing down this replica will not cause availability loss for the partition.
    pub fn new(
        kind: ::models::SafetyCheckKind,
    ) -> EnsureAvailabilitySafetyCheck {
        EnsureAvailabilitySafetyCheck {
            partition_id: None,
            kind,
        }
    }

    pub fn set_partition_id(&mut self, partition_id: ::models::PartitionId) {
        self.partition_id = Some(partition_id);
    }

    pub fn with_partition_id(
        mut self,
        partition_id: ::models::PartitionId,
    ) -> EnsureAvailabilitySafetyCheck {
        self.partition_id = Some(partition_id);
        self
    }

    pub fn partition_id(&self) -> Option<&::models::PartitionId> {
        self.partition_id.as_ref()
    }

    pub fn reset_partition_id(&mut self) {
        self.partition_id = None;
    }

    pub fn set_kind(&mut self, kind: ::models::SafetyCheckKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::SafetyCheckKind,
    ) -> EnsureAvailabilitySafetyCheck {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::SafetyCheckKind {
        &self.kind
    }
}
