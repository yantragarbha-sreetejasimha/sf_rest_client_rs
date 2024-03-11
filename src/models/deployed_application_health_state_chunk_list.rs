/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationHealthStateChunkList : The list of deployed application health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationHealthStateChunkList {
    /// The list of deployed application health state chunks that respect the input filters in the chunk query.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::DeployedApplicationHealthStateChunk>>,
}

impl Default for DeployedApplicationHealthStateChunkList {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedApplicationHealthStateChunkList {
    /// The list of deployed application health state chunks that respect the input filters in the chunk query. Returned by get cluster health state chunks query.
    pub fn new() -> DeployedApplicationHealthStateChunkList {
        DeployedApplicationHealthStateChunkList { items: None }
    }

    pub fn set_items(
        &mut self,
        items: Vec<::models::DeployedApplicationHealthStateChunk>,
    ) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::DeployedApplicationHealthStateChunk>,
    ) -> DeployedApplicationHealthStateChunkList {
        self.items = Some(items);
        self
    }

    pub fn items(
        &self,
    ) -> Option<&Vec<::models::DeployedApplicationHealthStateChunk>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}
