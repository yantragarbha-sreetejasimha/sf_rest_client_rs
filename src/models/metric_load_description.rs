/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MetricLoadDescription : Specifies metric load information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricLoadDescription {
    /// The name of the reported metric.
    #[serde(rename = "MetricName")]
    metric_name: Option<String>,
    /// The current value of the metric load.
    #[serde(rename = "CurrentLoad")]
    current_load: Option<i64>,
    /// The predicted value of the metric load.
    #[serde(rename = "PredictedLoad")]
    predicted_load: Option<i64>,
}

impl Default for MetricLoadDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricLoadDescription {
    /// Specifies metric load information.
    pub fn new() -> MetricLoadDescription {
        MetricLoadDescription {
            metric_name: None,
            current_load: None,
            predicted_load: None,
        }
    }

    pub fn set_metric_name(&mut self, metric_name: String) {
        self.metric_name = Some(metric_name);
    }

    pub fn with_metric_name(
        mut self,
        metric_name: String,
    ) -> MetricLoadDescription {
        self.metric_name = Some(metric_name);
        self
    }

    pub fn metric_name(&self) -> Option<&String> {
        self.metric_name.as_ref()
    }

    pub fn reset_metric_name(&mut self) {
        self.metric_name = None;
    }

    pub fn set_current_load(&mut self, current_load: i64) {
        self.current_load = Some(current_load);
    }

    pub fn with_current_load(
        mut self,
        current_load: i64,
    ) -> MetricLoadDescription {
        self.current_load = Some(current_load);
        self
    }

    pub fn current_load(&self) -> Option<&i64> {
        self.current_load.as_ref()
    }

    pub fn reset_current_load(&mut self) {
        self.current_load = None;
    }

    pub fn set_predicted_load(&mut self, predicted_load: i64) {
        self.predicted_load = Some(predicted_load);
    }

    pub fn with_predicted_load(
        mut self,
        predicted_load: i64,
    ) -> MetricLoadDescription {
        self.predicted_load = Some(predicted_load);
        self
    }

    pub fn predicted_load(&self) -> Option<&i64> {
        self.predicted_load.as_ref()
    }

    pub fn reset_predicted_load(&mut self) {
        self.predicted_load = None;
    }
}
