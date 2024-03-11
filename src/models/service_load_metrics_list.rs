/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceLoadMetricsList : The service load metrics is given as an array of ServiceLoadMetricDescription objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceLoadMetricsList {}

impl Default for ServiceLoadMetricsList {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceLoadMetricsList {
    /// The service load metrics is given as an array of ServiceLoadMetricDescription objects.
    pub fn new() -> ServiceLoadMetricsList {
        ServiceLoadMetricsList {}
    }
}