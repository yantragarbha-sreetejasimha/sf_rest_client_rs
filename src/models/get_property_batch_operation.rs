/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetPropertyBatchOperation : Represents a PropertyBatchOperation that gets the specified property if it exists. Note that if one PropertyBatchOperation in a PropertyBatch fails, the entire batch fails and cannot be committed in a transactional manner.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPropertyBatchOperation {
    /// The kind of property batch operation, determined by the operation to be performed. The following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyBatchOperationKind,
    /// The name of the Service Fabric property.
    #[serde(rename = "PropertyName")]
    property_name: ::models::PropertyName,
    /// Whether or not to return the property value with the metadata. True if values should be returned with the metadata; False to return only property metadata.
    #[serde(rename = "IncludeValue")]
    include_value: Option<bool>,
}

impl GetPropertyBatchOperation {
    /// Represents a PropertyBatchOperation that gets the specified property if it exists. Note that if one PropertyBatchOperation in a PropertyBatch fails, the entire batch fails and cannot be committed in a transactional manner.
    pub fn new(
        kind: ::models::PropertyBatchOperationKind,
        property_name: ::models::PropertyName,
    ) -> GetPropertyBatchOperation {
        GetPropertyBatchOperation {
            kind,
            property_name,
            include_value: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyBatchOperationKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyBatchOperationKind,
    ) -> GetPropertyBatchOperation {
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
    ) -> GetPropertyBatchOperation {
        self.property_name = property_name;
        self
    }

    pub fn property_name(&self) -> &::models::PropertyName {
        &self.property_name
    }

    pub fn set_include_value(&mut self, include_value: bool) {
        self.include_value = Some(include_value);
    }

    pub fn with_include_value(
        mut self,
        include_value: bool,
    ) -> GetPropertyBatchOperation {
        self.include_value = Some(include_value);
        self
    }

    pub fn include_value(&self) -> Option<&bool> {
        self.include_value.as_ref()
    }

    pub fn reset_include_value(&mut self) {
        self.include_value = None;
    }
}
