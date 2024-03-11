/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UnhealthyEvaluations : List of health evaluations that resulted in the current aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnhealthyEvaluations {}

impl Default for UnhealthyEvaluations {
    fn default() -> Self {
        Self::new()
    }
}

impl UnhealthyEvaluations {
    /// List of health evaluations that resulted in the current aggregated health state.
    pub fn new() -> UnhealthyEvaluations {
        UnhealthyEvaluations {}
    }
}
