/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BasicRetentionPolicyDescription : Describes basic retention policy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicRetentionPolicyDescription {
    /// The type of retention policy. Currently only \"Basic\" retention policy is supported.
    #[serde(rename = "RetentionPolicyType")]
    retention_policy_type: ::models::RetentionPolicyType,
    /// It is the minimum duration for which a backup created, will remain stored in the storage and might get deleted after that span of time. It should be specified in ISO8601 format.
    #[serde(rename = "RetentionDuration")]
    retention_duration: String,
    /// It is the minimum number of backups to be retained at any point of time. If specified with a non zero value, backups will not be deleted even if the backups have gone past retention duration and have number of backups less than or equal to it.
    #[serde(rename = "MinimumNumberOfBackups")]
    minimum_number_of_backups: Option<i32>,
}

impl BasicRetentionPolicyDescription {
    /// Describes basic retention policy.
    pub fn new(
        retention_policy_type: ::models::RetentionPolicyType,
        retention_duration: String,
    ) -> BasicRetentionPolicyDescription {
        BasicRetentionPolicyDescription {
            retention_policy_type,
            retention_duration,
            minimum_number_of_backups: None,
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
    ) -> BasicRetentionPolicyDescription {
        self.retention_policy_type = retention_policy_type;
        self
    }

    pub fn retention_policy_type(&self) -> &::models::RetentionPolicyType {
        &self.retention_policy_type
    }

    pub fn set_retention_duration(&mut self, retention_duration: String) {
        self.retention_duration = retention_duration;
    }

    pub fn with_retention_duration(
        mut self,
        retention_duration: String,
    ) -> BasicRetentionPolicyDescription {
        self.retention_duration = retention_duration;
        self
    }

    pub fn retention_duration(&self) -> &String {
        &self.retention_duration
    }

    pub fn set_minimum_number_of_backups(
        &mut self,
        minimum_number_of_backups: i32,
    ) {
        self.minimum_number_of_backups = Some(minimum_number_of_backups);
    }

    pub fn with_minimum_number_of_backups(
        mut self,
        minimum_number_of_backups: i32,
    ) -> BasicRetentionPolicyDescription {
        self.minimum_number_of_backups = Some(minimum_number_of_backups);
        self
    }

    pub fn minimum_number_of_backups(&self) -> Option<&i32> {
        self.minimum_number_of_backups.as_ref()
    }

    pub fn reset_minimum_number_of_backups(&mut self) {
        self.minimum_number_of_backups = None;
    }
}
