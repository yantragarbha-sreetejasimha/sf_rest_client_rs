/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HttpRouteConfig : Describes the hostname properties for http routing.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRouteConfig {
    /// http route name.
    #[serde(rename = "name")]
    name: String,
    /// Describes a rule for http route matching.
    #[serde(rename = "match")]
    _match: ::models::HttpRouteMatchRule,
    /// Describes destination endpoint for routing traffic.
    #[serde(rename = "destination")]
    destination: ::models::GatewayDestination,
}

impl HttpRouteConfig {
    /// Describes the hostname properties for http routing.
    pub fn new(
        name: String,
        _match: ::models::HttpRouteMatchRule,
        destination: ::models::GatewayDestination,
    ) -> HttpRouteConfig {
        HttpRouteConfig {
            name,
            _match,
            destination,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> HttpRouteConfig {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_match(&mut self, _match: ::models::HttpRouteMatchRule) {
        self._match = _match;
    }

    pub fn with_match(
        mut self,
        _match: ::models::HttpRouteMatchRule,
    ) -> HttpRouteConfig {
        self._match = _match;
        self
    }

    pub fn _match(&self) -> &::models::HttpRouteMatchRule {
        &self._match
    }

    pub fn set_destination(
        &mut self,
        destination: ::models::GatewayDestination,
    ) {
        self.destination = destination;
    }

    pub fn with_destination(
        mut self,
        destination: ::models::GatewayDestination,
    ) -> HttpRouteConfig {
        self.destination = destination;
        self
    }

    pub fn destination(&self) -> &::models::GatewayDestination {
        &self.destination
    }
}
