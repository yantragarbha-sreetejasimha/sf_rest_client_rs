/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SafetyCheckInfoList : List of pending safety checks

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyCheckInfoList {}

impl Default for SafetyCheckInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyCheckInfoList {
    /// List of pending safety checks
    pub fn new() -> SafetyCheckInfoList {
        SafetyCheckInfoList {}
    }
}
