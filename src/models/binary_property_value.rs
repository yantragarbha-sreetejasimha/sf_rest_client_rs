/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BinaryPropertyValue : Describes a Service Fabric property value of type Binary.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryPropertyValue {
    /// The kind of property, determined by the type of data. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyValueKind,
    /// Array of bytes to be sent as an integer array. Each element of array is a number between 0 and 255.
    #[serde(rename = "Data")]
    data: String,
}

impl BinaryPropertyValue {
    /// Describes a Service Fabric property value of type Binary.
    pub fn new(
        kind: ::models::PropertyValueKind,
        data: String,
    ) -> BinaryPropertyValue {
        BinaryPropertyValue { kind, data }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyValueKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyValueKind,
    ) -> BinaryPropertyValue {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::PropertyValueKind {
        &self.kind
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn with_data(mut self, data: String) -> BinaryPropertyValue {
        self.data = data;
        self
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}
