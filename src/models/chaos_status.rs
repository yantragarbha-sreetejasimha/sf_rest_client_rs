/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosStatus : Current status of the Chaos run.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosStatus {}

impl Default for ChaosStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosStatus {
    /// Current status of the Chaos run.
    pub fn new() -> ChaosStatus {
        ChaosStatus {}
    }
}

// TODO enum
// List of ChaosStatus
//const (
//
//
//
//)
