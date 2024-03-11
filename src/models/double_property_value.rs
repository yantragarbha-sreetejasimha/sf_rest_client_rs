/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DoublePropertyValue : Describes a Service Fabric property value of type Double.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DoublePropertyValue {
    /// The kind of property, determined by the type of data. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyValueKind,
    /// The data of the property value.
    #[serde(rename = "Data")]
    data: f64,
}

impl DoublePropertyValue {
    /// Describes a Service Fabric property value of type Double.
    pub fn new(
        kind: ::models::PropertyValueKind,
        data: f64,
    ) -> DoublePropertyValue {
        DoublePropertyValue { kind, data }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyValueKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyValueKind,
    ) -> DoublePropertyValue {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::PropertyValueKind {
        &self.kind
    }

    pub fn set_data(&mut self, data: f64) {
        self.data = data;
    }

    pub fn with_data(mut self, data: f64) -> DoublePropertyValue {
        self.data = data;
        self
    }

    pub fn data(&self) -> &f64 {
        &self.data
    }
}
