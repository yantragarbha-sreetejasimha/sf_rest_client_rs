/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HttpConfig : Describes the http configuration for external connectivity for this network.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    /// http gateway config name.
    #[serde(rename = "name")]
    name: String,
    /// Specifies the port at which the service endpoint below needs to be exposed.
    #[serde(rename = "port")]
    port: i32,
    /// description for routing.
    #[serde(rename = "hosts")]
    hosts: Vec<::models::HttpHostConfig>,
}

impl HttpConfig {
    /// Describes the http configuration for external connectivity for this network.
    pub fn new(
        name: String,
        port: i32,
        hosts: Vec<::models::HttpHostConfig>,
    ) -> HttpConfig {
        HttpConfig { name, port, hosts }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> HttpConfig {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_port(&mut self, port: i32) {
        self.port = port;
    }

    pub fn with_port(mut self, port: i32) -> HttpConfig {
        self.port = port;
        self
    }

    pub fn port(&self) -> &i32 {
        &self.port
    }

    pub fn set_hosts(&mut self, hosts: Vec<::models::HttpHostConfig>) {
        self.hosts = hosts;
    }

    pub fn with_hosts(
        mut self,
        hosts: Vec<::models::HttpHostConfig>,
    ) -> HttpConfig {
        self.hosts = hosts;
        self
    }

    pub fn hosts(&self) -> &Vec<::models::HttpHostConfig> {
        &self.hosts
    }
}
