/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupProgressInfo : Describes the progress of a partition's backup.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupProgressInfo {
    /// Represents the current state of the partition backup operation.
    #[serde(rename = "BackupState")]
    backup_state: Option<::models::BackupState>,
    /// TimeStamp in UTC when operation succeeded or failed.
    #[serde(rename = "TimeStampUtc")]
    time_stamp_utc: Option<String>,
    /// Unique ID of the newly created backup.
    #[serde(rename = "BackupId")]
    backup_id: Option<String>,
    /// Location, relative to the backup store, of the newly created backup.
    #[serde(rename = "BackupLocation")]
    backup_location: Option<String>,
    /// Specifies the epoch of the last record included in backup.
    #[serde(rename = "EpochOfLastBackupRecord")]
    epoch_of_last_backup_record: Option<::models::BackupEpoch>,
    /// The LSN of last record included in backup.
    #[serde(rename = "LsnOfLastBackupRecord")]
    lsn_of_last_backup_record: Option<String>,
    /// Denotes the failure encountered in performing backup operation.
    #[serde(rename = "FailureError")]
    failure_error: Option<::models::FabricErrorError>,
}

impl Default for BackupProgressInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupProgressInfo {
    /// Describes the progress of a partition's backup.
    pub fn new() -> BackupProgressInfo {
        BackupProgressInfo {
            backup_state: None,
            time_stamp_utc: None,
            backup_id: None,
            backup_location: None,
            epoch_of_last_backup_record: None,
            lsn_of_last_backup_record: None,
            failure_error: None,
        }
    }

    pub fn set_backup_state(&mut self, backup_state: ::models::BackupState) {
        self.backup_state = Some(backup_state);
    }

    pub fn with_backup_state(
        mut self,
        backup_state: ::models::BackupState,
    ) -> BackupProgressInfo {
        self.backup_state = Some(backup_state);
        self
    }

    pub fn backup_state(&self) -> Option<&::models::BackupState> {
        self.backup_state.as_ref()
    }

    pub fn reset_backup_state(&mut self) {
        self.backup_state = None;
    }

    pub fn set_time_stamp_utc(&mut self, time_stamp_utc: String) {
        self.time_stamp_utc = Some(time_stamp_utc);
    }

    pub fn with_time_stamp_utc(
        mut self,
        time_stamp_utc: String,
    ) -> BackupProgressInfo {
        self.time_stamp_utc = Some(time_stamp_utc);
        self
    }

    pub fn time_stamp_utc(&self) -> Option<&String> {
        self.time_stamp_utc.as_ref()
    }

    pub fn reset_time_stamp_utc(&mut self) {
        self.time_stamp_utc = None;
    }

    pub fn set_backup_id(&mut self, backup_id: String) {
        self.backup_id = Some(backup_id);
    }

    pub fn with_backup_id(mut self, backup_id: String) -> BackupProgressInfo {
        self.backup_id = Some(backup_id);
        self
    }

    pub fn backup_id(&self) -> Option<&String> {
        self.backup_id.as_ref()
    }

    pub fn reset_backup_id(&mut self) {
        self.backup_id = None;
    }

    pub fn set_backup_location(&mut self, backup_location: String) {
        self.backup_location = Some(backup_location);
    }

    pub fn with_backup_location(
        mut self,
        backup_location: String,
    ) -> BackupProgressInfo {
        self.backup_location = Some(backup_location);
        self
    }

    pub fn backup_location(&self) -> Option<&String> {
        self.backup_location.as_ref()
    }

    pub fn reset_backup_location(&mut self) {
        self.backup_location = None;
    }

    pub fn set_epoch_of_last_backup_record(
        &mut self,
        epoch_of_last_backup_record: ::models::BackupEpoch,
    ) {
        self.epoch_of_last_backup_record = Some(epoch_of_last_backup_record);
    }

    pub fn with_epoch_of_last_backup_record(
        mut self,
        epoch_of_last_backup_record: ::models::BackupEpoch,
    ) -> BackupProgressInfo {
        self.epoch_of_last_backup_record = Some(epoch_of_last_backup_record);
        self
    }

    pub fn epoch_of_last_backup_record(
        &self,
    ) -> Option<&::models::BackupEpoch> {
        self.epoch_of_last_backup_record.as_ref()
    }

    pub fn reset_epoch_of_last_backup_record(&mut self) {
        self.epoch_of_last_backup_record = None;
    }

    pub fn set_lsn_of_last_backup_record(
        &mut self,
        lsn_of_last_backup_record: String,
    ) {
        self.lsn_of_last_backup_record = Some(lsn_of_last_backup_record);
    }

    pub fn with_lsn_of_last_backup_record(
        mut self,
        lsn_of_last_backup_record: String,
    ) -> BackupProgressInfo {
        self.lsn_of_last_backup_record = Some(lsn_of_last_backup_record);
        self
    }

    pub fn lsn_of_last_backup_record(&self) -> Option<&String> {
        self.lsn_of_last_backup_record.as_ref()
    }

    pub fn reset_lsn_of_last_backup_record(&mut self) {
        self.lsn_of_last_backup_record = None;
    }

    pub fn set_failure_error(
        &mut self,
        failure_error: ::models::FabricErrorError,
    ) {
        self.failure_error = Some(failure_error);
    }

    pub fn with_failure_error(
        mut self,
        failure_error: ::models::FabricErrorError,
    ) -> BackupProgressInfo {
        self.failure_error = Some(failure_error);
        self
    }

    pub fn failure_error(&self) -> Option<&::models::FabricErrorError> {
        self.failure_error.as_ref()
    }

    pub fn reset_failure_error(&mut self) {
        self.failure_error = None;
    }
}
