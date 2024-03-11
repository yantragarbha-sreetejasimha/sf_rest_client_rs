/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AddRemoveReplicaScalingMechanism : Describes the horizontal auto scaling mechanism that adds or removes replicas (containers or container groups).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRemoveReplicaScalingMechanism {
    /// The type of auto scaling mechanism.
    #[serde(rename = "kind")]
    kind: ::models::AutoScalingMechanismKind,
    /// Minimum number of containers (scale down won't be performed below this number).
    #[serde(rename = "minCount")]
    min_count: i32,
    /// Maximum number of containers (scale up won't be performed above this number).
    #[serde(rename = "maxCount")]
    max_count: i32,
    /// Each time auto scaling is performed, this number of containers will be added or removed.
    #[serde(rename = "scaleIncrement")]
    scale_increment: i32,
}

impl AddRemoveReplicaScalingMechanism {
    /// Describes the horizontal auto scaling mechanism that adds or removes replicas (containers or container groups).
    pub fn new(
        kind: ::models::AutoScalingMechanismKind,
        min_count: i32,
        max_count: i32,
        scale_increment: i32,
    ) -> AddRemoveReplicaScalingMechanism {
        AddRemoveReplicaScalingMechanism {
            kind,
            min_count,
            max_count,
            scale_increment,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::AutoScalingMechanismKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::AutoScalingMechanismKind,
    ) -> AddRemoveReplicaScalingMechanism {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::AutoScalingMechanismKind {
        &self.kind
    }

    pub fn set_min_count(&mut self, min_count: i32) {
        self.min_count = min_count;
    }

    pub fn with_min_count(
        mut self,
        min_count: i32,
    ) -> AddRemoveReplicaScalingMechanism {
        self.min_count = min_count;
        self
    }

    pub fn min_count(&self) -> &i32 {
        &self.min_count
    }

    pub fn set_max_count(&mut self, max_count: i32) {
        self.max_count = max_count;
    }

    pub fn with_max_count(
        mut self,
        max_count: i32,
    ) -> AddRemoveReplicaScalingMechanism {
        self.max_count = max_count;
        self
    }

    pub fn max_count(&self) -> &i32 {
        &self.max_count
    }

    pub fn set_scale_increment(&mut self, scale_increment: i32) {
        self.scale_increment = scale_increment;
    }

    pub fn with_scale_increment(
        mut self,
        scale_increment: i32,
    ) -> AddRemoveReplicaScalingMechanism {
        self.scale_increment = scale_increment;
        self
    }

    pub fn scale_increment(&self) -> &i32 {
        &self.scale_increment
    }
}
