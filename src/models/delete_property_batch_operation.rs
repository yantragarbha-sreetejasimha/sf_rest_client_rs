/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeletePropertyBatchOperation : Represents a PropertyBatchOperation that deletes a specified property if it exists. Note that if one PropertyBatchOperation in a PropertyBatch fails, the entire batch fails and cannot be committed in a transactional manner.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePropertyBatchOperation {
    /// The kind of property batch operation, determined by the operation to be performed. The following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyBatchOperationKind,
    /// The name of the Service Fabric property.
    #[serde(rename = "PropertyName")]
    property_name: ::models::PropertyName,
}

impl DeletePropertyBatchOperation {
    /// Represents a PropertyBatchOperation that deletes a specified property if it exists. Note that if one PropertyBatchOperation in a PropertyBatch fails, the entire batch fails and cannot be committed in a transactional manner.
    pub fn new(
        kind: ::models::PropertyBatchOperationKind,
        property_name: ::models::PropertyName,
    ) -> DeletePropertyBatchOperation {
        DeletePropertyBatchOperation {
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
    ) -> DeletePropertyBatchOperation {
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
    ) -> DeletePropertyBatchOperation {
        self.property_name = property_name;
        self
    }

    pub fn property_name(&self) -> &::models::PropertyName {
        &self.property_name
    }
}
