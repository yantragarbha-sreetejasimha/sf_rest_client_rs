/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ImageStoreRelativePath : The remote location within image store. This path is relative to the image store root.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageStoreRelativePath {}

impl Default for ImageStoreRelativePath {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageStoreRelativePath {
    /// The remote location within image store. This path is relative to the image store root.
    pub fn new() -> ImageStoreRelativePath {
        ImageStoreRelativePath {}
    }
}
