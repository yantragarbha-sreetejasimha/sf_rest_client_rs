/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosContext : Describes a map, which is a collection of (string, string) type key-value pairs. The map can be used to record information about the Chaos run. There cannot be more than 100 such pairs and each string (key or value) can be at most 4095 characters long. This map is set by the starter of the Chaos run to optionally store the context about the specific run.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosContext {
    /// Describes a map that contains a collection of ChaosContextMapItem's.
    #[serde(rename = "Map")]
    map: Option<::models::ChaosContextMap>,
}

impl Default for ChaosContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosContext {
    /// Describes a map, which is a collection of (string, string) type key-value pairs. The map can be used to record information about the Chaos run. There cannot be more than 100 such pairs and each string (key or value) can be at most 4095 characters long. This map is set by the starter of the Chaos run to optionally store the context about the specific run.
    pub fn new() -> ChaosContext {
        ChaosContext { map: None }
    }

    pub fn set_map(&mut self, map: ::models::ChaosContextMap) {
        self.map = Some(map);
    }

    pub fn with_map(mut self, map: ::models::ChaosContextMap) -> ChaosContext {
        self.map = Some(map);
        self
    }

    pub fn map(&self) -> Option<&::models::ChaosContextMap> {
        self.map.as_ref()
    }

    pub fn reset_map(&mut self) {
        self.map = None;
    }
}
