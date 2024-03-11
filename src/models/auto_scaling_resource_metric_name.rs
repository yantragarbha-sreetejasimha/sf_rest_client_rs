/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoScalingResourceMetricName : Enumerates the resources that are used for triggering auto scaling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoScalingResourceMetricName {}

impl Default for AutoScalingResourceMetricName {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoScalingResourceMetricName {
    /// Enumerates the resources that are used for triggering auto scaling.
    pub fn new() -> AutoScalingResourceMetricName {
        AutoScalingResourceMetricName {}
    }
}

// TODO enum
// List of AutoScalingResourceMetricName
//const (
//
//
//
//)
