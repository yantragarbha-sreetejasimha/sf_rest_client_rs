/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LoadMetricReportInfo : Information about load reported by replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadMetricReportInfo {
    /// The name of the metric.
    #[serde(rename = "Name")]
    name: Option<String>,
    /// The value of the load for the metric. In future releases of Service Fabric this parameter will be deprecated in favor of CurrentValue.
    #[serde(rename = "Value")]
    value: Option<i32>,
    /// The double value of the load for the metric.
    #[serde(rename = "CurrentValue")]
    current_value: Option<String>,
    /// The UTC time when the load is reported.
    #[serde(rename = "LastReportedUtc")]
    last_reported_utc: Option<String>,
}

impl Default for LoadMetricReportInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadMetricReportInfo {
    /// Information about load reported by replica.
    pub fn new() -> LoadMetricReportInfo {
        LoadMetricReportInfo {
            name: None,
            value: None,
            current_value: None,
            last_reported_utc: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> LoadMetricReportInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = Some(value);
    }

    pub fn with_value(mut self, value: i32) -> LoadMetricReportInfo {
        self.value = Some(value);
        self
    }

    pub fn value(&self) -> Option<&i32> {
        self.value.as_ref()
    }

    pub fn reset_value(&mut self) {
        self.value = None;
    }

    pub fn set_current_value(&mut self, current_value: String) {
        self.current_value = Some(current_value);
    }

    pub fn with_current_value(
        mut self,
        current_value: String,
    ) -> LoadMetricReportInfo {
        self.current_value = Some(current_value);
        self
    }

    pub fn current_value(&self) -> Option<&String> {
        self.current_value.as_ref()
    }

    pub fn reset_current_value(&mut self) {
        self.current_value = None;
    }

    pub fn set_last_reported_utc(&mut self, last_reported_utc: String) {
        self.last_reported_utc = Some(last_reported_utc);
    }

    pub fn with_last_reported_utc(
        mut self,
        last_reported_utc: String,
    ) -> LoadMetricReportInfo {
        self.last_reported_utc = Some(last_reported_utc);
        self
    }

    pub fn last_reported_utc(&self) -> Option<&String> {
        self.last_reported_utc.as_ref()
    }

    pub fn reset_last_reported_utc(&mut self) {
        self.last_reported_utc = None;
    }
}
