/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationHealthPolicyMapItem : Defines an item in ApplicationHealthPolicyMap.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationHealthPolicyMapItem {
    /// The key of the application health policy map item. This is the name of the application.
    #[serde(rename = "Key")]
    key: ::models::ApplicationName,
    /// The value of the application health policy map item. This is the ApplicationHealthPolicy for this application.
    #[serde(rename = "Value")]
    value: ::models::ApplicationHealthPolicy,
}

impl ApplicationHealthPolicyMapItem {
    /// Defines an item in ApplicationHealthPolicyMap.
    pub fn new(
        key: ::models::ApplicationName,
        value: ::models::ApplicationHealthPolicy,
    ) -> ApplicationHealthPolicyMapItem {
        ApplicationHealthPolicyMapItem { key, value }
    }

    pub fn set_key(&mut self, key: ::models::ApplicationName) {
        self.key = key;
    }

    pub fn with_key(
        mut self,
        key: ::models::ApplicationName,
    ) -> ApplicationHealthPolicyMapItem {
        self.key = key;
        self
    }

    pub fn key(&self) -> &::models::ApplicationName {
        &self.key
    }

    pub fn set_value(&mut self, value: ::models::ApplicationHealthPolicy) {
        self.value = value;
    }

    pub fn with_value(
        mut self,
        value: ::models::ApplicationHealthPolicy,
    ) -> ApplicationHealthPolicyMapItem {
        self.value = value;
        self
    }

    pub fn value(&self) -> &::models::ApplicationHealthPolicy {
        &self.value
    }
}
