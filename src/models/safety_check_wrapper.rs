/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SafetyCheckWrapper : A wrapper for the safety check object. Safety checks are performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyCheckWrapper {
    /// Represents a safety check performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state.
    #[serde(rename = "SafetyCheck")]
    safety_check: Option<::models::SafetyCheck>,
}

impl Default for SafetyCheckWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyCheckWrapper {
    /// A wrapper for the safety check object. Safety checks are performed by service fabric before continuing with the operations. These checks ensure the availability of the service and the reliability of the state.
    pub fn new() -> SafetyCheckWrapper {
        SafetyCheckWrapper { safety_check: None }
    }

    pub fn set_safety_check(&mut self, safety_check: ::models::SafetyCheck) {
        self.safety_check = Some(safety_check);
    }

    pub fn with_safety_check(
        mut self,
        safety_check: ::models::SafetyCheck,
    ) -> SafetyCheckWrapper {
        self.safety_check = Some(safety_check);
        self
    }

    pub fn safety_check(&self) -> Option<&::models::SafetyCheck> {
        self.safety_check.as_ref()
    }

    pub fn reset_safety_check(&mut self) {
        self.safety_check = None;
    }
}
