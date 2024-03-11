/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeHealthPolicyMapItem : Defines an item in ApplicationTypeHealthPolicyMap.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeHealthPolicyMapItem {
    /// The key of the application type health policy map item. This is the name of the application type.
    #[serde(rename = "Key")]
    key: String,
    /// The value of the application type health policy map item. The max percent unhealthy applications allowed for the application type. Must be between zero and 100.
    #[serde(rename = "Value")]
    value: i32,
}

impl ApplicationTypeHealthPolicyMapItem {
    /// Defines an item in ApplicationTypeHealthPolicyMap.
    pub fn new(key: String, value: i32) -> ApplicationTypeHealthPolicyMapItem {
        ApplicationTypeHealthPolicyMapItem { key, value }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn with_key(
        mut self,
        key: String,
    ) -> ApplicationTypeHealthPolicyMapItem {
        self.key = key;
        self
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn with_value(
        mut self,
        value: i32,
    ) -> ApplicationTypeHealthPolicyMapItem {
        self.value = value;
        self
    }

    pub fn value(&self) -> &i32 {
        &self.value
    }
}
