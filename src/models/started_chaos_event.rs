/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StartedChaosEvent : Describes a Chaos event that gets generated when Chaos is started.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StartedChaosEvent {
    /// The kind of Chaos event.
    #[serde(rename = "Kind")]
    kind: ::models::ChaosEventKind,
    /// The UTC timestamp when this Chaos event was generated.
    #[serde(rename = "TimeStampUtc")]
    time_stamp_utc: String,
    /// Defines all the parameters to configure a Chaos run.
    #[serde(rename = "ChaosParameters")]
    chaos_parameters: Option<::models::ChaosParameters>,
}

impl StartedChaosEvent {
    /// Describes a Chaos event that gets generated when Chaos is started.
    pub fn new(
        kind: ::models::ChaosEventKind,
        time_stamp_utc: String,
    ) -> StartedChaosEvent {
        StartedChaosEvent {
            kind,
            time_stamp_utc,
            chaos_parameters: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ChaosEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ChaosEventKind,
    ) -> StartedChaosEvent {
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
    ) -> StartedChaosEvent {
        self.time_stamp_utc = time_stamp_utc;
        self
    }

    pub fn time_stamp_utc(&self) -> &String {
        &self.time_stamp_utc
    }

    pub fn set_chaos_parameters(
        &mut self,
        chaos_parameters: ::models::ChaosParameters,
    ) {
        self.chaos_parameters = Some(chaos_parameters);
    }

    pub fn with_chaos_parameters(
        mut self,
        chaos_parameters: ::models::ChaosParameters,
    ) -> StartedChaosEvent {
        self.chaos_parameters = Some(chaos_parameters);
        self
    }

    pub fn chaos_parameters(&self) -> Option<&::models::ChaosParameters> {
        self.chaos_parameters.as_ref()
    }

    pub fn reset_chaos_parameters(&mut self) {
        self.chaos_parameters = None;
    }
}
