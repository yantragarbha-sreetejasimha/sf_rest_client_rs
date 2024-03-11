/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StoppedChaosEvent : Describes a Chaos event that gets generated when Chaos stops because either the user issued a stop or the time to run was up.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoppedChaosEvent {
    /// The kind of Chaos event.
    #[serde(rename = "Kind")]
    kind: ::models::ChaosEventKind,
    /// The UTC timestamp when this Chaos event was generated.
    #[serde(rename = "TimeStampUtc")]
    time_stamp_utc: String,
    /// Describes why Chaos stopped. Chaos can stop because of StopChaos API call or the timeToRun provided in ChaosParameters is over.
    #[serde(rename = "Reason")]
    reason: Option<String>,
}

impl StoppedChaosEvent {
    /// Describes a Chaos event that gets generated when Chaos stops because either the user issued a stop or the time to run was up.
    pub fn new(
        kind: ::models::ChaosEventKind,
        time_stamp_utc: String,
    ) -> StoppedChaosEvent {
        StoppedChaosEvent {
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
    ) -> StoppedChaosEvent {
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
    ) -> StoppedChaosEvent {
        self.time_stamp_utc = time_stamp_utc;
        self
    }

    pub fn time_stamp_utc(&self) -> &String {
        &self.time_stamp_utc
    }

    pub fn set_reason(&mut self, reason: String) {
        self.reason = Some(reason);
    }

    pub fn with_reason(mut self, reason: String) -> StoppedChaosEvent {
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