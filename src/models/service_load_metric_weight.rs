/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceLoadMetricWeight : Determines the metric weight relative to the other metrics that are configured for this service. During runtime, if two metrics end up in conflict, the Cluster Resource Manager prefers the metric with the higher weight.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceLoadMetricWeight {}

impl Default for ServiceLoadMetricWeight {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceLoadMetricWeight {
    /// Determines the metric weight relative to the other metrics that are configured for this service. During runtime, if two metrics end up in conflict, the Cluster Resource Manager prefers the metric with the higher weight.
    pub fn new() -> ServiceLoadMetricWeight {
        ServiceLoadMetricWeight {}
    }
}

// TODO enum
// List of ServiceLoadMetricWeight
//const (
//
//
//
//)
