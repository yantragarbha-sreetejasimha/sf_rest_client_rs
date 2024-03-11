/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosParametersDictionaryItem : Defines an item in ChaosParametersDictionary of the Chaos Schedule.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosParametersDictionaryItem {
    /// The key identifying the Chaos Parameter in the dictionary. This key is referenced by Chaos Schedule Jobs.
    #[serde(rename = "Key")]
    key: String,
    /// Defines all the parameters to configure a Chaos run.
    #[serde(rename = "Value")]
    value: ::models::ChaosParameters,
}

impl ChaosParametersDictionaryItem {
    /// Defines an item in ChaosParametersDictionary of the Chaos Schedule.
    pub fn new(
        key: String,
        value: ::models::ChaosParameters,
    ) -> ChaosParametersDictionaryItem {
        ChaosParametersDictionaryItem { key, value }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn with_key(mut self, key: String) -> ChaosParametersDictionaryItem {
        self.key = key;
        self
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn set_value(&mut self, value: ::models::ChaosParameters) {
        self.value = value;
    }

    pub fn with_value(
        mut self,
        value: ::models::ChaosParameters,
    ) -> ChaosParametersDictionaryItem {
        self.value = value;
        self
    }

    pub fn value(&self) -> &::models::ChaosParameters {
        &self.value
    }
}
