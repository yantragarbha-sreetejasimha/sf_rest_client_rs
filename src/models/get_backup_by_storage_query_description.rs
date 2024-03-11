/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetBackupByStorageQueryDescription : Describes additional filters to be applied, while listing backups, and backup storage details from where to fetch the backups.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBackupByStorageQueryDescription {
    /// Specifies the start date time in ISO8601 from which to enumerate backups. If not specified, backups are enumerated from the beginning.
    #[serde(rename = "StartDateTimeFilter")]
    start_date_time_filter: Option<String>,
    /// Specifies the end date time in ISO8601 till which to enumerate backups. If not specified, backups are enumerated till the end.
    #[serde(rename = "EndDateTimeFilter")]
    end_date_time_filter: Option<String>,
    /// If specified as true, gets the most recent backup (within the specified time range) for every partition under the specified backup entity.
    #[serde(rename = "Latest")]
    latest: Option<bool>,
    /// Describes the parameters for the backup storage from where to enumerate backups. This is optional and by default backups are enumerated from the backup storage where this backup entity is currently being backed up (as specified in backup policy). This parameter is useful to be able to enumerate backups from another cluster where you may intend to restore.
    #[serde(rename = "Storage")]
    storage: ::models::BackupStorageDescription,
    /// Indicates the entity for which to enumerate backups.
    #[serde(rename = "BackupEntity")]
    backup_entity: ::models::BackupEntity,
}

impl GetBackupByStorageQueryDescription {
    /// Describes additional filters to be applied, while listing backups, and backup storage details from where to fetch the backups.
    pub fn new(
        storage: ::models::BackupStorageDescription,
        backup_entity: ::models::BackupEntity,
    ) -> GetBackupByStorageQueryDescription {
        GetBackupByStorageQueryDescription {
            start_date_time_filter: None,
            end_date_time_filter: None,
            latest: None,
            storage,
            backup_entity,
        }
    }

    pub fn set_start_date_time_filter(
        &mut self,
        start_date_time_filter: String,
    ) {
        self.start_date_time_filter = Some(start_date_time_filter);
    }

    pub fn with_start_date_time_filter(
        mut self,
        start_date_time_filter: String,
    ) -> GetBackupByStorageQueryDescription {
        self.start_date_time_filter = Some(start_date_time_filter);
        self
    }

    pub fn start_date_time_filter(&self) -> Option<&String> {
        self.start_date_time_filter.as_ref()
    }

    pub fn reset_start_date_time_filter(&mut self) {
        self.start_date_time_filter = None;
    }

    pub fn set_end_date_time_filter(&mut self, end_date_time_filter: String) {
        self.end_date_time_filter = Some(end_date_time_filter);
    }

    pub fn with_end_date_time_filter(
        mut self,
        end_date_time_filter: String,
    ) -> GetBackupByStorageQueryDescription {
        self.end_date_time_filter = Some(end_date_time_filter);
        self
    }

    pub fn end_date_time_filter(&self) -> Option<&String> {
        self.end_date_time_filter.as_ref()
    }

    pub fn reset_end_date_time_filter(&mut self) {
        self.end_date_time_filter = None;
    }

    pub fn set_latest(&mut self, latest: bool) {
        self.latest = Some(latest);
    }

    pub fn with_latest(
        mut self,
        latest: bool,
    ) -> GetBackupByStorageQueryDescription {
        self.latest = Some(latest);
        self
    }

    pub fn latest(&self) -> Option<&bool> {
        self.latest.as_ref()
    }

    pub fn reset_latest(&mut self) {
        self.latest = None;
    }

    pub fn set_storage(&mut self, storage: ::models::BackupStorageDescription) {
        self.storage = storage;
    }

    pub fn with_storage(
        mut self,
        storage: ::models::BackupStorageDescription,
    ) -> GetBackupByStorageQueryDescription {
        self.storage = storage;
        self
    }

    pub fn storage(&self) -> &::models::BackupStorageDescription {
        &self.storage
    }

    pub fn set_backup_entity(&mut self, backup_entity: ::models::BackupEntity) {
        self.backup_entity = backup_entity;
    }

    pub fn with_backup_entity(
        mut self,
        backup_entity: ::models::BackupEntity,
    ) -> GetBackupByStorageQueryDescription {
        self.backup_entity = backup_entity;
        self
    }

    pub fn backup_entity(&self) -> &::models::BackupEntity {
        &self.backup_entity
    }
}
