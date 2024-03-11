/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceOperationName : Specifies the current active life-cycle operation on a stateful service replica or stateless service instance.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceOperationName {}

impl Default for ServiceOperationName {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceOperationName {
    /// Specifies the current active life-cycle operation on a stateful service replica or stateless service instance.
    pub fn new() -> ServiceOperationName {
        ServiceOperationName {}
    }
}

// TODO enum
// List of ServiceOperationName
//const (
//
//
//
//)
