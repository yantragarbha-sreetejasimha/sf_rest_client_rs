/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedCodePackageInfo : Information about code package deployed on a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedCodePackageInfo {
    /// The name of the code package.
    #[serde(rename = "Name")]
    name: Option<::models::CodePackageName>,
    /// The version of the code package specified in service manifest.
    #[serde(rename = "Version")]
    version: Option<String>,
    /// The name of service manifest that specified this code package.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: Option<::models::ServiceManifestName>,
    /// The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId is always an empty string.
    #[serde(rename = "ServicePackageActivationId")]
    service_package_activation_id: Option<::models::ServicePackageActivationId>,
    /// Specifies the type of host for main entry point of a code package as specified in service manifest.
    #[serde(rename = "HostType")]
    host_type: Option<::models::HostType>,
    /// Specifies the isolation mode of main entry point of a code package when it's host type is ContainerHost. This is specified as part of container host policies in application manifest while importing service manifest.
    #[serde(rename = "HostIsolationMode")]
    host_isolation_mode: Option<::models::HostIsolationMode>,
    /// Specifies the status of a deployed application or service package on a Service Fabric node.
    #[serde(rename = "Status")]
    status: Option<::models::DeploymentStatus>,
    /// The interval at which code package is run. This is used for periodic code package.
    #[serde(rename = "RunFrequencyInterval")]
    run_frequency_interval: Option<String>,
    /// Information about setup or main entry point of a code package deployed on a Service Fabric node.
    #[serde(rename = "SetupEntryPoint")]
    setup_entry_point: Option<::models::CodePackageEntryPoint>,
    /// Information about setup or main entry point of a code package deployed on a Service Fabric node.
    #[serde(rename = "MainEntryPoint")]
    main_entry_point: Option<::models::CodePackageEntryPoint>,
}

impl Default for DeployedCodePackageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedCodePackageInfo {
    /// Information about code package deployed on a Service Fabric node.
    pub fn new() -> DeployedCodePackageInfo {
        DeployedCodePackageInfo {
            name: None,
            version: None,
            service_manifest_name: None,
            service_package_activation_id: None,
            host_type: None,
            host_isolation_mode: None,
            status: None,
            run_frequency_interval: None,
            setup_entry_point: None,
            main_entry_point: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::CodePackageName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::CodePackageName,
    ) -> DeployedCodePackageInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::CodePackageName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn with_version(mut self, version: String) -> DeployedCodePackageInfo {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn reset_version(&mut self) {
        self.version = None;
    }

    pub fn set_service_manifest_name(
        &mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) {
        self.service_manifest_name = Some(service_manifest_name);
    }

    pub fn with_service_manifest_name(
        mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) -> DeployedCodePackageInfo {
        self.service_manifest_name = Some(service_manifest_name);
        self
    }

    pub fn service_manifest_name(
        &self,
    ) -> Option<&::models::ServiceManifestName> {
        self.service_manifest_name.as_ref()
    }

    pub fn reset_service_manifest_name(&mut self) {
        self.service_manifest_name = None;
    }

    pub fn set_service_package_activation_id(
        &mut self,
        service_package_activation_id: ::models::ServicePackageActivationId,
    ) {
        self.service_package_activation_id =
            Some(service_package_activation_id);
    }

    pub fn with_service_package_activation_id(
        mut self,
        service_package_activation_id: ::models::ServicePackageActivationId,
    ) -> DeployedCodePackageInfo {
        self.service_package_activation_id =
            Some(service_package_activation_id);
        self
    }

    pub fn service_package_activation_id(
        &self,
    ) -> Option<&::models::ServicePackageActivationId> {
        self.service_package_activation_id.as_ref()
    }

    pub fn reset_service_package_activation_id(&mut self) {
        self.service_package_activation_id = None;
    }

    pub fn set_host_type(&mut self, host_type: ::models::HostType) {
        self.host_type = Some(host_type);
    }

    pub fn with_host_type(
        mut self,
        host_type: ::models::HostType,
    ) -> DeployedCodePackageInfo {
        self.host_type = Some(host_type);
        self
    }

    pub fn host_type(&self) -> Option<&::models::HostType> {
        self.host_type.as_ref()
    }

    pub fn reset_host_type(&mut self) {
        self.host_type = None;
    }

    pub fn set_host_isolation_mode(
        &mut self,
        host_isolation_mode: ::models::HostIsolationMode,
    ) {
        self.host_isolation_mode = Some(host_isolation_mode);
    }

    pub fn with_host_isolation_mode(
        mut self,
        host_isolation_mode: ::models::HostIsolationMode,
    ) -> DeployedCodePackageInfo {
        self.host_isolation_mode = Some(host_isolation_mode);
        self
    }

    pub fn host_isolation_mode(&self) -> Option<&::models::HostIsolationMode> {
        self.host_isolation_mode.as_ref()
    }

    pub fn reset_host_isolation_mode(&mut self) {
        self.host_isolation_mode = None;
    }

    pub fn set_status(&mut self, status: ::models::DeploymentStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::DeploymentStatus,
    ) -> DeployedCodePackageInfo {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::DeploymentStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_run_frequency_interval(
        &mut self,
        run_frequency_interval: String,
    ) {
        self.run_frequency_interval = Some(run_frequency_interval);
    }

    pub fn with_run_frequency_interval(
        mut self,
        run_frequency_interval: String,
    ) -> DeployedCodePackageInfo {
        self.run_frequency_interval = Some(run_frequency_interval);
        self
    }

    pub fn run_frequency_interval(&self) -> Option<&String> {
        self.run_frequency_interval.as_ref()
    }

    pub fn reset_run_frequency_interval(&mut self) {
        self.run_frequency_interval = None;
    }

    pub fn set_setup_entry_point(
        &mut self,
        setup_entry_point: ::models::CodePackageEntryPoint,
    ) {
        self.setup_entry_point = Some(setup_entry_point);
    }

    pub fn with_setup_entry_point(
        mut self,
        setup_entry_point: ::models::CodePackageEntryPoint,
    ) -> DeployedCodePackageInfo {
        self.setup_entry_point = Some(setup_entry_point);
        self
    }

    pub fn setup_entry_point(
        &self,
    ) -> Option<&::models::CodePackageEntryPoint> {
        self.setup_entry_point.as_ref()
    }

    pub fn reset_setup_entry_point(&mut self) {
        self.setup_entry_point = None;
    }

    pub fn set_main_entry_point(
        &mut self,
        main_entry_point: ::models::CodePackageEntryPoint,
    ) {
        self.main_entry_point = Some(main_entry_point);
    }

    pub fn with_main_entry_point(
        mut self,
        main_entry_point: ::models::CodePackageEntryPoint,
    ) -> DeployedCodePackageInfo {
        self.main_entry_point = Some(main_entry_point);
        self
    }

    pub fn main_entry_point(&self) -> Option<&::models::CodePackageEntryPoint> {
        self.main_entry_point.as_ref()
    }

    pub fn reset_main_entry_point(&mut self) {
        self.main_entry_point = None;
    }
}
