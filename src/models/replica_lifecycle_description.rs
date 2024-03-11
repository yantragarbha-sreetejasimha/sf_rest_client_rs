/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicaLifecycleDescription : Describes how the replica will behave

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaLifecycleDescription {
    /// If set to true, replicas with a target replica set size of 1 will be permitted to move during upgrade.
    #[serde(rename = "IsSingletonReplicaMoveAllowedDuringUpgrade")]
    is_singleton_replica_move_allowed_during_upgrade: Option<bool>,
    /// If set to true, move/swap replica to original location after upgrade.
    #[serde(rename = "RestoreReplicaLocationAfterUpgrade")]
    restore_replica_location_after_upgrade: Option<bool>,
}

impl Default for ReplicaLifecycleDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplicaLifecycleDescription {
    /// Describes how the replica will behave
    pub fn new() -> ReplicaLifecycleDescription {
        ReplicaLifecycleDescription {
            is_singleton_replica_move_allowed_during_upgrade: None,
            restore_replica_location_after_upgrade: None,
        }
    }

    pub fn set_is_singleton_replica_move_allowed_during_upgrade(
        &mut self,
        is_singleton_replica_move_allowed_during_upgrade: bool,
    ) {
        self.is_singleton_replica_move_allowed_during_upgrade =
            Some(is_singleton_replica_move_allowed_during_upgrade);
    }

    pub fn with_is_singleton_replica_move_allowed_during_upgrade(
        mut self,
        is_singleton_replica_move_allowed_during_upgrade: bool,
    ) -> ReplicaLifecycleDescription {
        self.is_singleton_replica_move_allowed_during_upgrade =
            Some(is_singleton_replica_move_allowed_during_upgrade);
        self
    }

    pub fn is_singleton_replica_move_allowed_during_upgrade(
        &self,
    ) -> Option<&bool> {
        self.is_singleton_replica_move_allowed_during_upgrade
            .as_ref()
    }

    pub fn reset_is_singleton_replica_move_allowed_during_upgrade(&mut self) {
        self.is_singleton_replica_move_allowed_during_upgrade = None;
    }

    pub fn set_restore_replica_location_after_upgrade(
        &mut self,
        restore_replica_location_after_upgrade: bool,
    ) {
        self.restore_replica_location_after_upgrade =
            Some(restore_replica_location_after_upgrade);
    }

    pub fn with_restore_replica_location_after_upgrade(
        mut self,
        restore_replica_location_after_upgrade: bool,
    ) -> ReplicaLifecycleDescription {
        self.restore_replica_location_after_upgrade =
            Some(restore_replica_location_after_upgrade);
        self
    }

    pub fn restore_replica_location_after_upgrade(&self) -> Option<&bool> {
        self.restore_replica_location_after_upgrade.as_ref()
    }

    pub fn reset_restore_replica_location_after_upgrade(&mut self) {
        self.restore_replica_location_after_upgrade = None;
    }
}
