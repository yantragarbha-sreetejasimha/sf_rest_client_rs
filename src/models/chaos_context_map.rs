/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosContextMap : Describes a map that contains a collection of ChaosContextMapItem's.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosContextMap {}

impl Default for ChaosContextMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosContextMap {
    /// Describes a map that contains a collection of ChaosContextMapItem's.
    pub fn new() -> ChaosContextMap {
        ChaosContextMap {}
    }
}
