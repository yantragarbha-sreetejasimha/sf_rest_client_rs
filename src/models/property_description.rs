/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PropertyDescription : Description of a Service Fabric property.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDescription {
    /// The name of the Service Fabric property.
    #[serde(rename = "PropertyName")]
    property_name: ::models::PropertyName,
    /// The property's custom type ID. Using this property, the user is able to tag the type of the value of the property.
    #[serde(rename = "CustomTypeId")]
    custom_type_id: Option<::models::PropertyCustomTypeId>,
    /// Describes a Service Fabric property value.
    #[serde(rename = "Value")]
    value: ::models::PropertyValue,
}

impl PropertyDescription {
    /// Description of a Service Fabric property.
    pub fn new(
        property_name: ::models::PropertyName,
        value: ::models::PropertyValue,
    ) -> PropertyDescription {
        PropertyDescription {
            property_name,
            custom_type_id: None,
            value,
        }
    }

    pub fn set_property_name(&mut self, property_name: ::models::PropertyName) {
        self.property_name = property_name;
    }

    pub fn with_property_name(
        mut self,
        property_name: ::models::PropertyName,
    ) -> PropertyDescription {
        self.property_name = property_name;
        self
    }

    pub fn property_name(&self) -> &::models::PropertyName {
        &self.property_name
    }

    pub fn set_custom_type_id(
        &mut self,
        custom_type_id: ::models::PropertyCustomTypeId,
    ) {
        self.custom_type_id = Some(custom_type_id);
    }

    pub fn with_custom_type_id(
        mut self,
        custom_type_id: ::models::PropertyCustomTypeId,
    ) -> PropertyDescription {
        self.custom_type_id = Some(custom_type_id);
        self
    }

    pub fn custom_type_id(&self) -> Option<&::models::PropertyCustomTypeId> {
        self.custom_type_id.as_ref()
    }

    pub fn reset_custom_type_id(&mut self) {
        self.custom_type_id = None;
    }

    pub fn set_value(&mut self, value: ::models::PropertyValue) {
        self.value = value;
    }

    pub fn with_value(
        mut self,
        value: ::models::PropertyValue,
    ) -> PropertyDescription {
        self.value = value;
        self
    }

    pub fn value(&self) -> &::models::PropertyValue {
        &self.value
    }
}
