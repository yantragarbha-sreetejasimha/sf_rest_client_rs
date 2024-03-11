/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterLoadInfo : Information about load in a Service Fabric cluster. It holds a summary of all metrics and their load in a cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterLoadInfo {
    /// The starting time of last resource balancing run.
    #[serde(rename = "LastBalancingStartTimeUtc")]
    last_balancing_start_time_utc: Option<String>,
    /// The end time of last resource balancing run.
    #[serde(rename = "LastBalancingEndTimeUtc")]
    last_balancing_end_time_utc: Option<String>,
    /// List that contains metrics and their load information in this cluster.
    #[serde(rename = "LoadMetricInformation")]
    load_metric_information: Option<Vec<::models::LoadMetricInformation>>,
}

impl Default for ClusterLoadInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterLoadInfo {
    /// Information about load in a Service Fabric cluster. It holds a summary of all metrics and their load in a cluster.
    pub fn new() -> ClusterLoadInfo {
        ClusterLoadInfo {
            last_balancing_start_time_utc: None,
            last_balancing_end_time_utc: None,
            load_metric_information: None,
        }
    }

    pub fn set_last_balancing_start_time_utc(
        &mut self,
        last_balancing_start_time_utc: String,
    ) {
        self.last_balancing_start_time_utc =
            Some(last_balancing_start_time_utc);
    }

    pub fn with_last_balancing_start_time_utc(
        mut self,
        last_balancing_start_time_utc: String,
    ) -> ClusterLoadInfo {
        self.last_balancing_start_time_utc =
            Some(last_balancing_start_time_utc);
        self
    }

    pub fn last_balancing_start_time_utc(&self) -> Option<&String> {
        self.last_balancing_start_time_utc.as_ref()
    }

    pub fn reset_last_balancing_start_time_utc(&mut self) {
        self.last_balancing_start_time_utc = None;
    }

    pub fn set_last_balancing_end_time_utc(
        &mut self,
        last_balancing_end_time_utc: String,
    ) {
        self.last_balancing_end_time_utc = Some(last_balancing_end_time_utc);
    }

    pub fn with_last_balancing_end_time_utc(
        mut self,
        last_balancing_end_time_utc: String,
    ) -> ClusterLoadInfo {
        self.last_balancing_end_time_utc = Some(last_balancing_end_time_utc);
        self
    }

    pub fn last_balancing_end_time_utc(&self) -> Option<&String> {
        self.last_balancing_end_time_utc.as_ref()
    }

    pub fn reset_last_balancing_end_time_utc(&mut self) {
        self.last_balancing_end_time_utc = None;
    }

    pub fn set_load_metric_information(
        &mut self,
        load_metric_information: Vec<::models::LoadMetricInformation>,
    ) {
        self.load_metric_information = Some(load_metric_information);
    }

    pub fn with_load_metric_information(
        mut self,
        load_metric_information: Vec<::models::LoadMetricInformation>,
    ) -> ClusterLoadInfo {
        self.load_metric_information = Some(load_metric_information);
        self
    }

    pub fn load_metric_information(
        &self,
    ) -> Option<&Vec<::models::LoadMetricInformation>> {
        self.load_metric_information.as_ref()
    }

    pub fn reset_load_metric_information(&mut self) {
        self.load_metric_information = None;
    }
}
