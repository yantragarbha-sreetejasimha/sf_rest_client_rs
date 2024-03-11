/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceTypeHealthPolicyMapItem : Defines an item in ServiceTypeHealthPolicyMap.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeHealthPolicyMapItem {
    /// The key of the service type health policy map item. This is the name of the service type.
    #[serde(rename = "Key")]
    key: String,
    /// The value of the service type health policy map item. This is the ServiceTypeHealthPolicy for this service type.
    #[serde(rename = "Value")]
    value: ::models::ServiceTypeHealthPolicy,
}

impl ServiceTypeHealthPolicyMapItem {
    /// Defines an item in ServiceTypeHealthPolicyMap.
    pub fn new(
        key: String,
        value: ::models::ServiceTypeHealthPolicy,
    ) -> ServiceTypeHealthPolicyMapItem {
        ServiceTypeHealthPolicyMapItem { key, value }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn with_key(mut self, key: String) -> ServiceTypeHealthPolicyMapItem {
        self.key = key;
        self
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn set_value(&mut self, value: ::models::ServiceTypeHealthPolicy) {
        self.value = value;
    }

    pub fn with_value(
        mut self,
        value: ::models::ServiceTypeHealthPolicy,
    ) -> ServiceTypeHealthPolicyMapItem {
        self.value = value;
        self
    }

    pub fn value(&self) -> &::models::ServiceTypeHealthPolicy {
        &self.value
    }
}
