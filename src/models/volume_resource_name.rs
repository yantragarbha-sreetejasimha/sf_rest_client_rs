/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// VolumeResourceName : Name of the Volume resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeResourceName {}

impl Default for VolumeResourceName {
    fn default() -> Self {
        Self::new()
    }
}

impl VolumeResourceName {
    /// Name of the Volume resource.
    pub fn new() -> VolumeResourceName {
        VolumeResourceName {}
    }
}
