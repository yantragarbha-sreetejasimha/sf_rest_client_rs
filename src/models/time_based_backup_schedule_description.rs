/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TimeBasedBackupScheduleDescription : Describes the time based backup schedule.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeBasedBackupScheduleDescription {
    /// The kind of backup schedule, time based or frequency based.
    #[serde(rename = "ScheduleKind")]
    schedule_kind: ::models::BackupScheduleKind,
    /// Describes the frequency with which to run the time based backup schedule.
    #[serde(rename = "ScheduleFrequencyType")]
    schedule_frequency_type: ::models::BackupScheduleFrequencyType,
    /// List of days of a week when to trigger the periodic backup. This is valid only when the backup schedule frequency type is weekly.
    #[serde(rename = "RunDays")]
    run_days: Option<::models::DayOfWeekList>,
    /// Represents the list of exact time during the day in ISO8601 format. Like '19:00:00' will represent '7PM' during the day. Date specified along with time will be ignored.
    #[serde(rename = "RunTimes")]
    run_times: ::models::TimeList,
}

impl TimeBasedBackupScheduleDescription {
    /// Describes the time based backup schedule.
    pub fn new(
        schedule_kind: ::models::BackupScheduleKind,
        schedule_frequency_type: ::models::BackupScheduleFrequencyType,
        run_times: ::models::TimeList,
    ) -> TimeBasedBackupScheduleDescription {
        TimeBasedBackupScheduleDescription {
            schedule_kind,
            schedule_frequency_type,
            run_days: None,
            run_times,
        }
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
    ) -> TimeBasedBackupScheduleDescription {
        self.schedule_kind = schedule_kind;
        self
    }

    pub fn schedule_kind(&self) -> &::models::BackupScheduleKind {
        &self.schedule_kind
    }

    pub fn set_schedule_frequency_type(
        &mut self,
        schedule_frequency_type: ::models::BackupScheduleFrequencyType,
    ) {
        self.schedule_frequency_type = schedule_frequency_type;
    }

    pub fn with_schedule_frequency_type(
        mut self,
        schedule_frequency_type: ::models::BackupScheduleFrequencyType,
    ) -> TimeBasedBackupScheduleDescription {
        self.schedule_frequency_type = schedule_frequency_type;
        self
    }

    pub fn schedule_frequency_type(
        &self,
    ) -> &::models::BackupScheduleFrequencyType {
        &self.schedule_frequency_type
    }

    pub fn set_run_days(&mut self, run_days: ::models::DayOfWeekList) {
        self.run_days = Some(run_days);
    }

    pub fn with_run_days(
        mut self,
        run_days: ::models::DayOfWeekList,
    ) -> TimeBasedBackupScheduleDescription {
        self.run_days = Some(run_days);
        self
    }

    pub fn run_days(&self) -> Option<&::models::DayOfWeekList> {
        self.run_days.as_ref()
    }

    pub fn reset_run_days(&mut self) {
        self.run_days = None;
    }

    pub fn set_run_times(&mut self, run_times: ::models::TimeList) {
        self.run_times = run_times;
    }

    pub fn with_run_times(
        mut self,
        run_times: ::models::TimeList,
    ) -> TimeBasedBackupScheduleDescription {
        self.run_times = run_times;
        self
    }

    pub fn run_times(&self) -> &::models::TimeList {
        &self.run_times
    }
}
