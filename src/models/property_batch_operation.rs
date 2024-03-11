/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PropertyBatchOperation : Represents the base type for property operations that can be put into a batch and submitted.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyBatchOperation {
    /// The kind of property batch operation, determined by the operation to be performed. The following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyBatchOperationKind,
    /// The name of the Service Fabric property.
    #[serde(rename = "PropertyName")]
    property_name: ::models::PropertyName,
}

impl PropertyBatchOperation {
    /// Represents the base type for property operations that can be put into a batch and submitted.
    pub fn new(
        kind: ::models::PropertyBatchOperationKind,
        property_name: ::models::PropertyName,
    ) -> PropertyBatchOperation {
        PropertyBatchOperation {
            kind,
            property_name,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyBatchOperationKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyBatchOperationKind,
    ) -> PropertyBatchOperation {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::PropertyBatchOperationKind {
        &self.kind
    }

    pub fn set_property_name(&mut self, property_name: ::models::PropertyName) {
        self.property_name = property_name;
    }

    pub fn with_property_name(
        mut self,
        property_name: ::models::PropertyName,
    ) -> PropertyBatchOperation {
        self.property_name = property_name;
        self
    }

    pub fn property_name(&self) -> &::models::PropertyName {
        &self.property_name
    }
}
