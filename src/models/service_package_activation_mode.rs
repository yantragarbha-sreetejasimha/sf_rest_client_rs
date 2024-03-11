/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServicePackageActivationMode : The activation mode of service package to be used for a Service Fabric service. This is specified at the time of creating the Service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicePackageActivationMode {}

impl Default for ServicePackageActivationMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ServicePackageActivationMode {
    /// The activation mode of service package to be used for a Service Fabric service. This is specified at the time of creating the Service.
    pub fn new() -> ServicePackageActivationMode {
        ServicePackageActivationMode {}
    }
}

// TODO enum
// List of ServicePackageActivationMode
//const (
//
//
//
//)
