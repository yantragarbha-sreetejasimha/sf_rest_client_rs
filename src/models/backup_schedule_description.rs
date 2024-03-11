/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupScheduleDescription : Describes the backup schedule parameters.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupScheduleDescription {
    /// The kind of backup schedule, time based or frequency based.
    #[serde(rename = "ScheduleKind")]
    schedule_kind: ::models::BackupScheduleKind,
}

impl BackupScheduleDescription {
    /// Describes the backup schedule parameters.
    pub fn new(
        schedule_kind: ::models::BackupScheduleKind,
    ) -> BackupScheduleDescription {
        BackupScheduleDescription { schedule_kind }
    }

    pub fn set_schedule_kind(
        &mut self,
        schedule_kind: ::models::BackupScheduleKind,
    ) {
        self.schedule_kind = schedule_kind;
    }

    pub fn with_schedule_kind(
        mut self,
        schedule_kind: ::models::BackupScheduleKind,
    ) -> BackupScheduleDescription {
        self.schedule_kind = schedule_kind;
        self
    }

    pub fn schedule_kind(&self) -> &::models::BackupScheduleKind {
        &self.schedule_kind
    }
}
