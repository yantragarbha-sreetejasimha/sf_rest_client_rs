/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairTaskHistory : A record of the times when the repair task entered each state.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairTaskHistory {
    /// The time when the repair task entered the Created state.
    #[serde(rename = "CreatedUtcTimestamp")]
    created_utc_timestamp: Option<String>,
    /// The time when the repair task entered the Claimed state.
    #[serde(rename = "ClaimedUtcTimestamp")]
    claimed_utc_timestamp: Option<String>,
    /// The time when the repair task entered the Preparing state.
    #[serde(rename = "PreparingUtcTimestamp")]
    preparing_utc_timestamp: Option<String>,
    /// The time when the repair task entered the Approved state
    #[serde(rename = "ApprovedUtcTimestamp")]
    approved_utc_timestamp: Option<String>,
    /// The time when the repair task entered the Executing state
    #[serde(rename = "ExecutingUtcTimestamp")]
    executing_utc_timestamp: Option<String>,
    /// The time when the repair task entered the Restoring state
    #[serde(rename = "RestoringUtcTimestamp")]
    restoring_utc_timestamp: Option<String>,
    /// The time when the repair task entered the Completed state
    #[serde(rename = "CompletedUtcTimestamp")]
    completed_utc_timestamp: Option<String>,
    /// The time when the repair task started the health check in the Preparing state.
    #[serde(rename = "PreparingHealthCheckStartUtcTimestamp")]
    preparing_health_check_start_utc_timestamp: Option<String>,
    /// The time when the repair task completed the health check in the Preparing state.
    #[serde(rename = "PreparingHealthCheckEndUtcTimestamp")]
    preparing_health_check_end_utc_timestamp: Option<String>,
    /// The time when the repair task started the health check in the Restoring state.
    #[serde(rename = "RestoringHealthCheckStartUtcTimestamp")]
    restoring_health_check_start_utc_timestamp: Option<String>,
    /// The time when the repair task completed the health check in the Restoring state.
    #[serde(rename = "RestoringHealthCheckEndUtcTimestamp")]
    restoring_health_check_end_utc_timestamp: Option<String>,
}

impl Default for RepairTaskHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairTaskHistory {
    /// A record of the times when the repair task entered each state.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.
    pub fn new() -> RepairTaskHistory {
        RepairTaskHistory {
            created_utc_timestamp: None,
            claimed_utc_timestamp: None,
            preparing_utc_timestamp: None,
            approved_utc_timestamp: None,
            executing_utc_timestamp: None,
            restoring_utc_timestamp: None,
            completed_utc_timestamp: None,
            preparing_health_check_start_utc_timestamp: None,
            preparing_health_check_end_utc_timestamp: None,
            restoring_health_check_start_utc_timestamp: None,
            restoring_health_check_end_utc_timestamp: None,
        }
    }

    pub fn set_created_utc_timestamp(&mut self, created_utc_timestamp: String) {
        self.created_utc_timestamp = Some(created_utc_timestamp);
    }

    pub fn with_created_utc_timestamp(
        mut self,
        created_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.created_utc_timestamp = Some(created_utc_timestamp);
        self
    }

    pub fn created_utc_timestamp(&self) -> Option<&String> {
        self.created_utc_timestamp.as_ref()
    }

    pub fn reset_created_utc_timestamp(&mut self) {
        self.created_utc_timestamp = None;
    }

    pub fn set_claimed_utc_timestamp(&mut self, claimed_utc_timestamp: String) {
        self.claimed_utc_timestamp = Some(claimed_utc_timestamp);
    }

    pub fn with_claimed_utc_timestamp(
        mut self,
        claimed_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.claimed_utc_timestamp = Some(claimed_utc_timestamp);
        self
    }

    pub fn claimed_utc_timestamp(&self) -> Option<&String> {
        self.claimed_utc_timestamp.as_ref()
    }

    pub fn reset_claimed_utc_timestamp(&mut self) {
        self.claimed_utc_timestamp = None;
    }

    pub fn set_preparing_utc_timestamp(
        &mut self,
        preparing_utc_timestamp: String,
    ) {
        self.preparing_utc_timestamp = Some(preparing_utc_timestamp);
    }

    pub fn with_preparing_utc_timestamp(
        mut self,
        preparing_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.preparing_utc_timestamp = Some(preparing_utc_timestamp);
        self
    }

    pub fn preparing_utc_timestamp(&self) -> Option<&String> {
        self.preparing_utc_timestamp.as_ref()
    }

    pub fn reset_preparing_utc_timestamp(&mut self) {
        self.preparing_utc_timestamp = None;
    }

    pub fn set_approved_utc_timestamp(
        &mut self,
        approved_utc_timestamp: String,
    ) {
        self.approved_utc_timestamp = Some(approved_utc_timestamp);
    }

    pub fn with_approved_utc_timestamp(
        mut self,
        approved_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.approved_utc_timestamp = Some(approved_utc_timestamp);
        self
    }

    pub fn approved_utc_timestamp(&self) -> Option<&String> {
        self.approved_utc_timestamp.as_ref()
    }

    pub fn reset_approved_utc_timestamp(&mut self) {
        self.approved_utc_timestamp = None;
    }

