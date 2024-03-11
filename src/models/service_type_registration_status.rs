/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeRegistrationStatus : The status of the service type registration on the node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeRegistrationStatus {}

impl Default for ServiceTypeRegistrationStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTypeRegistrationStatus {
    /// The status of the service type registration on the node.
    pub fn new() -> ServiceTypeRegistrationStatus {
        ServiceTypeRegistrationStatus {}
    }
}

// TODO enum
// List of ServiceTypeRegistrationStatus
//const (
//
//
//
//)
