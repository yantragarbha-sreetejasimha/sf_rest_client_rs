/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScalingPolicyDescriptionList : A list that describes the scaling policies.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalingPolicyDescriptionList {}

impl Default for ScalingPolicyDescriptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl ScalingPolicyDescriptionList {
    /// A list that describes the scaling policies.
    pub fn new() -> ScalingPolicyDescriptionList {
        ScalingPolicyDescriptionList {}
    }
}
