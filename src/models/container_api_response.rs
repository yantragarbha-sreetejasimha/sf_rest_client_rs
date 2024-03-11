/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerApiResponse : Response body that wraps container API result.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerApiResponse {
    /// Container API result.
    #[serde(rename = "ContainerApiResult")]
    container_api_result: ::models::ContainerApiResult,
}

impl ContainerApiResponse {
    /// Response body that wraps container API result.
    pub fn new(
        container_api_result: ::models::ContainerApiResult,
    ) -> ContainerApiResponse {
        ContainerApiResponse {
            container_api_result,
        }
    }

    pub fn set_container_api_result(
        &mut self,
        container_api_result: ::models::ContainerApiResult,
    ) {
        self.container_api_result = container_api_result;
    }

    pub fn with_container_api_result(
        mut self,
        container_api_result: ::models::ContainerApiResult,
    ) -> ContainerApiResponse {
        self.container_api_result = container_api_result;
        self
    }

    pub fn container_api_result(&self) -> &::models::ContainerApiResult {
        &self.container_api_result
    }
}
