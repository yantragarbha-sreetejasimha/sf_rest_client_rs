/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeDomainNodesHealthEvaluation : Represents health evaluation for cluster nodes in an upgrade domain, containing health evaluations for each unhealthy node that impacted current aggregated health state. Can be returned when evaluating cluster health during cluster upgrade and the aggregated health state is either Error or Warning.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeDomainNodesHealthEvaluation {
    /// The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::HealthEvaluationKind,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Description of the health evaluation, which represents a summary of the evaluation process.
    #[serde(rename = "Description")]
    description: Option<String>,
    /// Name of the upgrade domain where nodes health is currently evaluated.
    #[serde(rename = "UpgradeDomainName")]
    upgrade_domain_name: Option<String>,
    /// Maximum allowed percentage of unhealthy nodes from the ClusterHealthPolicy.
    #[serde(rename = "MaxPercentUnhealthyNodes")]
    max_percent_unhealthy_nodes: Option<i32>,
    /// Total number of nodes in the current upgrade domain.
    #[serde(rename = "TotalCount")]
    total_count: Option<i64>,
    /// List of unhealthy evaluations that led to the aggregated health state. Includes all the unhealthy NodeHealthEvaluation that impacted the aggregated health.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
}

impl UpgradeDomainNodesHealthEvaluation {
    /// Represents health evaluation for cluster nodes in an upgrade domain, containing health evaluations for each unhealthy node that impacted current aggregated health state. Can be returned when evaluating cluster health during cluster upgrade and the aggregated health state is either Error or Warning.
    pub fn new(
        kind: ::models::HealthEvaluationKind,
    ) -> UpgradeDomainNodesHealthEvaluation {
        UpgradeDomainNodesHealthEvaluation {
            kind,
            aggregated_health_state: None,
            description: None,
            upgrade_domain_name: None,
            max_percent_unhealthy_nodes: None,
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
    ) -> UpgradeDomainNodesHealthEvaluation {
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
    ) -> UpgradeDomainNodesHealthEvaluation {
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
    ) -> UpgradeDomainNodesHealthEvaluation {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_upgrade_domain_name(&mut self, upgrade_domain_name: String) {
        self.upgrade_domain_name = Some(upgrade_domain_name);
    }

    pub fn with_upgrade_domain_name(
        mut self,
        upgrade_domain_name: String,
    ) -> UpgradeDomainNodesHealthEvaluation {
        self.upgrade_domain_name = Some(upgrade_domain_name);
        self
    }

    pub fn upgrade_domain_name(&self) -> Option<&String> {
        self.upgrade_domain_name.as_ref()
    }

    pub fn reset_upgrade_domain_name(&mut self) {
        self.upgrade_domain_name = None;
    }

    pub fn set_max_percent_unhealthy_nodes(
        &mut self,
        max_percent_unhealthy_nodes: i32,
    ) {
        self.max_percent_unhealthy_nodes = Some(max_percent_unhealthy_nodes);
    }

    pub fn with_max_percent_unhealthy_nodes(
        mut self,
        max_percent_unhealthy_nodes: i32,
    ) -> UpgradeDomainNodesHealthEvaluation {
        self.max_percent_unhealthy_nodes = Some(max_percent_unhealthy_nodes);
        self
    }

    pub fn max_percent_unhealthy_nodes(&self) -> Option<&i32> {
        self.max_percent_unhealthy_nodes.as_ref()
    }

    pub fn reset_max_percent_unhealthy_nodes(&mut self) {
        self.max_percent_unhealthy_nodes = None;
    }

    pub fn set_total_count(&mut self, total_count: i64) {
        self.total_count = Some(total_count);
    }

    pub fn with_total_count(
        mut self,
        total_count: i64,
    ) -> UpgradeDomainNodesHealthEvaluation {
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
    ) -> UpgradeDomainNodesHealthEvaluation {
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
