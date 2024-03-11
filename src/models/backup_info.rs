/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupInfo : Represents a backup point which can be used to trigger a restore.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    /// Unique backup ID .
    #[serde(rename = "BackupId")]
    backup_id: Option<String>,
    /// Unique backup chain ID. All backups part of the same chain has the same backup chain id. A backup chain is comprised of 1 full backup and multiple incremental backups.
    #[serde(rename = "BackupChainId")]
    backup_chain_id: Option<String>,
    /// Name of the Service Fabric application this partition backup belongs to.
    #[serde(rename = "ApplicationName")]
    application_name: Option<String>,
    /// Name of the Service Fabric service this partition backup belongs to.
    #[serde(rename = "ServiceName")]
    service_name: Option<String>,
    /// Information about the partition to which this backup belongs to
    #[serde(rename = "PartitionInformation")]
    partition_information: Option<::models::PartitionInformation>,
    /// Location of the backup, relative to the backup store.
    #[serde(rename = "BackupLocation")]
    backup_location: Option<String>,
    /// Describes the type of backup, whether its full or incremental.
    #[serde(rename = "BackupType")]
    backup_type: Option<::models::BackupType>,
    /// Epoch of the last record in this backup.
    #[serde(rename = "EpochOfLastBackupRecord")]
    epoch_of_last_backup_record: Option<::models::Epoch>,
    /// LSN of the last record in this backup.
    #[serde(rename = "LsnOfLastBackupRecord")]
    lsn_of_last_backup_record: Option<String>,
    /// The date time when this backup was taken.
    #[serde(rename = "CreationTimeUtc")]
    creation_time_utc: Option<String>,
    /// Manifest Version of the service this partition backup belongs to.
    #[serde(rename = "ServiceManifestVersion")]
    service_manifest_version: Option<String>,
    /// Denotes the failure encountered in getting backup point information.
    #[serde(rename = "FailureError")]
    failure_error: Option<::models::FabricErrorError>,
}

impl Default for BackupInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupInfo {
    /// Represents a backup point which can be used to trigger a restore.
    pub fn new() -> BackupInfo {
        BackupInfo {
            backup_id: None,
            backup_chain_id: None,
            application_name: None,
            service_name: None,
            partition_information: None,
            backup_location: None,
            backup_type: None,
            epoch_of_last_backup_record: None,
            lsn_of_last_backup_record: None,
            creation_time_utc: None,
            service_manifest_version: None,
            failure_error: None,
        }
    }

    pub fn set_backup_id(&mut self, backup_id: String) {
        self.backup_id = Some(backup_id);
    }

    pub fn with_backup_id(mut self, backup_id: String) -> BackupInfo {
        self.backup_id = Some(backup_id);
        self
    }

    pub fn backup_id(&self) -> Option<&String> {
        self.backup_id.as_ref()
    }

    pub fn reset_backup_id(&mut self) {
        self.backup_id = None;
    }

    pub fn set_backup_chain_id(&mut self, backup_chain_id: String) {
        self.backup_chain_id = Some(backup_chain_id);
    }

    pub fn with_backup_chain_id(
        mut self,
        backup_chain_id: String,
    ) -> BackupInfo {
        self.backup_chain_id = Some(backup_chain_id);
        self
    }

    pub fn backup_chain_id(&self) -> Option<&String> {
        self.backup_chain_id.as_ref()
    }

    pub fn reset_backup_chain_id(&mut self) {
        self.backup_chain_id = None;
    }

    pub fn set_application_name(&mut self, application_name: String) {
        self.application_name = Some(application_name);
    }

    pub fn with_application_name(
        mut self,
        application_name: String,
    ) -> BackupInfo {
        self.application_name = Some(application_name);
        self
    }

    pub fn application_name(&self) -> Option<&String> {
        self.application_name.as_ref()
    }

    pub fn reset_application_name(&mut self) {
        self.application_name = None;
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = Some(service_name);
    }

