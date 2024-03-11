/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerApiResult : Container API result.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerApiResult {
    /// HTTP status code returned by the target container API
    #[serde(rename = "Status")]
    status: i32,
    /// HTTP content type
    #[serde(rename = "Content-Type")]
    content_type: Option<String>,
    /// HTTP content encoding
    #[serde(rename = "Content-Encoding")]
    content_encoding: Option<String>,
    /// container API result body
    #[serde(rename = "Body")]
    body: Option<String>,
}

impl ContainerApiResult {
    /// Container API result.
    pub fn new(status: i32) -> ContainerApiResult {
        ContainerApiResult {
            status,
            content_type: None,
            content_encoding: None,
            body: None,
        }
    }

    pub fn set_status(&mut self, status: i32) {
        self.status = status;
    }

    pub fn with_status(mut self, status: i32) -> ContainerApiResult {
        self.status = status;
        self
    }

    pub fn status(&self) -> &i32 {
        &self.status
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type);
    }

    pub fn with_content_type(
        mut self,
        content_type: String,
    ) -> ContainerApiResult {
        self.content_type = Some(content_type);
        self
    }

    pub fn content_type(&self) -> Option<&String> {
        self.content_type.as_ref()
    }

    pub fn reset_content_type(&mut self) {
        self.content_type = None;
    }

    pub fn set_content_encoding(&mut self, content_encoding: String) {
        self.content_encoding = Some(content_encoding);
    }

    pub fn with_content_encoding(
        mut self,
        content_encoding: String,
    ) -> ContainerApiResult {
        self.content_encoding = Some(content_encoding);
        self
    }

    pub fn content_encoding(&self) -> Option<&String> {
        self.content_encoding.as_ref()
    }

    pub fn reset_content_encoding(&mut self) {
        self.content_encoding = None;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
    }

    pub fn with_body(mut self, body: String) -> ContainerApiResult {
        self.body = Some(body);
        self
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    pub fn reset_body(&mut self) {
        self.body = None;
    }
}
