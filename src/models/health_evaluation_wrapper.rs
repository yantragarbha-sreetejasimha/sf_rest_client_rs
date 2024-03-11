/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HealthEvaluationWrapper : Wrapper object for health evaluation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthEvaluationWrapper {
    /// Represents a health evaluation which describes the data and the algorithm used by health manager to evaluate the health of an entity.
    #[serde(rename = "HealthEvaluation")]
    health_evaluation: Option<::models::HealthEvaluation>,
}

impl Default for HealthEvaluationWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthEvaluationWrapper {
    /// Wrapper object for health evaluation.
    pub fn new() -> HealthEvaluationWrapper {
        HealthEvaluationWrapper {
            health_evaluation: None,
        }
    }

    pub fn set_health_evaluation(
        &mut self,
        health_evaluation: ::models::HealthEvaluation,
    ) {
        self.health_evaluation = Some(health_evaluation);
    }

    pub fn with_health_evaluation(
        mut self,
        health_evaluation: ::models::HealthEvaluation,
    ) -> HealthEvaluationWrapper {
        self.health_evaluation = Some(health_evaluation);
        self
    }

    pub fn health_evaluation(&self) -> Option<&::models::HealthEvaluation> {
        self.health_evaluation.as_ref()
    }

    pub fn reset_health_evaluation(&mut self) {
        self.health_evaluation = None;
    }
}
