/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PropertyCustomTypeId : The property's custom type ID. Using this property, the user is able to tag the type of the value of the property.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyCustomTypeId {}

impl Default for PropertyCustomTypeId {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyCustomTypeId {
    /// The property's custom type ID. Using this property, the user is able to tag the type of the value of the property.
    pub fn new() -> PropertyCustomTypeId {
        PropertyCustomTypeId {}
    }
}
