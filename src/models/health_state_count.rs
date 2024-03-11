/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HealthStateCount : Represents information about how many health entities are in Ok, Warning and Error health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStateCount {
    /// The number of health entities with aggregated health state Ok.
    #[serde(rename = "OkCount")]
    ok_count: Option<i64>,
    /// The number of health entities with aggregated health state Warning.
    #[serde(rename = "WarningCount")]
    warning_count: Option<i64>,
    /// The number of health entities with aggregated health state Error.
    #[serde(rename = "ErrorCount")]
    error_count: Option<i64>,
}

impl Default for HealthStateCount {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthStateCount {
    /// Represents information about how many health entities are in Ok, Warning and Error health state.
    pub fn new() -> HealthStateCount {
        HealthStateCount {
            ok_count: None,
            warning_count: None,
            error_count: None,
        }
    }

    pub fn set_ok_count(&mut self, ok_count: i64) {
        self.ok_count = Some(ok_count);
    }

    pub fn with_ok_count(mut self, ok_count: i64) -> HealthStateCount {
        self.ok_count = Some(ok_count);
        self
    }

    pub fn ok_count(&self) -> Option<&i64> {
        self.ok_count.as_ref()
    }

    pub fn reset_ok_count(&mut self) {
        self.ok_count = None;
    }

    pub fn set_warning_count(&mut self, warning_count: i64) {
        self.warning_count = Some(warning_count);
    }

    pub fn with_warning_count(
        mut self,
        warning_count: i64,
    ) -> HealthStateCount {
        self.warning_count = Some(warning_count);
        self
    }

    pub fn warning_count(&self) -> Option<&i64> {
        self.warning_count.as_ref()
    }

    pub fn reset_warning_count(&mut self) {
        self.warning_count = None;
    }

    pub fn set_error_count(&mut self, error_count: i64) {
        self.error_count = Some(error_count);
    }

    pub fn with_error_count(mut self, error_count: i64) -> HealthStateCount {
        self.error_count = Some(error_count);
        self
    }

    pub fn error_count(&self) -> Option<&i64> {
        self.error_count.as_ref()
    }

    pub fn reset_error_count(&mut self) {
        self.error_count = None;
    }
}
