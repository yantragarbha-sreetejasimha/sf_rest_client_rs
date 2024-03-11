/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HttpRouteMatchRule : Describes a rule for http route matching.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRouteMatchRule {
    /// Path to match for routing.
    #[serde(rename = "path")]
    path: ::models::HttpRouteMatchPath,
    /// headers and their values to match in request.
    #[serde(rename = "headers")]
    headers: Option<Vec<::models::HttpRouteMatchHeader>>,
}

impl HttpRouteMatchRule {
    /// Describes a rule for http route matching.
    pub fn new(path: ::models::HttpRouteMatchPath) -> HttpRouteMatchRule {
        HttpRouteMatchRule {
            path,
            headers: None,
        }
    }

    pub fn set_path(&mut self, path: ::models::HttpRouteMatchPath) {
        self.path = path;
    }

    pub fn with_path(
        mut self,
        path: ::models::HttpRouteMatchPath,
    ) -> HttpRouteMatchRule {
        self.path = path;
        self
    }

    pub fn path(&self) -> &::models::HttpRouteMatchPath {
        &self.path
    }

    pub fn set_headers(
        &mut self,
        headers: Vec<::models::HttpRouteMatchHeader>,
    ) {
        self.headers = Some(headers);
    }

    pub fn with_headers(
        mut self,
        headers: Vec<::models::HttpRouteMatchHeader>,
    ) -> HttpRouteMatchRule {
        self.headers = Some(headers);
        self
    }

    pub fn headers(&self) -> Option<&Vec<::models::HttpRouteMatchHeader>> {
        self.headers.as_ref()
    }

    pub fn reset_headers(&mut self) {
        self.headers = None;
    }
}
