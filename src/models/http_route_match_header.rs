/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HttpRouteMatchHeader : Describes header information for http route matching.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRouteMatchHeader {
    /// Name of header to match in request.
    #[serde(rename = "name")]
    name: String,
    /// Value of header to match in request.
    #[serde(rename = "value")]
    value: Option<String>,
    /// how to match header value
    #[serde(rename = "type")]
    _type: Option<String>,
}

impl HttpRouteMatchHeader {
    /// Describes header information for http route matching.
    pub fn new(name: String) -> HttpRouteMatchHeader {
        HttpRouteMatchHeader {
            name,
            value: None,
            _type: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> HttpRouteMatchHeader {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn with_value(mut self, value: String) -> HttpRouteMatchHeader {
        self.value = Some(value);
        self
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn reset_value(&mut self) {
        self.value = None;
    }

    pub fn set_type(&mut self, _type: String) {
        self._type = Some(_type);
    }

    pub fn with_type(mut self, _type: String) -> HttpRouteMatchHeader {
        self._type = Some(_type);
        self
    }

    pub fn _type(&self) -> Option<&String> {
        self._type.as_ref()
    }

    pub fn reset_type(&mut self) {
        self._type = None;
    }
}
