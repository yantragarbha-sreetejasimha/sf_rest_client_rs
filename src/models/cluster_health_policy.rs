/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ClusterHealthPolicy : Defines a health policy used to evaluate the health of the cluster or of a cluster node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterHealthPolicy {
    /// Indicates whether warnings are treated with the same severity as errors.
    #[serde(rename = "ConsiderWarningAsError")]
    consider_warning_as_error: Option<bool>,
    /// The maximum allowed percentage of unhealthy nodes before reporting an error. For example, to allow 10% of nodes to be unhealthy, this value would be 10.  The percentage represents the maximum tolerated percentage of nodes that can be unhealthy before the cluster is considered in error. If the percentage is respected but there is at least one unhealthy node, the health is evaluated as Warning. The percentage is calculated by dividing the number of unhealthy nodes over the total number of nodes in the cluster. The computation rounds up to tolerate one failure on small numbers of nodes. Default percentage is zero.  In large clusters, some nodes will always be down or out for repairs, so this percentage should be configured to tolerate that.
    #[serde(rename = "MaxPercentUnhealthyNodes")]
    max_percent_unhealthy_nodes: Option<i32>,
    /// The maximum allowed percentage of unhealthy applications before reporting an error. For example, to allow 10% of applications to be unhealthy, this value would be 10.  The percentage represents the maximum tolerated percentage of applications that can be unhealthy before the cluster is considered in error. If the percentage is respected but there is at least one unhealthy application, the health is evaluated as Warning. This is calculated by dividing the number of unhealthy applications over the total number of application instances in the cluster, excluding applications of application types that are included in the ApplicationTypeHealthPolicyMap. The computation rounds up to tolerate one failure on small numbers of applications. Default percentage is zero.
    #[serde(rename = "MaxPercentUnhealthyApplications")]
    max_percent_unhealthy_applications: Option<i32>,
    /// Defines a map with max percentage unhealthy applications for specific application types. Each entry specifies as key the application type name and as value an integer that represents the MaxPercentUnhealthyApplications percentage used to evaluate the applications of the specified application type.  The application type health policy map can be used during cluster health evaluation to describe special application types. The application types included in the map are evaluated against the percentage specified in the map, and not with the global MaxPercentUnhealthyApplications defined in the cluster health policy. The applications of application types specified in the map are not counted against the global pool of applications. For example, if some applications of a type are critical, the cluster administrator can add an entry to the map for that application type and assign it a value of 0% (that is, do not tolerate any failures). All other applications can be evaluated with MaxPercentUnhealthyApplications set to 20% to tolerate some failures out of the thousands of application instances. The application type health policy map is used only if the cluster manifest enables application type health evaluation using the configuration entry for HealthManager/EnableApplicationTypeHealthEvaluation.
    #[serde(rename = "ApplicationTypeHealthPolicyMap")]
    application_type_health_policy_map:
        Option<::models::ApplicationTypeHealthPolicyMap>,
}

impl Default for ClusterHealthPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterHealthPolicy {
    /// Defines a health policy used to evaluate the health of the cluster or of a cluster node.
    pub fn new() -> ClusterHealthPolicy {
        ClusterHealthPolicy {
            consider_warning_as_error: None,
            max_percent_unhealthy_nodes: None,
            max_percent_unhealthy_applications: None,
            application_type_health_policy_map: None,
        }
    }

    pub fn set_consider_warning_as_error(
        &mut self,
        consider_warning_as_error: bool,
    ) {
        self.consider_warning_as_error = Some(consider_warning_as_error);
    }

    pub fn with_consider_warning_as_error(
        mut self,
        consider_warning_as_error: bool,
    ) -> ClusterHealthPolicy {
        self.consider_warning_as_error = Some(consider_warning_as_error);
        self
    }

    pub fn consider_warning_as_error(&self) -> Option<&bool> {
        self.consider_warning_as_error.as_ref()
    }

    pub fn reset_consider_warning_as_error(&mut self) {
        self.consider_warning_as_error = None;
    }

    pub fn set_max_percent_unhealthy_nodes(
        &mut self,
        max_percent_unhealthy_nodes: i32,
    ) {
        self.max_percent_unhealthy_nodes = Some(max_percent_unhealthy_nodes);
    }

    pub fn with_max_percent_unhealthy_nodes(
        mut self,
        max_percent_unhealthy_nodes: i32,
    ) -> ClusterHealthPolicy {
        self.max_percent_unhealthy_nodes = Some(max_percent_unhealthy_nodes);
        self
    }

    pub fn max_percent_unhealthy_nodes(&self) -> Option<&i32> {
        self.max_percent_unhealthy_nodes.as_ref()
    }

    pub fn reset_max_percent_unhealthy_nodes(&mut self) {
        self.max_percent_unhealthy_nodes = None;
    }

    pub fn set_max_percent_unhealthy_applications(
        &mut self,
        max_percent_unhealthy_applications: i32,
    ) {
        self.max_percent_unhealthy_applications =
            Some(max_percent_unhealthy_applications);
    }

    pub fn with_max_percent_unhealthy_applications(
        mut self,
        max_percent_unhealthy_applications: i32,
    ) -> ClusterHealthPolicy {
        self.max_percent_unhealthy_applications =
            Some(max_percent_unhealthy_applications);
        self
    }

    pub fn max_percent_unhealthy_applications(&self) -> Option<&i32> {
        self.max_percent_unhealthy_applications.as_ref()
    }

    pub fn reset_max_percent_unhealthy_applications(&mut self) {
        self.max_percent_unhealthy_applications = None;
    }

    pub fn set_application_type_health_policy_map(
        &mut self,
        application_type_health_policy_map: ::models::ApplicationTypeHealthPolicyMap,
    ) {
        self.application_type_health_policy_map =
            Some(application_type_health_policy_map);
    }

    pub fn with_application_type_health_policy_map(
        mut self,
        application_type_health_policy_map: ::models::ApplicationTypeHealthPolicyMap,
    ) -> ClusterHealthPolicy {
        self.application_type_health_policy_map =
            Some(application_type_health_policy_map);
        self
    }

    pub fn application_type_health_policy_map(
        &self,
    ) -> Option<&::models::ApplicationTypeHealthPolicyMap> {
        self.application_type_health_policy_map.as_ref()
    }

    pub fn reset_application_type_health_policy_map(&mut self) {
        self.application_type_health_policy_map = None;
    }
}
