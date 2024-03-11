/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationMetricDescriptionList : List of application capacity metric description.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationMetricDescriptionList {}

impl Default for ApplicationMetricDescriptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationMetricDescriptionList {
    /// List of application capacity metric description.
    pub fn new() -> ApplicationMetricDescriptionList {
        ApplicationMetricDescriptionList {}
    }
}
