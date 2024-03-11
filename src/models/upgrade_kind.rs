/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeKind : The kind of upgrade out of the following possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeKind {}

impl Default for UpgradeKind {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeKind {
    /// The kind of upgrade out of the following possible values.
    pub fn new() -> UpgradeKind {
        UpgradeKind {}
    }
}

// TODO enum
// List of UpgradeKind
//const (
//
//
//
//)