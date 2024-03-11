/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CheckSequencePropertyBatchOperation : Compares the Sequence Number of a property with the SequenceNumber argument. A property's sequence number can be thought of as that property's version. Every time the property is modified, its sequence number is increased. The sequence number can be found in a property's metadata. The comparison fails if the sequence numbers are not equal. CheckSequencePropertyBatchOperation is generally used as a precondition for the write operations in the batch. Note that if one PropertyBatchOperation in a PropertyBatch fails, the entire batch fails and cannot be committed in a transactional manner.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckSequencePropertyBatchOperation {
    /// The kind of property batch operation, determined by the operation to be performed. The following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyBatchOperationKind,
    /// The name of the Service Fabric property.
    #[serde(rename = "PropertyName")]
    property_name: ::models::PropertyName,
    /// The expected sequence number.
    #[serde(rename = "SequenceNumber")]
    sequence_number: String,
}

impl CheckSequencePropertyBatchOperation {
    /// Compares the Sequence Number of a property with the SequenceNumber argument. A property's sequence number can be thought of as that property's version. Every time the property is modified, its sequence number is increased. The sequence number can be found in a property's metadata. The comparison fails if the sequence numbers are not equal. CheckSequencePropertyBatchOperation is generally used as a precondition for the write operations in the batch. Note that if one PropertyBatchOperation in a PropertyBatch fails, the entire batch fails and cannot be committed in a transactional manner.
    pub fn new(
        kind: ::models::PropertyBatchOperationKind,
        property_name: ::models::PropertyName,
        sequence_number: String,
    ) -> CheckSequencePropertyBatchOperation {
        CheckSequencePropertyBatchOperation {
            kind,
            property_name,
            sequence_number,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyBatchOperationKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyBatchOperationKind,
    ) -> CheckSequencePropertyBatchOperation {
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
    ) -> CheckSequencePropertyBatchOperation {
        self.property_name = property_name;
        self
    }

    pub fn property_name(&self) -> &::models::PropertyName {
        &self.property_name
    }

    pub fn set_sequence_number(&mut self, sequence_number: String) {
        self.sequence_number = sequence_number;
    }

    pub fn with_sequence_number(
        mut self,
        sequence_number: String,
    ) -> CheckSequencePropertyBatchOperation {
        self.sequence_number = sequence_number;
        self
    }

    pub fn sequence_number(&self) -> &String {
        &self.sequence_number
    }
}