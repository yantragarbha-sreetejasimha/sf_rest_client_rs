/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LocalNetworkResourceProperties : Information about a Service Fabric container network local to a single Service Fabric cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalNetworkResourceProperties {
    /// User readable description of the network.
    #[serde(rename = "description")]
    description: Option<String>,
    /// Status of the network.
    #[serde(rename = "status")]
    status: Option<::models::ResourceStatus>,
    /// Gives additional information about the current status of the network.
    #[serde(rename = "statusDetails")]
    status_details: Option<String>,
    /// The type of a Service Fabric container network.
    #[serde(rename = "kind")]
    kind: ::models::NetworkKind,
    /// Address space for the local container network.
    #[serde(rename = "networkAddressPrefix")]
    network_address_prefix: Option<::models::NetworkAddressPrefix>,
}

impl LocalNetworkResourceProperties {
    /// Information about a Service Fabric container network local to a single Service Fabric cluster.
    pub fn new(kind: ::models::NetworkKind) -> LocalNetworkResourceProperties {
        LocalNetworkResourceProperties {
            description: None,
            status: None,
            status_details: None,
            kind,
            network_address_prefix: None,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(
        mut self,
        description: String,
    ) -> LocalNetworkResourceProperties {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_status(&mut self, status: ::models::ResourceStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ResourceStatus,
    ) -> LocalNetworkResourceProperties {
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
    ) -> LocalNetworkResourceProperties {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }

    pub fn set_kind(&mut self, kind: ::models::NetworkKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::NetworkKind,
    ) -> LocalNetworkResourceProperties {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::NetworkKind {
        &self.kind
    }

    pub fn set_network_address_prefix(
        &mut self,
        network_address_prefix: ::models::NetworkAddressPrefix,
    ) {
        self.network_address_prefix = Some(network_address_prefix);
    }

    pub fn with_network_address_prefix(
        mut self,
        network_address_prefix: ::models::NetworkAddressPrefix,
    ) -> LocalNetworkResourceProperties {
        self.network_address_prefix = Some(network_address_prefix);
        self
    }

    pub fn network_address_prefix(
        &self,
    ) -> Option<&::models::NetworkAddressPrefix> {
        self.network_address_prefix.as_ref()
    }

    pub fn reset_network_address_prefix(&mut self) {
        self.network_address_prefix = None;
    }
}
