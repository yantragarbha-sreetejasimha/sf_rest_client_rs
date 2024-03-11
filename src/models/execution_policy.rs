/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExecutionPolicy : The execution policy of the service

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    /// Enumerates the execution policy types for services.
    #[serde(rename = "type")]
    _type: ::models::ExecutionPolicyType,
}

impl ExecutionPolicy {
    /// The execution policy of the service
    pub fn new(_type: ::models::ExecutionPolicyType) -> ExecutionPolicy {
        ExecutionPolicy { _type }
    }

    pub fn set_type(&mut self, _type: ::models::ExecutionPolicyType) {
        self._type = _type;
    }

    pub fn with_type(
        mut self,
        _type: ::models::ExecutionPolicyType,
    ) -> ExecutionPolicy {
        self._type = _type;
        self
    }

    pub fn _type(&self) -> &::models::ExecutionPolicyType {
        &self._type
    }
}
