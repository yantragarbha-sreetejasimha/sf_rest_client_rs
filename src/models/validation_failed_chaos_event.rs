/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ValidationFailedChaosEvent : Chaos event corresponding to a failure during validation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationFailedChaosEvent {
    /// The kind of Chaos event.
    #[serde(rename = "Kind")]
    kind: ::models::ChaosEventKind,
    /// The UTC timestamp when this Chaos event was generated.
    #[serde(rename = "TimeStampUtc")]
    time_stamp_utc: String,
    /// Describes why the ValidationFailedChaosEvent was generated. This may happen because more than MaxPercentUnhealthyNodes are unhealthy for more than MaxClusterStabilizationTimeout. This reason will be in the Reason property of the ValidationFailedChaosEvent as a string.
    #[serde(rename = "Reason")]
    reason: Option<String>,
}

impl ValidationFailedChaosEvent {
    /// Chaos event corresponding to a failure during validation.
    pub fn new(
        kind: ::models::ChaosEventKind,
        time_stamp_utc: String,
    ) -> ValidationFailedChaosEvent {
        ValidationFailedChaosEvent {
            kind,
            time_stamp_utc,
            reason: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ChaosEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ChaosEventKind,
    ) -> ValidationFailedChaosEvent {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ChaosEventKind {
        &self.kind
    }

    pub fn set_time_stamp_utc(&mut self, time_stamp_utc: String) {
        self.time_stamp_utc = time_stamp_utc;
    }

    pub fn with_time_stamp_utc(
        mut self,
        time_stamp_utc: String,
    ) -> ValidationFailedChaosEvent {
        self.time_stamp_utc = time_stamp_utc;
        self
    }

    pub fn time_stamp_utc(&self) -> &String {
        &self.time_stamp_utc
    }

    pub fn set_reason(&mut self, reason: String) {
        self.reason = Some(reason);
    }

    pub fn with_reason(mut self, reason: String) -> ValidationFailedChaosEvent {
        self.reason = Some(reason);
        self
    }

    pub fn reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    pub fn reset_reason(&mut self) {
        self.reason = None;
    }
}
