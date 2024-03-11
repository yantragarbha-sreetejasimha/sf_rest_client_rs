/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerLogs : Container logs.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerLogs {
    /// Container logs.
    #[serde(rename = "Content")]
    content: Option<String>,
}

impl Default for ContainerLogs {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerLogs {
    /// Container logs.
    pub fn new() -> ContainerLogs {
        ContainerLogs { content: None }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn with_content(mut self, content: String) -> ContainerLogs {
        self.content = Some(content);
        self
    }

    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    pub fn reset_content(&mut self) {
        self.content = None;
    }
}
