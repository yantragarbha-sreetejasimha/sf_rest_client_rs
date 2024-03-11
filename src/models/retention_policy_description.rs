/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RetentionPolicyDescription : Describes the retention policy configured.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionPolicyDescription {
    /// The type of retention policy. Currently only \"Basic\" retention policy is supported.
    #[serde(rename = "RetentionPolicyType")]
    retention_policy_type: ::models::RetentionPolicyType,
}

impl RetentionPolicyDescription {
    /// Describes the retention policy configured.
    pub fn new(
        retention_policy_type: ::models::RetentionPolicyType,
    ) -> RetentionPolicyDescription {
        RetentionPolicyDescription {
            retention_policy_type,
        }
    }

    pub fn set_retention_policy_type(
        &mut self,
        retention_policy_type: ::models::RetentionPolicyType,
    ) {
        self.retention_policy_type = retention_policy_type;
    }

    pub fn with_retention_policy_type(
        mut self,
        retention_policy_type: ::models::RetentionPolicyType,
    ) -> RetentionPolicyDescription {
        self.retention_policy_type = retention_policy_type;
        self
    }

    pub fn retention_policy_type(&self) -> &::models::RetentionPolicyType {
        &self.retention_policy_type
    }
}
