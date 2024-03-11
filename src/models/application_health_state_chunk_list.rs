/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationHealthStateChunkList : The list of application health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationHealthStateChunkList {
    /// Total number of entity health state objects that match the specified filters from the cluster health chunk query description.
    #[serde(rename = "TotalCount")]
    total_count: Option<i64>,
    /// The list of application health state chunks that respect the input filters in the chunk query.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::ApplicationHealthStateChunk>>,
}

impl Default for ApplicationHealthStateChunkList {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHealthStateChunkList {
    /// The list of application health state chunks in the cluster that respect the input filters in the chunk query. Returned by get cluster health state chunks query.
    pub fn new() -> ApplicationHealthStateChunkList {
        ApplicationHealthStateChunkList {
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
    ) -> ApplicationHealthStateChunkList {
        self.total_count = Some(total_count);
        self
    }

    pub fn total_count(&self) -> Option<&i64> {
        self.total_count.as_ref()
    }

    pub fn reset_total_count(&mut self) {
        self.total_count = None;
    }

    pub fn set_items(
        &mut self,
        items: Vec<::models::ApplicationHealthStateChunk>,
    ) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::ApplicationHealthStateChunk>,
    ) -> ApplicationHealthStateChunkList {
        self.items = Some(items);
        self
    }

    pub fn items(&self) -> Option<&Vec<::models::ApplicationHealthStateChunk>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}