    pub fn set_executing_utc_timestamp(
        &mut self,
        executing_utc_timestamp: String,
    ) {
        self.executing_utc_timestamp = Some(executing_utc_timestamp);
    }

    pub fn with_executing_utc_timestamp(
        mut self,
        executing_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.executing_utc_timestamp = Some(executing_utc_timestamp);
        self
    }

    pub fn executing_utc_timestamp(&self) -> Option<&String> {
        self.executing_utc_timestamp.as_ref()
    }

    pub fn reset_executing_utc_timestamp(&mut self) {
        self.executing_utc_timestamp = None;
    }

    pub fn set_restoring_utc_timestamp(
        &mut self,
        restoring_utc_timestamp: String,
    ) {
        self.restoring_utc_timestamp = Some(restoring_utc_timestamp);
    }

    pub fn with_restoring_utc_timestamp(
        mut self,
        restoring_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.restoring_utc_timestamp = Some(restoring_utc_timestamp);
        self
    }

    pub fn restoring_utc_timestamp(&self) -> Option<&String> {
        self.restoring_utc_timestamp.as_ref()
    }

    pub fn reset_restoring_utc_timestamp(&mut self) {
        self.restoring_utc_timestamp = None;
    }

    pub fn set_completed_utc_timestamp(
        &mut self,
        completed_utc_timestamp: String,
    ) {
        self.completed_utc_timestamp = Some(completed_utc_timestamp);
    }

    pub fn with_completed_utc_timestamp(
        mut self,
        completed_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.completed_utc_timestamp = Some(completed_utc_timestamp);
        self
    }

    pub fn completed_utc_timestamp(&self) -> Option<&String> {
        self.completed_utc_timestamp.as_ref()
    }

    pub fn reset_completed_utc_timestamp(&mut self) {
        self.completed_utc_timestamp = None;
    }

    pub fn set_preparing_health_check_start_utc_timestamp(
        &mut self,
        preparing_health_check_start_utc_timestamp: String,
    ) {
        self.preparing_health_check_start_utc_timestamp =
            Some(preparing_health_check_start_utc_timestamp);
    }

    pub fn with_preparing_health_check_start_utc_timestamp(
        mut self,
        preparing_health_check_start_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.preparing_health_check_start_utc_timestamp =
            Some(preparing_health_check_start_utc_timestamp);
        self
    }

    pub fn preparing_health_check_start_utc_timestamp(
        &self,
    ) -> Option<&String> {
        self.preparing_health_check_start_utc_timestamp.as_ref()
    }

    pub fn reset_preparing_health_check_start_utc_timestamp(&mut self) {
        self.preparing_health_check_start_utc_timestamp = None;
    }

    pub fn set_preparing_health_check_end_utc_timestamp(
        &mut self,
        preparing_health_check_end_utc_timestamp: String,
    ) {
        self.preparing_health_check_end_utc_timestamp =
            Some(preparing_health_check_end_utc_timestamp);
    }

    pub fn with_preparing_health_check_end_utc_timestamp(
        mut self,
        preparing_health_check_end_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.preparing_health_check_end_utc_timestamp =
            Some(preparing_health_check_end_utc_timestamp);
        self
    }

    pub fn preparing_health_check_end_utc_timestamp(&self) -> Option<&String> {
        self.preparing_health_check_end_utc_timestamp.as_ref()
    }

    pub fn reset_preparing_health_check_end_utc_timestamp(&mut self) {
        self.preparing_health_check_end_utc_timestamp = None;
    }

    pub fn set_restoring_health_check_start_utc_timestamp(
        &mut self,
        restoring_health_check_start_utc_timestamp: String,
    ) {
        self.restoring_health_check_start_utc_timestamp =
            Some(restoring_health_check_start_utc_timestamp);
    }

    pub fn with_restoring_health_check_start_utc_timestamp(
        mut self,
        restoring_health_check_start_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.restoring_health_check_start_utc_timestamp =
            Some(restoring_health_check_start_utc_timestamp);
        self
    }

    pub fn restoring_health_check_start_utc_timestamp(
        &self,
    ) -> Option<&String> {
        self.restoring_health_check_start_utc_timestamp.as_ref()
    }

    pub fn reset_restoring_health_check_start_utc_timestamp(&mut self) {
        self.restoring_health_check_start_utc_timestamp = None;
    }

    pub fn set_restoring_health_check_end_utc_timestamp(
        &mut self,
        restoring_health_check_end_utc_timestamp: String,
    ) {
        self.restoring_health_check_end_utc_timestamp =
            Some(restoring_health_check_end_utc_timestamp);
    }

    pub fn with_restoring_health_check_end_utc_timestamp(
        mut self,
        restoring_health_check_end_utc_timestamp: String,
    ) -> RepairTaskHistory {
        self.restoring_health_check_end_utc_timestamp =
            Some(restoring_health_check_end_utc_timestamp);
        self
    }

    pub fn restoring_health_check_end_utc_timestamp(&self) -> Option<&String> {
        self.restoring_health_check_end_utc_timestamp.as_ref()
    }

    pub fn reset_restoring_health_check_end_utc_timestamp(&mut self) {
        self.restoring_health_check_end_utc_timestamp = None;
    }
}