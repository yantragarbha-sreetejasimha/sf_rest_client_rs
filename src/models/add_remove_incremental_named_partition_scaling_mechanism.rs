/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AddRemoveIncrementalNamedPartitionScalingMechanism : Represents a scaling mechanism for adding or removing named partitions of a stateless service. Partition names are in the format '0','1''N-1'

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRemoveIncrementalNamedPartitionScalingMechanism {
    /// Specifies the kind of scaling mechanism
    #[serde(rename = "Kind")]
    kind: ::models::ScalingMechanismKind,
    /// Minimum number of named partitions of the service.
    #[serde(rename = "MinPartitionCount")]
    min_partition_count: i32,
    /// Maximum number of named partitions of the service.
    #[serde(rename = "MaxPartitionCount")]
    max_partition_count: i32,
    /// The number of instances to add or remove during a scaling operation.
    #[serde(rename = "ScaleIncrement")]
    scale_increment: i32,
}

impl AddRemoveIncrementalNamedPartitionScalingMechanism {
    /// Represents a scaling mechanism for adding or removing named partitions of a stateless service. Partition names are in the format '0','1''N-1'
    pub fn new(
        kind: ::models::ScalingMechanismKind,
        min_partition_count: i32,
        max_partition_count: i32,
        scale_increment: i32,
    ) -> AddRemoveIncrementalNamedPartitionScalingMechanism {
        AddRemoveIncrementalNamedPartitionScalingMechanism {
            kind,
            min_partition_count,
            max_partition_count,
            scale_increment,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ScalingMechanismKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ScalingMechanismKind,
    ) -> AddRemoveIncrementalNamedPartitionScalingMechanism {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ScalingMechanismKind {
        &self.kind
    }

    pub fn set_min_partition_count(&mut self, min_partition_count: i32) {
        self.min_partition_count = min_partition_count;
    }

    pub fn with_min_partition_count(
        mut self,
        min_partition_count: i32,
    ) -> AddRemoveIncrementalNamedPartitionScalingMechanism {
        self.min_partition_count = min_partition_count;
        self
    }

    pub fn min_partition_count(&self) -> &i32 {
        &self.min_partition_count
    }

    pub fn set_max_partition_count(&mut self, max_partition_count: i32) {
        self.max_partition_count = max_partition_count;
    }

    pub fn with_max_partition_count(
        mut self,
        max_partition_count: i32,
    ) -> AddRemoveIncrementalNamedPartitionScalingMechanism {
        self.max_partition_count = max_partition_count;
        self
    }

    pub fn max_partition_count(&self) -> &i32 {
        &self.max_partition_count
    }

    pub fn set_scale_increment(&mut self, scale_increment: i32) {
        self.scale_increment = scale_increment;
    }

    pub fn with_scale_increment(
        mut self,
        scale_increment: i32,
    ) -> AddRemoveIncrementalNamedPartitionScalingMechanism {
        self.scale_increment = scale_increment;
        self
    }

    pub fn scale_increment(&self) -> &i32 {
        &self.scale_increment
    }
}
