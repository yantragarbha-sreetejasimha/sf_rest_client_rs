/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PropertyBatchOperationKind : The kind of property batch operation, determined by the operation to be performed. The following are the possible values.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyBatchOperationKind {}

impl Default for PropertyBatchOperationKind {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyBatchOperationKind {
    /// The kind of property batch operation, determined by the operation to be performed. The following are the possible values.
    pub fn new() -> PropertyBatchOperationKind {
        PropertyBatchOperationKind {}
    }
}

// TODO enum
// List of PropertyBatchOperationKind
//const (
//
//
//
//)
