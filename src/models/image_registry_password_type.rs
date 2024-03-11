/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ImageRegistryPasswordType : The type of the image registry password being given in password

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageRegistryPasswordType {}

impl Default for ImageRegistryPasswordType {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageRegistryPasswordType {
    /// The type of the image registry password being given in password
    pub fn new() -> ImageRegistryPasswordType {
        ImageRegistryPasswordType {}
    }
}

// TODO enum
// List of ImageRegistryPasswordType
//const (
//
//
//
//)
