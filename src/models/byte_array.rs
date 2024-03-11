/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ByteArray : Array of bytes to be sent as an integer array. Each element of array is a number between 0 and 255.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ByteArray {}

impl Default for ByteArray {
    fn default() -> Self {
        Self::new()
    }
}

impl ByteArray {
    /// Array of bytes to be sent as an integer array. Each element of array is a number between 0 and 255.
    pub fn new() -> ByteArray {
        ByteArray {}
    }
}
