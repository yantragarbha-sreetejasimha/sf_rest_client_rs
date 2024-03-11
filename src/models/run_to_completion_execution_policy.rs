/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RunToCompletionExecutionPolicy : The run to completion execution policy, the service will perform its desired operation and complete successfully. If the service encounters failure, it will restarted based on restart policy specified. If the service completes its operation successfully, it will not be restarted again.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunToCompletionExecutionPolicy {
    /// Enumerates the execution policy types for services.
    #[serde(rename = "type")]
    _type: ::models::ExecutionPolicyType,
    /// Enumerates the restart policy for RunToCompletionExecutionPolicy
    #[serde(rename = "restart")]
    restart: ::models::RestartPolicy,
}

impl RunToCompletionExecutionPolicy {
    /// The run to completion execution policy, the service will perform its desired operation and complete successfully. If the service encounters failure, it will restarted based on restart policy specified. If the service completes its operation successfully, it will not be restarted again.
    pub fn new(
        _type: ::models::ExecutionPolicyType,
        restart: ::models::RestartPolicy,
    ) -> RunToCompletionExecutionPolicy {
        RunToCompletionExecutionPolicy { _type, restart }
    }

    pub fn set_type(&mut self, _type: ::models::ExecutionPolicyType) {
        self._type = _type;
    }

    pub fn with_type(
        mut self,
        _type: ::models::ExecutionPolicyType,
    ) -> RunToCompletionExecutionPolicy {
        self._type = _type;
        self
    }

    pub fn _type(&self) -> &::models::ExecutionPolicyType {
        &self._type
    }

    pub fn set_restart(&mut self, restart: ::models::RestartPolicy) {
        self.restart = restart;
    }

    pub fn with_restart(
        mut self,
        restart: ::models::RestartPolicy,
    ) -> RunToCompletionExecutionPolicy {
        self.restart = restart;
        self
    }

    pub fn restart(&self) -> &::models::RestartPolicy {
        &self.restart
    }
}
