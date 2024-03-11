/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GatewayProperties : Describes properties of a gateway resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayProperties {
    /// User readable description of the gateway.
    #[serde(rename = "description")]
    description: Option<String>,
    /// Network the gateway should listen on for requests.
    #[serde(rename = "sourceNetwork")]
    source_network: ::models::NetworkRef,
    /// Network that the Application is using.
    #[serde(rename = "destinationNetwork")]
    destination_network: ::models::NetworkRef,
    /// Configuration for tcp connectivity for this gateway.
    #[serde(rename = "tcp")]
    tcp: Option<Vec<::models::TcpConfig>>,
    /// Configuration for http connectivity for this gateway.
    #[serde(rename = "http")]
    http: Option<Vec<::models::HttpConfig>>,
    /// Status of the resource.
    #[serde(rename = "status")]
    status: Option<::models::ResourceStatus>,
    /// Gives additional information about the current status of the gateway.
    #[serde(rename = "statusDetails")]
    status_details: Option<String>,
    /// IP address of the gateway. This is populated in the response and is ignored for incoming requests.
    #[serde(rename = "ipAddress")]
    ip_address: Option<String>,
}

impl GatewayProperties {
    /// Describes properties of a gateway resource.
    pub fn new(
        source_network: ::models::NetworkRef,
        destination_network: ::models::NetworkRef,
    ) -> GatewayProperties {
        GatewayProperties {
            description: None,
            source_network,
            destination_network,
            tcp: None,
            http: None,
            status: None,
            status_details: None,
            ip_address: None,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> GatewayProperties {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_source_network(&mut self, source_network: ::models::NetworkRef) {
        self.source_network = source_network;
    }

    pub fn with_source_network(
        mut self,
        source_network: ::models::NetworkRef,
    ) -> GatewayProperties {
        self.source_network = source_network;
        self
    }

    pub fn source_network(&self) -> &::models::NetworkRef {
        &self.source_network
    }

    pub fn set_destination_network(
        &mut self,
        destination_network: ::models::NetworkRef,
    ) {
        self.destination_network = destination_network;
    }

    pub fn with_destination_network(
        mut self,
        destination_network: ::models::NetworkRef,
    ) -> GatewayProperties {
        self.destination_network = destination_network;
        self
    }

    pub fn destination_network(&self) -> &::models::NetworkRef {
        &self.destination_network
    }

    pub fn set_tcp(&mut self, tcp: Vec<::models::TcpConfig>) {
        self.tcp = Some(tcp);
    }

    pub fn with_tcp(
        mut self,
        tcp: Vec<::models::TcpConfig>,
    ) -> GatewayProperties {
        self.tcp = Some(tcp);
        self
    }

    pub fn tcp(&self) -> Option<&Vec<::models::TcpConfig>> {
        self.tcp.as_ref()
    }

    pub fn reset_tcp(&mut self) {
        self.tcp = None;
    }

    pub fn set_http(&mut self, http: Vec<::models::HttpConfig>) {
        self.http = Some(http);
    }

    pub fn with_http(
        mut self,
        http: Vec<::models::HttpConfig>,
    ) -> GatewayProperties {
        self.http = Some(http);
        self
    }

    pub fn http(&self) -> Option<&Vec<::models::HttpConfig>> {
        self.http.as_ref()
    }

    pub fn reset_http(&mut self) {
        self.http = None;
    }

    pub fn set_status(&mut self, status: ::models::ResourceStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ResourceStatus,
    ) -> GatewayProperties {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::ResourceStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_status_details(&mut self, status_details: String) {
        self.status_details = Some(status_details);
    }

    pub fn with_status_details(
        mut self,
        status_details: String,
    ) -> GatewayProperties {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }

    pub fn set_ip_address(&mut self, ip_address: String) {
        self.ip_address = Some(ip_address);
    }

    pub fn with_ip_address(mut self, ip_address: String) -> GatewayProperties {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }

    pub fn reset_ip_address(&mut self) {
        self.ip_address = None;
    }
}