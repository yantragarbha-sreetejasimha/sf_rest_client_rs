/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Int64PropertyValue : Describes a Service Fabric property value of type Int64.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Int64PropertyValue {
    /// The kind of property, determined by the type of data. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyValueKind,
    /// The data of the property value.
    #[serde(rename = "Data")]
    data: String,
}

impl Int64PropertyValue {
    /// Describes a Service Fabric property value of type Int64.
    pub fn new(
        kind: ::models::PropertyValueKind,
        data: String,
    ) -> Int64PropertyValue {
        Int64PropertyValue { kind, data }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyValueKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyValueKind,
    ) -> Int64PropertyValue {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::PropertyValueKind {
        &self.kind
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn with_data(mut self, data: String) -> Int64PropertyValue {
        self.data = data;
        self
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}
