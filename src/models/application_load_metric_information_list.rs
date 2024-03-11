/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationLoadMetricInformationList : List of application load metric information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationLoadMetricInformationList {}

impl Default for ApplicationLoadMetricInformationList {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationLoadMetricInformationList {
    /// List of application load metric information.
    pub fn new() -> ApplicationLoadMetricInformationList {
        ApplicationLoadMetricInformationList {}
    }
}
