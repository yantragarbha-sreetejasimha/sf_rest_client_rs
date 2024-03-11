/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeltaHealthEvaluationBool : When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeltaHealthEvaluationBool {}

impl Default for DeltaHealthEvaluationBool {
    fn default() -> Self {
        Self::new()
    }
}

impl DeltaHealthEvaluationBool {
    /// When true, enables delta health evaluation rather than absolute health evaluation after completion of each upgrade domain.
    pub fn new() -> DeltaHealthEvaluationBool {
        DeltaHealthEvaluationBool {}
    }
}