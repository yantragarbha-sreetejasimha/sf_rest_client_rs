/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationHealthEvaluation : Represents health evaluation for an application, containing information about the data and the algorithm used by the health store to evaluate health.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationHealthEvaluation {
    /// The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::HealthEvaluationKind,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Description of the health evaluation, which represents a summary of the evaluation process.
    #[serde(rename = "Description")]
    description: Option<String>,
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "ApplicationName")]
    application_name: Option<::models::ApplicationName>,
    /// List of unhealthy evaluations that led to the current aggregated health state of the application. The types of the unhealthy evaluations can be DeployedApplicationsHealthEvaluation, ServicesHealthEvaluation or EventHealthEvaluation.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
}

impl ApplicationHealthEvaluation {
    /// Represents health evaluation for an application, containing information about the data and the algorithm used by the health store to evaluate health.
    pub fn new(
        kind: ::models::HealthEvaluationKind,
    ) -> ApplicationHealthEvaluation {
        ApplicationHealthEvaluation {
            kind,
            aggregated_health_state: None,
            description: None,
            application_name: None,
            unhealthy_evaluations: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::HealthEvaluationKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::HealthEvaluationKind,
    ) -> ApplicationHealthEvaluation {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::HealthEvaluationKind {
        &self.kind
    }

    pub fn set_aggregated_health_state(
        &mut self,
        aggregated_health_state: ::models::HealthState,
    ) {
        self.aggregated_health_state = Some(aggregated_health_state);
    }

    pub fn with_aggregated_health_state(
        mut self,
        aggregated_health_state: ::models::HealthState,
    ) -> ApplicationHealthEvaluation {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> ApplicationHealthEvaluation {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_application_name(
        &mut self,
        application_name: ::models::ApplicationName,
    ) {
        self.application_name = Some(application_name);
    }

    pub fn with_application_name(
        mut self,
        application_name: ::models::ApplicationName,
    ) -> ApplicationHealthEvaluation {
        self.application_name = Some(application_name);
        self
    }

    pub fn application_name(&self) -> Option<&::models::ApplicationName> {
        self.application_name.as_ref()
    }

    pub fn reset_application_name(&mut self) {
        self.application_name = None;
    }

    pub fn set_unhealthy_evaluations(
        &mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
    }

    pub fn with_unhealthy_evaluations(
        mut self,
        unhealthy_evaluations: ::models::UnhealthyEvaluations,
    ) -> ApplicationHealthEvaluation {
        self.unhealthy_evaluations = Some(unhealthy_evaluations);
        self
    }

    pub fn unhealthy_evaluations(
        &self,
    ) -> Option<&::models::UnhealthyEvaluations> {
        self.unhealthy_evaluations.as_ref()
    }

    pub fn reset_unhealthy_evaluations(&mut self) {
        self.unhealthy_evaluations = None;
    }
}
