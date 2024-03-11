/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeTypeNodesHealthEvaluation : Represents health evaluation for nodes of a particular node type. The node type nodes evaluation can be returned when cluster health evaluation returns unhealthy aggregated health state, either Error or Warning. It contains health evaluations for each unhealthy node of the included node type that impacted current aggregated health state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTypeNodesHealthEvaluation {
    /// The health manager in the cluster performs health evaluations in determining the aggregated health state of an entity. This enumeration provides information on the kind of evaluation that was performed. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::HealthEvaluationKind,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// Description of the health evaluation, which represents a summary of the evaluation process.
    #[serde(rename = "Description")]
    description: Option<String>,
    /// The node type name as defined in the cluster manifest.
    #[serde(rename = "NodeTypeName")]
    node_type_name: Option<::models::NodeTypeName>,
    /// Maximum allowed percentage of unhealthy nodes for the node type, specified as an entry in NodeTypeHealthPolicyMap.
    #[serde(rename = "MaxPercentUnhealthyNodes")]
    max_percent_unhealthy_nodes: Option<i32>,
    /// Total number of nodes of the node type found in the health store.
    #[serde(rename = "TotalCount")]
    total_count: Option<i64>,
    /// List of unhealthy evaluations that led to the aggregated health state. Includes all the unhealthy NodeHealthEvaluation of this node type that impacted the aggregated health.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
}

impl NodeTypeNodesHealthEvaluation {
    /// Represents health evaluation for nodes of a particular node type. The node type nodes evaluation can be returned when cluster health evaluation returns unhealthy aggregated health state, either Error or Warning. It contains health evaluations for each unhealthy node of the included node type that impacted current aggregated health state.
    pub fn new(
        kind: ::models::HealthEvaluationKind,
    ) -> NodeTypeNodesHealthEvaluation {
        NodeTypeNodesHealthEvaluation {
            kind,
            aggregated_health_state: None,
            description: None,
            node_type_name: None,
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
    ) -> NodeTypeNodesHealthEvaluation {
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
    ) -> NodeTypeNodesHealthEvaluation {
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
    ) -> NodeTypeNodesHealthEvaluation {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_node_type_name(
        &mut self,
        node_type_name: ::models::NodeTypeName,
    ) {
        self.node_type_name = Some(node_type_name);
    }

    pub fn with_node_type_name(
        mut self,
        node_type_name: ::models::NodeTypeName,
    ) -> NodeTypeNodesHealthEvaluation {
        self.node_type_name = Some(node_type_name);
        self
    }

    pub fn node_type_name(&self) -> Option<&::models::NodeTypeName> {
        self.node_type_name.as_ref()
    }

    pub fn reset_node_type_name(&mut self) {
        self.node_type_name = None;
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
    ) -> NodeTypeNodesHealthEvaluation {
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
    ) -> NodeTypeNodesHealthEvaluation {
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
    ) -> NodeTypeNodesHealthEvaluation {
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