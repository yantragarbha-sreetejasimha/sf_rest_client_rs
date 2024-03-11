/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceHealthStateChunkList : The list of service health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceHealthStateChunkList {
    /// The list of service health state chunks that respect the input filters in the chunk query.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::ServiceHealthStateChunk>>,
}

impl Default for ServiceHealthStateChunkList {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceHealthStateChunkList {
    /// The list of service health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query.
    pub fn new() -> ServiceHealthStateChunkList {
        ServiceHealthStateChunkList { items: None }
    }

    pub fn set_items(&mut self, items: Vec<::models::ServiceHealthStateChunk>) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::ServiceHealthStateChunk>,
    ) -> ServiceHealthStateChunkList {
        self.items = Some(items);
        self
    }

    pub fn items(&self) -> Option<&Vec<::models::ServiceHealthStateChunk>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}
