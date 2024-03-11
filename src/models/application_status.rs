/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationStatus : The status of the application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationStatus {}

impl Default for ApplicationStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationStatus {
    /// The status of the application.
    pub fn new() -> ApplicationStatus {
        ApplicationStatus {}
    }
}

// TODO enum
// List of ApplicationStatus
//const (
//
//
//
//)
