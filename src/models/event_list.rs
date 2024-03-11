/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EventList : A list of FabricEvent objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventList {}

impl Default for EventList {
    fn default() -> Self {
        Self::new()
    }
}

impl EventList {
    /// A list of FabricEvent objects.
    pub fn new() -> EventList {
        EventList {}
    }
}
