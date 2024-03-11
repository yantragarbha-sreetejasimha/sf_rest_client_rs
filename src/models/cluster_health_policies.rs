/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterHealthPolicies : Health policies to evaluate cluster health.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterHealthPolicies {
    /// Defines a map that contains specific application health policies for different applications. Each entry specifies as key the application name and as value an ApplicationHealthPolicy used to evaluate the application health. If an application is not specified in the map, the application health evaluation uses the ApplicationHealthPolicy found in its application manifest or the default application health policy (if no health policy is defined in the manifest). The map is empty by default.
    #[serde(rename = "ApplicationHealthPolicyMap")]
    application_health_policy_map: Option<::models::ApplicationHealthPolicyMap>,
    /// Defines a health policy used to evaluate the health of the cluster or of a cluster node.
    #[serde(rename = "ClusterHealthPolicy")]
    cluster_health_policy: Option<::models::ClusterHealthPolicy>,
}

impl Default for ClusterHealthPolicies {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterHealthPolicies {
    /// Health policies to evaluate cluster health.
    pub fn new() -> ClusterHealthPolicies {
        ClusterHealthPolicies {
            application_health_policy_map: None,
            cluster_health_policy: None,
        }
    }

    pub fn set_application_health_policy_map(
        &mut self,
        application_health_policy_map: ::models::ApplicationHealthPolicyMap,
    ) {
        self.application_health_policy_map =
            Some(application_health_policy_map);
    }

    pub fn with_application_health_policy_map(
        mut self,
        application_health_policy_map: ::models::ApplicationHealthPolicyMap,
    ) -> ClusterHealthPolicies {
        self.application_health_policy_map =
            Some(application_health_policy_map);
        self
    }

    pub fn application_health_policy_map(
        &self,
    ) -> Option<&::models::ApplicationHealthPolicyMap> {
        self.application_health_policy_map.as_ref()
    }

    pub fn reset_application_health_policy_map(&mut self) {
        self.application_health_policy_map = None;
    }

    pub fn set_cluster_health_policy(
        &mut self,
        cluster_health_policy: ::models::ClusterHealthPolicy,
    ) {
        self.cluster_health_policy = Some(cluster_health_policy);
    }

    pub fn with_cluster_health_policy(
        mut self,
        cluster_health_policy: ::models::ClusterHealthPolicy,
    ) -> ClusterHealthPolicies {
        self.cluster_health_policy = Some(cluster_health_policy);
        self
    }

    pub fn cluster_health_policy(
        &self,
    ) -> Option<&::models::ClusterHealthPolicy> {
        self.cluster_health_policy.as_ref()
    }

    pub fn reset_cluster_health_policy(&mut self) {
        self.cluster_health_policy = None;
    }
}
