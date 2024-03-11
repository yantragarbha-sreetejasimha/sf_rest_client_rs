/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosEventWrapper : Wrapper object for Chaos event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosEventWrapper {
    /// Represents an event generated during a Chaos run.
    #[serde(rename = "ChaosEvent")]
    chaos_event: Option<::models::ChaosEvent>,
}

impl Default for ChaosEventWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosEventWrapper {
    /// Wrapper object for Chaos event.
    pub fn new() -> ChaosEventWrapper {
        ChaosEventWrapper { chaos_event: None }
    }

    pub fn set_chaos_event(&mut self, chaos_event: ::models::ChaosEvent) {
        self.chaos_event = Some(chaos_event);
    }

    pub fn with_chaos_event(
        mut self,
        chaos_event: ::models::ChaosEvent,
    ) -> ChaosEventWrapper {
        self.chaos_event = Some(chaos_event);
        self
    }

    pub fn chaos_event(&self) -> Option<&::models::ChaosEvent> {
        self.chaos_event.as_ref()
    }

    pub fn reset_chaos_event(&mut self) {
        self.chaos_event = None;
    }
}
