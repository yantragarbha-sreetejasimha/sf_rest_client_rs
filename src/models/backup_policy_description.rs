/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupPolicyDescription : Describes a backup policy for configuring periodic backup.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupPolicyDescription {
    /// The unique name identifying this backup policy.
    #[serde(rename = "Name")]
    name: String,
    /// Specifies whether to trigger restore automatically using the latest available backup in case the partition experiences a data loss event.
    #[serde(rename = "AutoRestoreOnDataLoss")]
    auto_restore_on_data_loss: bool,
    /// Defines the maximum number of incremental backups to be taken between two full backups. This is just the upper limit. A full backup may be taken before specified number of incremental backups are completed in one of the following conditions - The replica has never taken a full backup since it has become primary, - Some of the log records since the last backup has been truncated, or - Replica passed the MaxAccumulatedBackupLogSizeInMB limit.
    #[serde(rename = "MaxIncrementalBackups")]
    max_incremental_backups: i32,
    /// Describes the backup schedule parameters.
    #[serde(rename = "Schedule")]
    schedule: ::models::BackupScheduleDescription,
    /// Describes the details of backup storage where to store the periodic backups.
    #[serde(rename = "Storage")]
    storage: ::models::BackupStorageDescription,
    /// Describes the policy to retain backups in storage.
    #[serde(rename = "RetentionPolicy")]
    retention_policy: Option<::models::RetentionPolicyDescription>,
}

impl BackupPolicyDescription {
    /// Describes a backup policy for configuring periodic backup.
    pub fn new(
        name: String,
        auto_restore_on_data_loss: bool,
        max_incremental_backups: i32,
        schedule: ::models::BackupScheduleDescription,
        storage: ::models::BackupStorageDescription,
    ) -> BackupPolicyDescription {
        BackupPolicyDescription {
            name,
            auto_restore_on_data_loss,
            max_incremental_backups,
            schedule,
            storage,
            retention_policy: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> BackupPolicyDescription {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_auto_restore_on_data_loss(
        &mut self,
        auto_restore_on_data_loss: bool,
    ) {
        self.auto_restore_on_data_loss = auto_restore_on_data_loss;
    }

    pub fn with_auto_restore_on_data_loss(
        mut self,
        auto_restore_on_data_loss: bool,
    ) -> BackupPolicyDescription {
        self.auto_restore_on_data_loss = auto_restore_on_data_loss;
        self
    }

    pub fn auto_restore_on_data_loss(&self) -> &bool {
        &self.auto_restore_on_data_loss
    }

    pub fn set_max_incremental_backups(
        &mut self,
        max_incremental_backups: i32,
    ) {
        self.max_incremental_backups = max_incremental_backups;
    }

    pub fn with_max_incremental_backups(
        mut self,
        max_incremental_backups: i32,
    ) -> BackupPolicyDescription {
        self.max_incremental_backups = max_incremental_backups;
        self
    }

    pub fn max_incremental_backups(&self) -> &i32 {
        &self.max_incremental_backups
    }

    pub fn set_schedule(
        &mut self,
        schedule: ::models::BackupScheduleDescription,
    ) {
        self.schedule = schedule;
    }

    pub fn with_schedule(
        mut self,
        schedule: ::models::BackupScheduleDescription,
    ) -> BackupPolicyDescription {
        self.schedule = schedule;
        self
    }

    pub fn schedule(&self) -> &::models::BackupScheduleDescription {
        &self.schedule
    }

    pub fn set_storage(&mut self, storage: ::models::BackupStorageDescription) {
        self.storage = storage;
    }

    pub fn with_storage(
        mut self,
        storage: ::models::BackupStorageDescription,
    ) -> BackupPolicyDescription {
        self.storage = storage;
        self
    }

    pub fn storage(&self) -> &::models::BackupStorageDescription {
        &self.storage
    }

    pub fn set_retention_policy(
        &mut self,
        retention_policy: ::models::RetentionPolicyDescription,
    ) {
        self.retention_policy = Some(retention_policy);
    }

    pub fn with_retention_policy(
        mut self,
        retention_policy: ::models::RetentionPolicyDescription,
    ) -> BackupPolicyDescription {
        self.retention_policy = Some(retention_policy);
        self
    }

    pub fn retention_policy(
        &self,
    ) -> Option<&::models::RetentionPolicyDescription> {
        self.retention_policy.as_ref()
    }

    pub fn reset_retention_policy(&mut self) {
        self.retention_policy = None;
    }
}
