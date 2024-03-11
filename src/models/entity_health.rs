/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EntityHealth : Health information common to all entities in the cluster. It contains the aggregated health state, health events and unhealthy evaluation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityHealth {
    /// The HealthState representing the aggregated health state of the entity computed by Health Manager. The health evaluation of the entity reflects all events reported on the entity and its children (if any). The aggregation is done by applying the desired health policy.
    #[serde(rename = "AggregatedHealthState")]
    aggregated_health_state: Option<::models::HealthState>,
    /// The list of health events reported on the entity.
    #[serde(rename = "HealthEvents")]
    health_events: Option<Vec<::models::HealthEvent>>,
    /// The unhealthy evaluations that show why the current aggregated health state was returned by Health Manager.
    #[serde(rename = "UnhealthyEvaluations")]
    unhealthy_evaluations: Option<::models::UnhealthyEvaluations>,
    /// Shows the health statistics for all children types of the queried entity.
    #[serde(rename = "HealthStatistics")]
    health_statistics: Option<::models::HealthStatistics>,
}

impl Default for EntityHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityHealth {
    /// Health information common to all entities in the cluster. It contains the aggregated health state, health events and unhealthy evaluation.
    pub fn new() -> EntityHealth {
        EntityHealth {
            aggregated_health_state: None,
            health_events: None,
            unhealthy_evaluations: None,
            health_statistics: None,
        }
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
    ) -> EntityHealth {
        self.aggregated_health_state = Some(aggregated_health_state);
        self
    }

    pub fn aggregated_health_state(&self) -> Option<&::models::HealthState> {
        self.aggregated_health_state.as_ref()
    }

    pub fn reset_aggregated_health_state(&mut self) {
        self.aggregated_health_state = None;
    }

    pub fn set_health_events(
        &mut self,
        health_events: Vec<::models::HealthEvent>,
    ) {
        self.health_events = Some(health_events);
    }

    pub fn with_health_events(
        mut self,
        health_events: Vec<::models::HealthEvent>,
    ) -> EntityHealth {
        self.health_events = Some(health_events);
        self
    }

    pub fn health_events(&self) -> Option<&Vec<::models::HealthEvent>> {
        self.health_events.as_ref()
    }

    pub fn reset_health_events(&mut self) {
        self.health_events = None;
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
    ) -> EntityHealth {
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

    pub fn set_health_statistics(
        &mut self,
        health_statistics: ::models::HealthStatistics,
    ) {
        self.health_statistics = Some(health_statistics);
    }

    pub fn with_health_statistics(
        mut self,
        health_statistics: ::models::HealthStatistics,
    ) -> EntityHealth {
        self.health_statistics = Some(health_statistics);
        self
    }

    pub fn health_statistics(&self) -> Option<&::models::HealthStatistics> {
        self.health_statistics.as_ref()
    }

    pub fn reset_health_statistics(&mut self) {
        self.health_statistics = None;
    }
}
