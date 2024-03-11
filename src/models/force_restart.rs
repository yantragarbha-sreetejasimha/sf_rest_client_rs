/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ForceRestart : If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceRestart {}

impl Default for ForceRestart {
    fn default() -> Self {
        Self::new()
    }
}

impl ForceRestart {
    /// If true, then processes are forcefully restarted during upgrade even when the code version has not changed (the upgrade only changes configuration or data).
    pub fn new() -> ForceRestart {
        ForceRestart {}
    }
}
