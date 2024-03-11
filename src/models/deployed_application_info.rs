/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedApplicationInfo : Information about application deployed on the node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedApplicationInfo {
    /// The identity of the application. This is an encoded representation of the application name. This is used in the REST APIs to identify the application resource. Starting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the application name is \"fabric:/myapp/app1\", the application identity would be \"myapp\\~app1\" in 6.0+ and \"myapp/app1\" in previous versions.
    #[serde(rename = "Id")]
    id: Option<::models::ApplicationId>,
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: Option<::models::ApplicationName>,
    /// The application type name as defined in the application manifest.
    #[serde(rename = "TypeName")]
    type_name: Option<::models::ApplicationTypeName>,
    /// The status of the application deployed on the node. Following are the possible values.
    #[serde(rename = "Status")]
    status: Option<::models::DeployedApplicationStatus>,
    /// The work directory of the application on the node. The work directory can be used to store application data.
    #[serde(rename = "WorkDirectory")]
    work_directory: Option<String>,
    /// The log directory of the application on the node. The log directory can be used to store application logs.
    #[serde(rename = "LogDirectory")]
    log_directory: Option<String>,
    /// The temp directory of the application on the node. The code packages belonging to the application are forked with this directory set as their temporary directory.
    #[serde(rename = "TempDirectory")]
    temp_directory: Option<String>,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
}

impl Default for DeployedApplicationInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedApplicationInfo {
    /// Information about application deployed on the node.
    pub fn new() -> DeployedApplicationInfo {
        DeployedApplicationInfo {
            id: None,
            name: None,
            type_name: None,
            status: None,
            work_directory: None,
            log_directory: None,
            temp_directory: None,
            health_state: None,
        }
    }

    pub fn set_id(&mut self, id: ::models::ApplicationId) {
        self.id = Some(id);
    }

    pub fn with_id(
        mut self,
        id: ::models::ApplicationId,
    ) -> DeployedApplicationInfo {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&::models::ApplicationId> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_name(&mut self, name: ::models::ApplicationName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::ApplicationName,
    ) -> DeployedApplicationInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::ApplicationName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_type_name(&mut self, type_name: ::models::ApplicationTypeName) {
        self.type_name = Some(type_name);
    }

    pub fn with_type_name(
        mut self,
        type_name: ::models::ApplicationTypeName,
    ) -> DeployedApplicationInfo {
        self.type_name = Some(type_name);
        self
    }

    pub fn type_name(&self) -> Option<&::models::ApplicationTypeName> {
        self.type_name.as_ref()
    }

    pub fn reset_type_name(&mut self) {
        self.type_name = None;
    }

    pub fn set_status(&mut self, status: ::models::DeployedApplicationStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::DeployedApplicationStatus,
    ) -> DeployedApplicationInfo {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::DeployedApplicationStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_work_directory(&mut self, work_directory: String) {
        self.work_directory = Some(work_directory);
    }

    pub fn with_work_directory(
        mut self,
        work_directory: String,
    ) -> DeployedApplicationInfo {
        self.work_directory = Some(work_directory);
        self
    }

    pub fn work_directory(&self) -> Option<&String> {
        self.work_directory.as_ref()
    }

    pub fn reset_work_directory(&mut self) {
        self.work_directory = None;
    }

    pub fn set_log_directory(&mut self, log_directory: String) {
        self.log_directory = Some(log_directory);
    }

    pub fn with_log_directory(
        mut self,
        log_directory: String,
    ) -> DeployedApplicationInfo {
        self.log_directory = Some(log_directory);
        self
    }

    pub fn log_directory(&self) -> Option<&String> {
        self.log_directory.as_ref()
    }

    pub fn reset_log_directory(&mut self) {
        self.log_directory = None;
    }

    pub fn set_temp_directory(&mut self, temp_directory: String) {
        self.temp_directory = Some(temp_directory);
    }

    pub fn with_temp_directory(
        mut self,
        temp_directory: String,
    ) -> DeployedApplicationInfo {
        self.temp_directory = Some(temp_directory);
        self
    }

    pub fn temp_directory(&self) -> Option<&String> {
        self.temp_directory.as_ref()
    }

    pub fn reset_temp_directory(&mut self) {
        self.temp_directory = None;
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> DeployedApplicationInfo {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }
}
