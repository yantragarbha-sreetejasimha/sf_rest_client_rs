/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PropertyBatchInfo : Information about the results of a property batch.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyBatchInfo {
    /// The kind of property batch info, determined by the results of a property batch. The following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyBatchInfoKind,
}

impl PropertyBatchInfo {
    /// Information about the results of a property batch.
    pub fn new(kind: ::models::PropertyBatchInfoKind) -> PropertyBatchInfo {
        PropertyBatchInfo { kind }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyBatchInfoKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyBatchInfoKind,
    ) -> PropertyBatchInfo {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::PropertyBatchInfoKind {
        &self.kind
    }
}
