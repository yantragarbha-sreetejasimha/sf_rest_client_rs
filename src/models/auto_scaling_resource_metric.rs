/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoScalingResourceMetric : Describes the resource that is used for triggering auto scaling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoScalingResourceMetric {
    /// The type of auto scaling metric
    #[serde(rename = "kind")]
    kind: ::models::AutoScalingMetricKind,
    /// Name of the resource.
    #[serde(rename = "name")]
    name: ::models::AutoScalingResourceMetricName,
}

impl AutoScalingResourceMetric {
    /// Describes the resource that is used for triggering auto scaling.
    pub fn new(
        kind: ::models::AutoScalingMetricKind,
        name: ::models::AutoScalingResourceMetricName,
    ) -> AutoScalingResourceMetric {
        AutoScalingResourceMetric { kind, name }
    }

    pub fn set_kind(&mut self, kind: ::models::AutoScalingMetricKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::AutoScalingMetricKind,
    ) -> AutoScalingResourceMetric {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::AutoScalingMetricKind {
        &self.kind
    }

    pub fn set_name(&mut self, name: ::models::AutoScalingResourceMetricName) {
        self.name = name;
    }

    pub fn with_name(
        mut self,
        name: ::models::AutoScalingResourceMetricName,
    ) -> AutoScalingResourceMetric {
        self.name = name;
        self
    }

    pub fn name(&self) -> &::models::AutoScalingResourceMetricName {
        &self.name
    }
}
