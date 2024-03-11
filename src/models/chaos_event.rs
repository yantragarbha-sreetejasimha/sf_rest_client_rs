/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosEvent : Represents an event generated during a Chaos run.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosEvent {
    /// The kind of Chaos event.
    #[serde(rename = "Kind")]
    kind: ::models::ChaosEventKind,
    /// The UTC timestamp when this Chaos event was generated.
    #[serde(rename = "TimeStampUtc")]
    time_stamp_utc: String,
}

impl ChaosEvent {
    /// Represents an event generated during a Chaos run.
    pub fn new(
        kind: ::models::ChaosEventKind,
        time_stamp_utc: String,
    ) -> ChaosEvent {
        ChaosEvent {
            kind,
            time_stamp_utc,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ChaosEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(mut self, kind: ::models::ChaosEventKind) -> ChaosEvent {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ChaosEventKind {
        &self.kind
    }

    pub fn set_time_stamp_utc(&mut self, time_stamp_utc: String) {
        self.time_stamp_utc = time_stamp_utc;
    }

    pub fn with_time_stamp_utc(mut self, time_stamp_utc: String) -> ChaosEvent {
        self.time_stamp_utc = time_stamp_utc;
        self
    }

    pub fn time_stamp_utc(&self) -> &String {
        &self.time_stamp_utc
    }
}
