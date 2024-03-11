/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairImpactKind : Specifies the kind of the impact. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairImpactKind {}

impl Default for RepairImpactKind {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairImpactKind {
    /// Specifies the kind of the impact. This type supports the Service Fabric platform; it is not meant to be used directly from your code.'
    pub fn new() -> RepairImpactKind {
        RepairImpactKind {}
    }
}

// TODO enum
// List of RepairImpactKind
//const (
//
//
//
//)
