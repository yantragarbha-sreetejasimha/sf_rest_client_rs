/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairTargetKind : Specifies the kind of the repair target. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairTargetKind {}

impl Default for RepairTargetKind {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairTargetKind {
    /// Specifies the kind of the repair target. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'
    pub fn new() -> RepairTargetKind {
        RepairTargetKind {}
    }
}

// TODO enum
// List of RepairTargetKind
//const (
//
//
//
//)
