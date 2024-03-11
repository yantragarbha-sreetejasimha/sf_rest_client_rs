/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReconfigurationInformation : Information about current reconfiguration like phase, type, previous configuration role of replica and reconfiguration start date time.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconfigurationInformation {
    /// Replica role before reconfiguration started.
    #[serde(rename = "PreviousConfigurationRole")]
    previous_configuration_role: Option<::models::ReplicaRole>,
    /// Current phase of ongoing reconfiguration. If no reconfiguration is taking place then this value will be \"None\".
    #[serde(rename = "ReconfigurationPhase")]
    reconfiguration_phase: Option<::models::ReconfigurationPhase>,
    /// Type of current ongoing reconfiguration. If no reconfiguration is taking place then this value will be \"None\".
    #[serde(rename = "ReconfigurationType")]
    reconfiguration_type: Option<::models::ReconfigurationType>,
    /// Start time (in UTC) of the ongoing reconfiguration. If no reconfiguration is taking place then this value will be zero date-time.
    #[serde(rename = "ReconfigurationStartTimeUtc")]
    reconfiguration_start_time_utc: Option<String>,
}

impl Default for ReconfigurationInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl ReconfigurationInformation {
    /// Information about current reconfiguration like phase, type, previous configuration role of replica and reconfiguration start date time.
    pub fn new() -> ReconfigurationInformation {
        ReconfigurationInformation {
            previous_configuration_role: None,
            reconfiguration_phase: None,
            reconfiguration_type: None,
            reconfiguration_start_time_utc: None,
        }
    }

    pub fn set_previous_configuration_role(
        &mut self,
        previous_configuration_role: ::models::ReplicaRole,
    ) {
        self.previous_configuration_role = Some(previous_configuration_role);
    }

    pub fn with_previous_configuration_role(
        mut self,
        previous_configuration_role: ::models::ReplicaRole,
    ) -> ReconfigurationInformation {
        self.previous_configuration_role = Some(previous_configuration_role);
        self
    }

    pub fn previous_configuration_role(
        &self,
    ) -> Option<&::models::ReplicaRole> {
        self.previous_configuration_role.as_ref()
    }

    pub fn reset_previous_configuration_role(&mut self) {
        self.previous_configuration_role = None;
    }

    pub fn set_reconfiguration_phase(
        &mut self,
        reconfiguration_phase: ::models::ReconfigurationPhase,
    ) {
        self.reconfiguration_phase = Some(reconfiguration_phase);
    }

    pub fn with_reconfiguration_phase(
        mut self,
        reconfiguration_phase: ::models::ReconfigurationPhase,
    ) -> ReconfigurationInformation {
        self.reconfiguration_phase = Some(reconfiguration_phase);
        self
    }

    pub fn reconfiguration_phase(
        &self,
    ) -> Option<&::models::ReconfigurationPhase> {
        self.reconfiguration_phase.as_ref()
    }

    pub fn reset_reconfiguration_phase(&mut self) {
        self.reconfiguration_phase = None;
    }

    pub fn set_reconfiguration_type(
        &mut self,
        reconfiguration_type: ::models::ReconfigurationType,
    ) {
        self.reconfiguration_type = Some(reconfiguration_type);
    }

    pub fn with_reconfiguration_type(
        mut self,
        reconfiguration_type: ::models::ReconfigurationType,
    ) -> ReconfigurationInformation {
        self.reconfiguration_type = Some(reconfiguration_type);
        self
    }

    pub fn reconfiguration_type(
        &self,
    ) -> Option<&::models::ReconfigurationType> {
        self.reconfiguration_type.as_ref()
    }

    pub fn reset_reconfiguration_type(&mut self) {
        self.reconfiguration_type = None;
    }

    pub fn set_reconfiguration_start_time_utc(
        &mut self,
        reconfiguration_start_time_utc: String,
    ) {
        self.reconfiguration_start_time_utc =
            Some(reconfiguration_start_time_utc);
    }

    pub fn with_reconfiguration_start_time_utc(
        mut self,
        reconfiguration_start_time_utc: String,
    ) -> ReconfigurationInformation {
        self.reconfiguration_start_time_utc =
            Some(reconfiguration_start_time_utc);
        self
    }

    pub fn reconfiguration_start_time_utc(&self) -> Option<&String> {
        self.reconfiguration_start_time_utc.as_ref()
    }

    pub fn reset_reconfiguration_start_time_utc(&mut self) {
        self.reconfiguration_start_time_utc = None;
    }
}
