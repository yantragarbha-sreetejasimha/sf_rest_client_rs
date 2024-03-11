/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// KeyValueStoreReplicaStatus : Key value store related information for the replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValueStoreReplicaStatus {
    /// The role of a replica of a stateful service.
    #[serde(rename = "Kind")]
    kind: ::models::ReplicaKind,
    /// Value indicating the estimated number of rows in the underlying database.
    #[serde(rename = "DatabaseRowCountEstimate")]
    database_row_count_estimate: Option<String>,
    /// Value indicating the estimated size of the underlying database.
    #[serde(rename = "DatabaseLogicalSizeEstimate")]
    database_logical_size_estimate: Option<String>,
    /// Value indicating the latest key-prefix filter applied to enumeration during the callback. Null if there is no pending callback.
    #[serde(rename = "CopyNotificationCurrentKeyFilter")]
    copy_notification_current_key_filter: Option<String>,
    /// Value indicating the latest number of keys enumerated during the callback. 0 if there is no pending callback.
    #[serde(rename = "CopyNotificationCurrentProgress")]
    copy_notification_current_progress: Option<String>,
    /// Value indicating the current status details of the replica.
    #[serde(rename = "StatusDetails")]
    status_details: Option<String>,
}

impl KeyValueStoreReplicaStatus {
    /// Key value store related information for the replica.
    pub fn new(kind: ::models::ReplicaKind) -> KeyValueStoreReplicaStatus {
        KeyValueStoreReplicaStatus {
            kind,
            database_row_count_estimate: None,
            database_logical_size_estimate: None,
            copy_notification_current_key_filter: None,
            copy_notification_current_progress: None,
            status_details: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ReplicaKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ReplicaKind,
    ) -> KeyValueStoreReplicaStatus {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ReplicaKind {
        &self.kind
    }

    pub fn set_database_row_count_estimate(
        &mut self,
        database_row_count_estimate: String,
    ) {
        self.database_row_count_estimate = Some(database_row_count_estimate);
    }

    pub fn with_database_row_count_estimate(
        mut self,
        database_row_count_estimate: String,
    ) -> KeyValueStoreReplicaStatus {
        self.database_row_count_estimate = Some(database_row_count_estimate);
        self
    }

    pub fn database_row_count_estimate(&self) -> Option<&String> {
        self.database_row_count_estimate.as_ref()
    }

    pub fn reset_database_row_count_estimate(&mut self) {
        self.database_row_count_estimate = None;
    }

    pub fn set_database_logical_size_estimate(
        &mut self,
        database_logical_size_estimate: String,
    ) {
        self.database_logical_size_estimate =
            Some(database_logical_size_estimate);
    }

    pub fn with_database_logical_size_estimate(
        mut self,
        database_logical_size_estimate: String,
    ) -> KeyValueStoreReplicaStatus {
        self.database_logical_size_estimate =
            Some(database_logical_size_estimate);
        self
    }

    pub fn database_logical_size_estimate(&self) -> Option<&String> {
        self.database_logical_size_estimate.as_ref()
    }

    pub fn reset_database_logical_size_estimate(&mut self) {
        self.database_logical_size_estimate = None;
    }

    pub fn set_copy_notification_current_key_filter(
        &mut self,
        copy_notification_current_key_filter: String,
    ) {
        self.copy_notification_current_key_filter =
            Some(copy_notification_current_key_filter);
    }

    pub fn with_copy_notification_current_key_filter(
        mut self,
        copy_notification_current_key_filter: String,
    ) -> KeyValueStoreReplicaStatus {
        self.copy_notification_current_key_filter =
            Some(copy_notification_current_key_filter);
        self
    }

    pub fn copy_notification_current_key_filter(&self) -> Option<&String> {
        self.copy_notification_current_key_filter.as_ref()
    }

    pub fn reset_copy_notification_current_key_filter(&mut self) {
        self.copy_notification_current_key_filter = None;
    }

    pub fn set_copy_notification_current_progress(
        &mut self,
        copy_notification_current_progress: String,
    ) {
        self.copy_notification_current_progress =
            Some(copy_notification_current_progress);
    }

    pub fn with_copy_notification_current_progress(
        mut self,
        copy_notification_current_progress: String,
    ) -> KeyValueStoreReplicaStatus {
        self.copy_notification_current_progress =
            Some(copy_notification_current_progress);
        self
    }

    pub fn copy_notification_current_progress(&self) -> Option<&String> {
        self.copy_notification_current_progress.as_ref()
    }

    pub fn reset_copy_notification_current_progress(&mut self) {
        self.copy_notification_current_progress = None;
    }

    pub fn set_status_details(&mut self, status_details: String) {
        self.status_details = Some(status_details);
    }

    pub fn with_status_details(
        mut self,
        status_details: String,
    ) -> KeyValueStoreReplicaStatus {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }
}
