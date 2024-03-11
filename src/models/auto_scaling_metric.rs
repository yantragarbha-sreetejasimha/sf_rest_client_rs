/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoScalingMetric : Describes the metric that is used for triggering auto scaling operation. Derived classes will describe resources or metrics.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoScalingMetric {
    /// The type of auto scaling metric
    #[serde(rename = "kind")]
    kind: ::models::AutoScalingMetricKind,
}

impl AutoScalingMetric {
    /// Describes the metric that is used for triggering auto scaling operation. Derived classes will describe resources or metrics.
    pub fn new(kind: ::models::AutoScalingMetricKind) -> AutoScalingMetric {
        AutoScalingMetric { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::AutoScalingMetricKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::AutoScalingMetricKind,
    ) -> AutoScalingMetric {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::AutoScalingMetricKind {
        &self.kind
    }
}