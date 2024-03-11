/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PropertyValueKind : The kind of property, determined by the type of data. Following are the possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyValueKind {}

impl Default for PropertyValueKind {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyValueKind {
    /// The kind of property, determined by the type of data. Following are the possible values.
    pub fn new() -> PropertyValueKind {
        PropertyValueKind {}
    }
}

// TODO enum
// List of PropertyValueKind
//const (
//
//
//
//)
