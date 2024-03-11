/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TimeRange : Defines a time range in a 24 hour day specified by a start and end time.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRange {
    /// Defines an hour and minute of the day specified in 24 hour time.
    #[serde(rename = "StartTime")]
    start_time: Option<::models::TimeOfDay>,
    /// Defines an hour and minute of the day specified in 24 hour time.
    #[serde(rename = "EndTime")]
    end_time: Option<::models::TimeOfDay>,
}

impl Default for TimeRange {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeRange {
    /// Defines a time range in a 24 hour day specified by a start and end time.
    pub fn new() -> TimeRange {
        TimeRange {
            start_time: None,
            end_time: None,
        }
    }

    pub fn set_start_time(&mut self, start_time: ::models::TimeOfDay) {
        self.start_time = Some(start_time);
    }

    pub fn with_start_time(
        mut self,
        start_time: ::models::TimeOfDay,
    ) -> TimeRange {
        self.start_time = Some(start_time);
        self
    }

    pub fn start_time(&self) -> Option<&::models::TimeOfDay> {
        self.start_time.as_ref()
    }

    pub fn reset_start_time(&mut self) {
        self.start_time = None;
    }

    pub fn set_end_time(&mut self, end_time: ::models::TimeOfDay) {
        self.end_time = Some(end_time);
    }

    pub fn with_end_time(mut self, end_time: ::models::TimeOfDay) -> TimeRange {
        self.end_time = Some(end_time);
        self
    }

    pub fn end_time(&self) -> Option<&::models::TimeOfDay> {
        self.end_time.as_ref()
    }

    pub fn reset_end_time(&mut self) {
        self.end_time = None;
    }
}