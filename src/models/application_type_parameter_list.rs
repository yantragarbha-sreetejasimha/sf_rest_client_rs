/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeParameterList : List of application type parameters that can be overridden when creating or updating the application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeParameterList {}

impl Default for ApplicationTypeParameterList {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationTypeParameterList {
    /// List of application type parameters that can be overridden when creating or updating the application.
    pub fn new() -> ApplicationTypeParameterList {
        ApplicationTypeParameterList {}
    }
}
