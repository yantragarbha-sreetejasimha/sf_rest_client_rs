/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationsHealthEvaluation : Represents health evaluation for deployed applications, containing health evaluations for each unhealthy deployed application that impacted current aggregated health state. Can be returned when evaluating application health and the aggregated health state is either Error or Warning.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationsHealthEvaluation {
    /// The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::HealthEvaluationKind,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Description of the health evaluation, which represents a summary of the evaluation process.
    #[serde(rename = "Description")]
    description: Option<String>,
    /// Maximum allowed percentage of unhealthy deployed applications from the ApplicationHealthPolicy.
    #[serde(rename = "MaxPercentUnhealthyDeployedApplications")]
    max_percent_unhealthy_deployed_applications: Option<i32>,
    /// Total number of deployed applications of the application in the health store.
    #[serde(rename = "TotalCount")]
    total_count: Option<i64>,
    /// List of unhealthy evaluations that led to the aggregated health state. Includes all the unhealthy DeployedApplicationHealthEvaluation that impacted the aggregated health.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
}

impl DeployedApplicationsHealthEvaluation {
    /// Represents health evaluation for deployed applications, containing health evaluations for each unhealthy deployed application that impacted current aggregated health state. Can be returned when evaluating application health and the aggregated health state is either Error or Warning.
    pub fn new(
        kind: ::models::HealthEvaluationKind,
    ) -> DeployedApplicationsHealthEvaluation {
        DeployedApplicationsHealthEvaluation {
            kind,
            aggregated_health_state: None,
            description: None,
            max_percent_unhealthy_deployed_applications: None,
            total_count: None,
            unhealthy_evaluations: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::HealthEvaluationKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::HealthEvaluationKind,
    ) -> DeployedApplicationsHealthEvaluation {
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
    ) -> DeployedApplicationsHealthEvaluation {
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
    ) -> DeployedApplicationsHealthEvaluation {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_max_percent_unhealthy_deployed_applications(
        &mut self,
        max_percent_unhealthy_deployed_applications: i32,
    ) {
        self.max_percent_unhealthy_deployed_applications =
            Some(max_percent_unhealthy_deployed_applications);
    }

    pub fn with_max_percent_unhealthy_deployed_applications(
        mut self,
        max_percent_unhealthy_deployed_applications: i32,
    ) -> DeployedApplicationsHealthEvaluation {
        self.max_percent_unhealthy_deployed_applications =
            Some(max_percent_unhealthy_deployed_applications);
        self
    }

    pub fn max_percent_unhealthy_deployed_applications(&self) -> Option<&i32> {
        self.max_percent_unhealthy_deployed_applications.as_ref()
    }

    pub fn reset_max_percent_unhealthy_deployed_applications(&mut self) {
        self.max_percent_unhealthy_deployed_applications = None;
    }

    pub fn set_total_count(&mut self, total_count: i64) {
        self.total_count = Some(total_count);
    }

    pub fn with_total_count(
        mut self,
        total_count: i64,
    ) -> DeployedApplicationsHealthEvaluation {
        self.total_count = Some(total_count);
        self
    }

    pub fn total_count(&self) -> Option<&i64> {
        self.total_count.as_ref()
    }

    pub fn reset_total_count(&mut self) {
        self.total_count = None;
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
    ) -> DeployedApplicationsHealthEvaluation {
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
