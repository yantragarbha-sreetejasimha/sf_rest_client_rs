/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerInstanceEventList : A list of ContainerInstanceEvent objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerInstanceEventList {}

impl Default for ContainerInstanceEventList {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerInstanceEventList {
    /// A list of ContainerInstanceEvent objects.
    pub fn new() -> ContainerInstanceEventList {
        ContainerInstanceEventList {}
    }
}