    pub fn with_service_name(mut self, service_name: String) -> BackupInfo {
        self.service_name = Some(service_name);
        self
    }

    pub fn service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    pub fn reset_service_name(&mut self) {
        self.service_name = None;
    }

    pub fn set_partition_information(
        &mut self,
        partition_information: ::models::PartitionInformation,
    ) {
        self.partition_information = Some(partition_information);
    }

    pub fn with_partition_information(
        mut self,
        partition_information: ::models::PartitionInformation,
    ) -> BackupInfo {
        self.partition_information = Some(partition_information);
        self
    }

    pub fn partition_information(
        &self,
    ) -> Option<&::models::PartitionInformation> {
        self.partition_information.as_ref()
    }

    pub fn reset_partition_information(&mut self) {
        self.partition_information = None;
    }

    pub fn set_backup_location(&mut self, backup_location: String) {
        self.backup_location = Some(backup_location);
    }

    pub fn with_backup_location(
        mut self,
        backup_location: String,
    ) -> BackupInfo {
        self.backup_location = Some(backup_location);
        self
    }

    pub fn backup_location(&self) -> Option<&String> {
        self.backup_location.as_ref()
    }

    pub fn reset_backup_location(&mut self) {
        self.backup_location = None;
    }

    pub fn set_backup_type(&mut self, backup_type: ::models::BackupType) {
        self.backup_type = Some(backup_type);
    }

    pub fn with_backup_type(
        mut self,
        backup_type: ::models::BackupType,
    ) -> BackupInfo {
        self.backup_type = Some(backup_type);
        self
    }

    pub fn backup_type(&self) -> Option<&::models::BackupType> {
        self.backup_type.as_ref()
    }

    pub fn reset_backup_type(&mut self) {
        self.backup_type = None;
    }

    pub fn set_epoch_of_last_backup_record(
        &mut self,
        epoch_of_last_backup_record: ::models::Epoch,
    ) {
        self.epoch_of_last_backup_record = Some(epoch_of_last_backup_record);
    }

    pub fn with_epoch_of_last_backup_record(
        mut self,
        epoch_of_last_backup_record: ::models::Epoch,
    ) -> BackupInfo {
        self.epoch_of_last_backup_record = Some(epoch_of_last_backup_record);
        self
    }

    pub fn epoch_of_last_backup_record(&self) -> Option<&::models::Epoch> {
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
    ) -> BackupInfo {
        self.lsn_of_last_backup_record = Some(lsn_of_last_backup_record);
        self
    }

    pub fn lsn_of_last_backup_record(&self) -> Option<&String> {
        self.lsn_of_last_backup_record.as_ref()
    }

    pub fn reset_lsn_of_last_backup_record(&mut self) {
        self.lsn_of_last_backup_record = None;
    }

    pub fn set_creation_time_utc(&mut self, creation_time_utc: String) {
        self.creation_time_utc = Some(creation_time_utc);
    }

    pub fn with_creation_time_utc(
        mut self,
        creation_time_utc: String,
    ) -> BackupInfo {
        self.creation_time_utc = Some(creation_time_utc);
        self
    }

    pub fn creation_time_utc(&self) -> Option<&String> {
        self.creation_time_utc.as_ref()
    }

    pub fn reset_creation_time_utc(&mut self) {
        self.creation_time_utc = None;
    }

    pub fn set_service_manifest_version(
        &mut self,
        service_manifest_version: String,
    ) {
        self.service_manifest_version = Some(service_manifest_version);
    }

    pub fn with_service_manifest_version(
        mut self,
        service_manifest_version: String,
    ) -> BackupInfo {
        self.service_manifest_version = Some(service_manifest_version);
        self
    }

    pub fn service_manifest_version(&self) -> Option<&String> {
        self.service_manifest_version.as_ref()
    }

    pub fn reset_service_manifest_version(&mut self) {
        self.service_manifest_version = None;
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
    ) -> BackupInfo {
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
