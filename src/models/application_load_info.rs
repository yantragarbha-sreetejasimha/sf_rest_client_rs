/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationLoadInfo : Load Information about a Service Fabric application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationLoadInfo {
    /// The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\", the application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions.
    #[serde(rename = "Id")]
    id: Option<::models::ApplicationId>,
    /// The minimum number of nodes for this application. It is the number of nodes where Service Fabric will reserve Capacity in the cluster which equals to ReservedLoad * MinimumNodes for this Application instance. For applications that do not have application capacity defined this value will be zero.
    #[serde(rename = "MinimumNodes")]
    minimum_nodes: Option<i64>,
    /// The maximum number of nodes where this application can be instantiated. It is the number of nodes this application is allowed to span. For applications that do not have application capacity defined this value will be zero.
    #[serde(rename = "MaximumNodes")]
    maximum_nodes: Option<i64>,
    /// The number of nodes on which this application is instantiated. For applications that do not have application capacity defined this value will be zero.
    #[serde(rename = "NodeCount")]
    node_count: Option<i64>,
    /// List of application capacity metric description.
    #[serde(rename = "ApplicationLoadMetricInformation")]
    application_load_metric_information:
        Option<::models::ApplicationMetricDescriptionList>,
}

impl Default for ApplicationLoadInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationLoadInfo {
    /// Load Information about a Service Fabric application.
    pub fn new() -> ApplicationLoadInfo {
        ApplicationLoadInfo {
            id: None,
            minimum_nodes: None,
            maximum_nodes: None,
            node_count: None,
            application_load_metric_information: None,
        }
    }

    pub fn set_id(&mut self, id: ::models::ApplicationId) {
        self.id = Some(id);
    }

    pub fn with_id(
        mut self,
        id: ::models::ApplicationId,
    ) -> ApplicationLoadInfo {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&::models::ApplicationId> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_minimum_nodes(&mut self, minimum_nodes: i64) {
        self.minimum_nodes = Some(minimum_nodes);
    }

    pub fn with_minimum_nodes(
        mut self,
        minimum_nodes: i64,
    ) -> ApplicationLoadInfo {
        self.minimum_nodes = Some(minimum_nodes);
        self
    }

    pub fn minimum_nodes(&self) -> Option<&i64> {
        self.minimum_nodes.as_ref()
    }

    pub fn reset_minimum_nodes(&mut self) {
        self.minimum_nodes = None;
    }

    pub fn set_maximum_nodes(&mut self, maximum_nodes: i64) {
        self.maximum_nodes = Some(maximum_nodes);
    }

    pub fn with_maximum_nodes(
        mut self,
        maximum_nodes: i64,
    ) -> ApplicationLoadInfo {
        self.maximum_nodes = Some(maximum_nodes);
        self
    }

    pub fn maximum_nodes(&self) -> Option<&i64> {
        self.maximum_nodes.as_ref()
    }

    pub fn reset_maximum_nodes(&mut self) {
        self.maximum_nodes = None;
    }

    pub fn set_node_count(&mut self, node_count: i64) {
        self.node_count = Some(node_count);
    }

    pub fn with_node_count(mut self, node_count: i64) -> ApplicationLoadInfo {
        self.node_count = Some(node_count);
        self
    }

    pub fn node_count(&self) -> Option<&i64> {
        self.node_count.as_ref()
    }

    pub fn reset_node_count(&mut self) {
        self.node_count = None;
    }

    pub fn set_application_load_metric_information(
        &mut self,
        application_load_metric_information: ::models::ApplicationMetricDescriptionList,
    ) {
        self.application_load_metric_information =
            Some(application_load_metric_information);
    }

    pub fn with_application_load_metric_information(
        mut self,
        application_load_metric_information: ::models::ApplicationMetricDescriptionList,
    ) -> ApplicationLoadInfo {
        self.application_load_metric_information =
            Some(application_load_metric_information);
        self
    }

    pub fn application_load_metric_information(
        &self,
    ) -> Option<&::models::ApplicationMetricDescriptionList> {
        self.application_load_metric_information.as_ref()
    }

    pub fn reset_application_load_metric_information(&mut self) {
        self.application_load_metric_information = None;
    }
}
