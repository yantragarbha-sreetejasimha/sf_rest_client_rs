/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HealthStatistics : The health statistics of an entity, returned as part of the health query result when the query description is configured to include statistics. The statistics include health state counts for all children types of the current entity. For example, for cluster, the health statistics include health state counts for nodes, applications, services, partitions, replicas, deployed applications and deployed service packages. For partition, the health statistics include health counts for replicas.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatistics {
    /// List of health state counts per entity kind, which keeps track of how many children of the queried entity are in Ok, Warning and Error state.
    #[serde(rename = "HealthStateCountList")]
    health_state_count_list: Option<Vec<::models::EntityKindHealthStateCount>>,
}

impl Default for HealthStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthStatistics {
    /// The health statistics of an entity, returned as part of the health query result when the query description is configured to include statistics. The statistics include health state counts for all children types of the current entity. For example, for cluster, the health statistics include health state counts for nodes, applications, services, partitions, replicas, deployed applications and deployed service packages. For partition, the health statistics include health counts for replicas.
    pub fn new() -> HealthStatistics {
        HealthStatistics {
            health_state_count_list: None,
        }
    }

    pub fn set_health_state_count_list(
        &mut self,
        health_state_count_list: Vec<::models::EntityKindHealthStateCount>,
    ) {
        self.health_state_count_list = Some(health_state_count_list);
    }

    pub fn with_health_state_count_list(
        mut self,
        health_state_count_list: Vec<::models::EntityKindHealthStateCount>,
    ) -> HealthStatistics {
        self.health_state_count_list = Some(health_state_count_list);
        self
    }

    pub fn health_state_count_list(
        &self,
    ) -> Option<&Vec<::models::EntityKindHealthStateCount>> {
        self.health_state_count_list.as_ref()
    }

    pub fn reset_health_state_count_list(&mut self) {
        self.health_state_count_list = None;
    }
}
