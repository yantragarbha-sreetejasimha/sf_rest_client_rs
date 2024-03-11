/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EnvironmentVariable : Describes an environment variable for the container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    /// The type of the environment variable being given in value
    #[serde(rename = "type")]
    _type: Option<::models::EnvironmentVariableType>,
    /// The name of the environment variable.
    #[serde(rename = "name")]
    name: Option<String>,
    /// The value of the environment variable, will be processed based on the type provided.
    #[serde(rename = "value")]
    value: Option<String>,
}

impl Default for EnvironmentVariable {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentVariable {
    /// Describes an environment variable for the container.
    pub fn new() -> EnvironmentVariable {
        EnvironmentVariable {
            _type: None,
            name: None,
            value: None,
        }
    }

    pub fn set_type(&mut self, _type: ::models::EnvironmentVariableType) {
        self._type = Some(_type);
    }

    pub fn with_type(
        mut self,
        _type: ::models::EnvironmentVariableType,
    ) -> EnvironmentVariable {
        self._type = Some(_type);
        self
    }

    pub fn _type(&self) -> Option<&::models::EnvironmentVariableType> {
        self._type.as_ref()
    }

    pub fn reset_type(&mut self) {
        self._type = None;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> EnvironmentVariable {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn with_value(mut self, value: String) -> EnvironmentVariable {
        self.value = Some(value);
        self
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn reset_value(&mut self) {
        self.value = None;
    }
}
