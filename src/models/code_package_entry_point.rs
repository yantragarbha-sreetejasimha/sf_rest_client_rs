/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CodePackageEntryPoint : Information about setup or main entry point of a code package deployed on a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePackageEntryPoint {
    /// The location of entry point executable on the node.
    #[serde(rename = "EntryPointLocation")]
    entry_point_location: Option<String>,
    /// The process ID of the entry point.
    #[serde(rename = "ProcessId")]
    process_id: Option<String>,
    /// The user name under which entry point executable is run on the node.
    #[serde(rename = "RunAsUserName")]
    run_as_user_name: Option<String>,
    /// Statistics about setup or main entry point  of a code package deployed on a Service Fabric node.
    #[serde(rename = "CodePackageEntryPointStatistics")]
    code_package_entry_point_statistics:
        Option<::models::CodePackageEntryPointStatistics>,
    /// Specifies the status of the code package entry point deployed on a Service Fabric node.
    #[serde(rename = "Status")]
    status: Option<::models::EntryPointStatus>,
    /// The time (in UTC) when the entry point executable will be run next.
    #[serde(rename = "NextActivationTime")]
    next_activation_time: Option<String>,
    /// The instance ID for current running entry point. For a code package setup entry point (if specified) runs first and after it finishes main entry point is started. Each time entry point executable is run, its instance id will change.
    #[serde(rename = "InstanceId")]
    instance_id: Option<::models::CodePackageInstanceId>,
}

impl Default for CodePackageEntryPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl CodePackageEntryPoint {
    /// Information about setup or main entry point of a code package deployed on a Service Fabric node.
    pub fn new() -> CodePackageEntryPoint {
        CodePackageEntryPoint {
            entry_point_location: None,
            process_id: None,
            run_as_user_name: None,
            code_package_entry_point_statistics: None,
            status: None,
            next_activation_time: None,
            instance_id: None,
        }
    }

    pub fn set_entry_point_location(&mut self, entry_point_location: String) {
        self.entry_point_location = Some(entry_point_location);
    }

    pub fn with_entry_point_location(
        mut self,
        entry_point_location: String,
    ) -> CodePackageEntryPoint {
        self.entry_point_location = Some(entry_point_location);
        self
    }

    pub fn entry_point_location(&self) -> Option<&String> {
        self.entry_point_location.as_ref()
    }

    pub fn reset_entry_point_location(&mut self) {
        self.entry_point_location = None;
    }

    pub fn set_process_id(&mut self, process_id: String) {
        self.process_id = Some(process_id);
    }

    pub fn with_process_id(
        mut self,
        process_id: String,
    ) -> CodePackageEntryPoint {
        self.process_id = Some(process_id);
        self
    }

    pub fn process_id(&self) -> Option<&String> {
        self.process_id.as_ref()
    }

    pub fn reset_process_id(&mut self) {
        self.process_id = None;
    }

    pub fn set_run_as_user_name(&mut self, run_as_user_name: String) {
        self.run_as_user_name = Some(run_as_user_name);
    }

    pub fn with_run_as_user_name(
        mut self,
        run_as_user_name: String,
    ) -> CodePackageEntryPoint {
        self.run_as_user_name = Some(run_as_user_name);
        self
    }

    pub fn run_as_user_name(&self) -> Option<&String> {
        self.run_as_user_name.as_ref()
    }

    pub fn reset_run_as_user_name(&mut self) {
        self.run_as_user_name = None;
    }

    pub fn set_code_package_entry_point_statistics(
        &mut self,
        code_package_entry_point_statistics: ::models::CodePackageEntryPointStatistics,
    ) {
        self.code_package_entry_point_statistics =
            Some(code_package_entry_point_statistics);
    }

    pub fn with_code_package_entry_point_statistics(
        mut self,
        code_package_entry_point_statistics: ::models::CodePackageEntryPointStatistics,
    ) -> CodePackageEntryPoint {
        self.code_package_entry_point_statistics =
            Some(code_package_entry_point_statistics);
        self
    }

    pub fn code_package_entry_point_statistics(
        &self,
    ) -> Option<&::models::CodePackageEntryPointStatistics> {
        self.code_package_entry_point_statistics.as_ref()
    }

    pub fn reset_code_package_entry_point_statistics(&mut self) {
        self.code_package_entry_point_statistics = None;
    }

    pub fn set_status(&mut self, status: ::models::EntryPointStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::EntryPointStatus,
    ) -> CodePackageEntryPoint {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::EntryPointStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_next_activation_time(&mut self, next_activation_time: String) {
        self.next_activation_time = Some(next_activation_time);
    }

    pub fn with_next_activation_time(
        mut self,
        next_activation_time: String,
    ) -> CodePackageEntryPoint {
        self.next_activation_time = Some(next_activation_time);
        self
    }

    pub fn next_activation_time(&self) -> Option<&String> {
        self.next_activation_time.as_ref()
    }

    pub fn reset_next_activation_time(&mut self) {
        self.next_activation_time = None;
    }

    pub fn set_instance_id(
        &mut self,
        instance_id: ::models::CodePackageInstanceId,
    ) {
        self.instance_id = Some(instance_id);
    }

    pub fn with_instance_id(
        mut self,
        instance_id: ::models::CodePackageInstanceId,
    ) -> CodePackageEntryPoint {
        self.instance_id = Some(instance_id);
        self
    }

    pub fn instance_id(&self) -> Option<&::models::CodePackageInstanceId> {
        self.instance_id.as_ref()
    }

    pub fn reset_instance_id(&mut self) {
        self.instance_id = None;
    }
}
