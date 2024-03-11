/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeApplicationsHealthEvaluation : Represents health evaluation for applications of a particular application type. The application type applications evaluation can be returned when cluster health evaluation returns unhealthy aggregated health state, either Error or Warning. It contains health evaluations for each unhealthy application of the included application type that impacted current aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeApplicationsHealthEvaluation {
    /// The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::HealthEvaluationKind,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Description of the health evaluation, which represents a summary of the evaluation process.
    #[serde(rename = "Description")]
    description: Option<String>,
    /// The application type name as defined in the application manifest.
    #[serde(rename = "ApplicationTypeName")]
    application_type_name: Option<::models::ApplicationTypeName>,
    /// Maximum allowed percentage of unhealthy applications for the application type, specified as an entry in ApplicationTypeHealthPolicyMap.
    #[serde(rename = "MaxPercentUnhealthyApplications")]
    max_percent_unhealthy_applications: Option<i32>,
    /// Total number of applications of the application type found in the health store.
    #[serde(rename = "TotalCount")]
    total_count: Option<i64>,
    /// List of unhealthy evaluations that led to the aggregated health state. Includes all the unhealthy ApplicationHealthEvaluation of this application type that impacted the aggregated health.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
}

impl ApplicationTypeApplicationsHealthEvaluation {
    /// Represents health evaluation for applications of a particular application type. The application type applications evaluation can be returned when cluster health evaluation returns unhealthy aggregated health state, either Error or Warning. It contains health evaluations for each unhealthy application of the included application type that impacted current aggregated health state.
    pub fn new(
        kind: ::models::HealthEvaluationKind,
    ) -> ApplicationTypeApplicationsHealthEvaluation {
        ApplicationTypeApplicationsHealthEvaluation {
            kind,
            aggregated_health_state: None,
            description: None,
            application_type_name: None,
            max_percent_unhealthy_applications: None,
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
    ) -> ApplicationTypeApplicationsHealthEvaluation {
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
    ) -> ApplicationTypeApplicationsHealthEvaluation {
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
    ) -> ApplicationTypeApplicationsHealthEvaluation {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_application_type_name(
        &mut self,
        application_type_name: ::models::ApplicationTypeName,
    ) {
        self.application_type_name = Some(application_type_name);
    }

    pub fn with_application_type_name(
        mut self,
        application_type_name: ::models::ApplicationTypeName,
    ) -> ApplicationTypeApplicationsHealthEvaluation {
        self.application_type_name = Some(application_type_name);
        self
    }

    pub fn application_type_name(
        &self,
    ) -> Option<&::models::ApplicationTypeName> {
        self.application_type_name.as_ref()
    }

    pub fn reset_application_type_name(&mut self) {
        self.application_type_name = None;
    }

    pub fn set_max_percent_unhealthy_applications(
        &mut self,
        max_percent_unhealthy_applications: i32,
    ) {
        self.max_percent_unhealthy_applications =
            Some(max_percent_unhealthy_applications);
    }

    pub fn with_max_percent_unhealthy_applications(
        mut self,
        max_percent_unhealthy_applications: i32,
    ) -> ApplicationTypeApplicationsHealthEvaluation {
        self.max_percent_unhealthy_applications =
            Some(max_percent_unhealthy_applications);
        self
    }

    pub fn max_percent_unhealthy_applications(&self) -> Option<&i32> {
        self.max_percent_unhealthy_applications.as_ref()
    }

    pub fn reset_max_percent_unhealthy_applications(&mut self) {
        self.max_percent_unhealthy_applications = None;
    }

    pub fn set_total_count(&mut self, total_count: i64) {
        self.total_count = Some(total_count);
    }

    pub fn with_total_count(
        mut self,
        total_count: i64,
    ) -> ApplicationTypeApplicationsHealthEvaluation {
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
    ) -> ApplicationTypeApplicationsHealthEvaluation {
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
