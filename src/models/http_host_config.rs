/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HttpHostConfig : Describes the hostname properties for http routing.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHostConfig {
    /// http hostname config name.
    #[serde(rename = "name")]
    name: String,
    /// Route information to use for routing. Routes are processed in the order they are specified. Specify routes that are more specific before routes that can handle general cases.
    #[serde(rename = "routes")]
    routes: Vec<::models::HttpRouteConfig>,
}

impl HttpHostConfig {
    /// Describes the hostname properties for http routing.
    pub fn new(
        name: String,
        routes: Vec<::models::HttpRouteConfig>,
    ) -> HttpHostConfig {
        HttpHostConfig { name, routes }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> HttpHostConfig {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_routes(&mut self, routes: Vec<::models::HttpRouteConfig>) {
        self.routes = routes;
    }

    pub fn with_routes(
        mut self,
        routes: Vec<::models::HttpRouteConfig>,
    ) -> HttpHostConfig {
        self.routes = routes;
        self
    }

    pub fn routes(&self) -> &Vec<::models::HttpRouteConfig> {
        &self.routes
    }
}
