/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionInstanceCountScaleMechanism : Represents a scaling mechanism for adding or removing instances of stateless service partition.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionInstanceCountScaleMechanism {
    /// Specifies the kind of scaling mechanism
    #[serde(rename = "Kind")]
    kind: ::models::ScalingMechanismKind,
    /// Minimum number of instances of the partition.
    #[serde(rename = "MinInstanceCount")]
    min_instance_count: i32,
    /// Maximum number of instances of the partition.
    #[serde(rename = "MaxInstanceCount")]
    max_instance_count: i32,
    /// The number of instances to add or remove during a scaling operation.
    #[serde(rename = "ScaleIncrement")]
    scale_increment: i32,
}

impl PartitionInstanceCountScaleMechanism {
    /// Represents a scaling mechanism for adding or removing instances of stateless service partition.
    pub fn new(
        kind: ::models::ScalingMechanismKind,
        min_instance_count: i32,
        max_instance_count: i32,
        scale_increment: i32,
    ) -> PartitionInstanceCountScaleMechanism {
        PartitionInstanceCountScaleMechanism {
            kind,
            min_instance_count,
            max_instance_count,
            scale_increment,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ScalingMechanismKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ScalingMechanismKind,
    ) -> PartitionInstanceCountScaleMechanism {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ScalingMechanismKind {
        &self.kind
    }

    pub fn set_min_instance_count(&mut self, min_instance_count: i32) {
        self.min_instance_count = min_instance_count;
    }

    pub fn with_min_instance_count(
        mut self,
        min_instance_count: i32,
    ) -> PartitionInstanceCountScaleMechanism {
        self.min_instance_count = min_instance_count;
        self
    }

    pub fn min_instance_count(&self) -> &i32 {
        &self.min_instance_count
    }

    pub fn set_max_instance_count(&mut self, max_instance_count: i32) {
        self.max_instance_count = max_instance_count;
    }

    pub fn with_max_instance_count(
        mut self,
        max_instance_count: i32,
    ) -> PartitionInstanceCountScaleMechanism {
        self.max_instance_count = max_instance_count;
        self
    }

    pub fn max_instance_count(&self) -> &i32 {
        &self.max_instance_count
    }

    pub fn set_scale_increment(&mut self, scale_increment: i32) {
        self.scale_increment = scale_increment;
    }

    pub fn with_scale_increment(
        mut self,
        scale_increment: i32,
    ) -> PartitionInstanceCountScaleMechanism {
        self.scale_increment = scale_increment;
        self
    }

    pub fn scale_increment(&self) -> &i32 {
        &self.scale_increment
    }
}