/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeHealthStateChunkList : The list of node health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHealthStateChunkList {
    /// Total number of entity health state objects that match the specified filters from the cluster health chunk query description.
    #[serde(rename = "TotalCount")]
    total_count: Option<i64>,
    /// The list of node health state chunks that respect the input filters in the chunk query.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::NodeHealthStateChunk>>,
}

impl Default for NodeHealthStateChunkList {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeHealthStateChunkList {
    /// The list of node health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query.
    pub fn new() -> NodeHealthStateChunkList {
        NodeHealthStateChunkList {
            total_count: None,
            items: None,
        }
    }

    pub fn set_total_count(&mut self, total_count: i64) {
        self.total_count = Some(total_count);
    }

    pub fn with_total_count(
        mut self,
        total_count: i64,
    ) -> NodeHealthStateChunkList {
        self.total_count = Some(total_count);
        self
    }

    pub fn total_count(&self) -> Option<&i64> {
        self.total_count.as_ref()
    }

    pub fn reset_total_count(&mut self) {
        self.total_count = None;
    }

    pub fn set_items(&mut self, items: Vec<::models::NodeHealthStateChunk>) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::NodeHealthStateChunk>,
    ) -> NodeHealthStateChunkList {
        self.items = Some(items);
        self
    }

    pub fn items(&self) -> Option<&Vec<::models::NodeHealthStateChunk>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}