/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NetworkAddressPrefix : Address space for a container network. This is expressed in CIDR notation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkAddressPrefix {}

impl Default for NetworkAddressPrefix {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkAddressPrefix {
    /// Address space for a container network. This is expressed in CIDR notation.
    pub fn new() -> NetworkAddressPrefix {
        NetworkAddressPrefix {}
    }
}
