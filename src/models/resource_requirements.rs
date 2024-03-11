/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResourceRequirements : This type describes the resource requirements for a container or a service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Describes the requested resources for a given container.
    #[serde(rename = "requests")]
    requests: ::models::ResourceRequests,
    /// Describes the maximum limits on the resources for a given container.
    #[serde(rename = "limits")]
    limits: Option<::models::ResourceLimits>,
}

impl ResourceRequirements {
    /// This type describes the resource requirements for a container or a service.
    pub fn new(requests: ::models::ResourceRequests) -> ResourceRequirements {
        ResourceRequirements {
            requests,
            limits: None,
        }
    }

    pub fn set_requests(&mut self, requests: ::models::ResourceRequests) {
        self.requests = requests;
    }

    pub fn with_requests(
        mut self,
        requests: ::models::ResourceRequests,
    ) -> ResourceRequirements {
        self.requests = requests;
        self
    }

    pub fn requests(&self) -> &::models::ResourceRequests {
        &self.requests
    }

    pub fn set_limits(&mut self, limits: ::models::ResourceLimits) {
        self.limits = Some(limits);
    }

    pub fn with_limits(
        mut self,
        limits: ::models::ResourceLimits,
    ) -> ResourceRequirements {
        self.limits = Some(limits);
        self
    }

    pub fn limits(&self) -> Option<&::models::ResourceLimits> {
        self.limits.as_ref()
    }

    pub fn reset_limits(&mut self) {
        self.limits = None;
    }
}
