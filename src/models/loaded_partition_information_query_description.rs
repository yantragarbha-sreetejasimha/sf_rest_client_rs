/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LoadedPartitionInformationQueryDescription : Represents data structure that contains query information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadedPartitionInformationQueryDescription {
    /// Name of the metric for which this information is provided.
    #[serde(rename = "MetricName")]
    metric_name: Option<String>,
    /// Name of the service this partition belongs to.
    #[serde(rename = "ServiceName")]
    service_name: Option<String>,
    /// Ordering of partitions' load.
    #[serde(rename = "Ordering")]
    ordering: Option<::models::Ordering>,
    /// The maximum number of results to be returned as part of the paged queries. This parameter defines the upper bound on the number of results returned. The results returned can be less than the specified maximum results if they do not fit in the message as per the max message size restrictions defined in the configuration. If this parameter is zero or not specified, the paged query includes as many results as possible that fit in the return message.
    #[serde(rename = "MaxResults")]
    max_results: Option<i64>,
    /// The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response.
    #[serde(rename = "ContinuationToken")]
    continuation_token: Option<::models::ContinuationToken>,
}

impl Default for LoadedPartitionInformationQueryDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadedPartitionInformationQueryDescription {
    /// Represents data structure that contains query information.
    pub fn new() -> LoadedPartitionInformationQueryDescription {
        LoadedPartitionInformationQueryDescription {
            metric_name: None,
            service_name: None,
            ordering: None,
            max_results: None,
            continuation_token: None,
        }
    }

    pub fn set_metric_name(&mut self, metric_name: String) {
        self.metric_name = Some(metric_name);
    }

    pub fn with_metric_name(
        mut self,
        metric_name: String,
    ) -> LoadedPartitionInformationQueryDescription {
        self.metric_name = Some(metric_name);
        self
    }

    pub fn metric_name(&self) -> Option<&String> {
        self.metric_name.as_ref()
    }

    pub fn reset_metric_name(&mut self) {
        self.metric_name = None;
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = Some(service_name);
    }

    pub fn with_service_name(
        mut self,
        service_name: String,
    ) -> LoadedPartitionInformationQueryDescription {
        self.service_name = Some(service_name);
        self
    }

    pub fn service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    pub fn reset_service_name(&mut self) {
        self.service_name = None;
    }

    pub fn set_ordering(&mut self, ordering: ::models::Ordering) {
        self.ordering = Some(ordering);
    }

    pub fn with_ordering(
        mut self,
        ordering: ::models::Ordering,
    ) -> LoadedPartitionInformationQueryDescription {
        self.ordering = Some(ordering);
        self
    }

    pub fn ordering(&self) -> Option<&::models::Ordering> {
        self.ordering.as_ref()
    }

    pub fn reset_ordering(&mut self) {
        self.ordering = None;
    }

    pub fn set_max_results(&mut self, max_results: i64) {
        self.max_results = Some(max_results);
    }

    pub fn with_max_results(
        mut self,
        max_results: i64,
    ) -> LoadedPartitionInformationQueryDescription {
        self.max_results = Some(max_results);
        self
    }

    pub fn max_results(&self) -> Option<&i64> {
        self.max_results.as_ref()
    }

    pub fn reset_max_results(&mut self) {
        self.max_results = None;
    }

    pub fn set_continuation_token(
        &mut self,
        continuation_token: ::models::ContinuationToken,
    ) {
        self.continuation_token = Some(continuation_token);
    }

    pub fn with_continuation_token(
        mut self,
        continuation_token: ::models::ContinuationToken,
    ) -> LoadedPartitionInformationQueryDescription {
        self.continuation_token = Some(continuation_token);
        self
    }

    pub fn continuation_token(&self) -> Option<&::models::ContinuationToken> {
        self.continuation_token.as_ref()
    }

    pub fn reset_continuation_token(&mut self) {
        self.continuation_token = None;
    }
}