/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MoveCost : Specifies the move cost for the service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveCost {}

impl Default for MoveCost {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveCost {
    /// Specifies the move cost for the service.
    pub fn new() -> MoveCost {
        MoveCost {}
    }
}

// TODO enum
// List of MoveCost
//const (
//
//
//
//)
