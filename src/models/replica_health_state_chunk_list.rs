/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ReplicaHealthStateChunkList : The list of replica health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaHealthStateChunkList {
    /// The list of replica health state chunks that respect the input filters in the chunk query.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::ReplicaHealthStateChunk>>,
}

impl Default for ReplicaHealthStateChunkList {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplicaHealthStateChunkList {
    /// The list of replica health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query.
    pub fn new() -> ReplicaHealthStateChunkList {
        ReplicaHealthStateChunkList { items: None }
    }

    pub fn set_items(&mut self, items: Vec<::models::ReplicaHealthStateChunk>) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::ReplicaHealthStateChunk>,
    ) -> ReplicaHealthStateChunkList {
        self.items = Some(items);
        self
    }

    pub fn items(&self) -> Option<&Vec<::models::ReplicaHealthStateChunk>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}
