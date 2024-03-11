/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HealthEvaluationKind : The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthEvaluationKind {}

impl Default for HealthEvaluationKind {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthEvaluationKind {
    /// The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.
    pub fn new() -> HealthEvaluationKind {
        HealthEvaluationKind {}
    }
}

// TODO enum
// List of HealthEvaluationKind
//const (
//
//
//
//)
