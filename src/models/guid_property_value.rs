/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GuidPropertyValue : Describes a Service Fabric property value of type Guid.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GuidPropertyValue {
    /// The kind of property, determined by the type of data. Following are the possible values.
    #[serde(rename = "Kind")]
    kind: ::models::PropertyValueKind,
    /// The data of the property value.
    #[serde(rename = "Data")]
    data: String,
}

impl GuidPropertyValue {
    /// Describes a Service Fabric property value of type Guid.
    pub fn new(
        kind: ::models::PropertyValueKind,
        data: String,
    ) -> GuidPropertyValue {
        GuidPropertyValue { kind, data }
    }

    pub fn set_kind(&mut self, kind: ::models::PropertyValueKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::PropertyValueKind,
    ) -> GuidPropertyValue {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::PropertyValueKind {
        &self.kind
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn with_data(mut self, data: String) -> GuidPropertyValue {
        self.data = data;
        self
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}
