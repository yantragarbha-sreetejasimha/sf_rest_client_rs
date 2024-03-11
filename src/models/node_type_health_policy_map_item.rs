/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeTypeHealthPolicyMapItem : Defines an item in NodeTypeHealthPolicyMap.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTypeHealthPolicyMapItem {
    /// The key of the node type health policy map item. This is the name of the node type.
    #[serde(rename = "Key")]
    key: String,
    /// The value of the node type health policy map item. If the percentage is respected but there is at least one unhealthy node in the node type, the health is evaluated as Warning.  The percentage is calculated by dividing the number of unhealthy nodes over the total number of nodes in the node type.  The computation rounds up to tolerate one failure on small numbers of nodes. The max percent unhealthy nodes allowed for the node type. Must be between zero and 100.
    #[serde(rename = "Value")]
    value: i32,
}

impl NodeTypeHealthPolicyMapItem {
    /// Defines an item in NodeTypeHealthPolicyMap.
    pub fn new(key: String, value: i32) -> NodeTypeHealthPolicyMapItem {
        NodeTypeHealthPolicyMapItem { key, value }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn with_key(mut self, key: String) -> NodeTypeHealthPolicyMapItem {
        self.key = key;
        self
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn with_value(mut self, value: i32) -> NodeTypeHealthPolicyMapItem {
        self.value = value;
        self
    }

    pub fn value(&self) -> &i32 {
        &self.value
    }
}
