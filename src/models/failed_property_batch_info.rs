/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FailedPropertyBatchInfo : Derived from PropertyBatchInfo. Represents the property batch failing. Contains information about the specific batch failure.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedPropertyBatchInfo {
    /// The kind of property batch info, determined by the results of a property batch. The following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyBatchInfoKind,
    /// The error message of the failed operation. Describes the exception thrown due to the first unsuccessful operation in the property batch.
    #[serde(rename = "ErrorMessage")]
    error_message: Option<String>,
    /// The index of the unsuccessful operation in the property batch.
    #[serde(rename = "OperationIndex")]
    operation_index: Option<i32>,
}

impl FailedPropertyBatchInfo {
    /// Derived from PropertyBatchInfo. Represents the property batch failing. Contains information about the specific batch failure.
    pub fn new(
        kind: ::models::PropertyBatchInfoKind,
    ) -> FailedPropertyBatchInfo {
        FailedPropertyBatchInfo {
            kind,
            error_message: None,
            operation_index: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyBatchInfoKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyBatchInfoKind,
    ) -> FailedPropertyBatchInfo {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::PropertyBatchInfoKind {
        &self.kind
    }

    pub fn set_error_message(&mut self, error_message: String) {
        self.error_message = Some(error_message);
    }

    pub fn with_error_message(
        mut self,
        error_message: String,
    ) -> FailedPropertyBatchInfo {
        self.error_message = Some(error_message);
        self
    }

    pub fn error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    pub fn reset_error_message(&mut self) {
        self.error_message = None;
    }

    pub fn set_operation_index(&mut self, operation_index: i32) {
        self.operation_index = Some(operation_index);
    }

    pub fn with_operation_index(
        mut self,
        operation_index: i32,
    ) -> FailedPropertyBatchInfo {
        self.operation_index = Some(operation_index);
        self
    }

    pub fn operation_index(&self) -> Option<&i32> {
        self.operation_index.as_ref()
    }

    pub fn reset_operation_index(&mut self) {
        self.operation_index = None;
    }
}
