/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartitionHealthStateChunkList : The list of partition health state chunks that respect the input filters in the chunk query description. Returned by get cluster health state chunks query as part of the parent application hierarchy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionHealthStateChunkList {
    /// The list of partition health state chunks that respect the input filters in the chunk query.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::PartitionHealthStateChunk>>,
}

impl Default for PartitionHealthStateChunkList {
    fn default() -> Self {
        Self::new()
    }
}

impl PartitionHealthStateChunkList {
    /// The list of partition health state chunks that respect the input filters in the chunk query description. Returned by get cluster health state chunks query as part of the parent application hierarchy.
    pub fn new() -> PartitionHealthStateChunkList {
        PartitionHealthStateChunkList { items: None }
    }

    pub fn set_items(
        &mut self,
        items: Vec<::models::PartitionHealthStateChunk>,
    ) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::PartitionHealthStateChunk>,
    ) -> PartitionHealthStateChunkList {
        self.items = Some(items);
        self
    }

    pub fn items(&self) -> Option<&Vec<::models::PartitionHealthStateChunk>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}
