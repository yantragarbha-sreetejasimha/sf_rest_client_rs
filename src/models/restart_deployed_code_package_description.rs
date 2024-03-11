/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RestartDeployedCodePackageDescription : Defines description for restarting a deployed code package on Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RestartDeployedCodePackageDescription {
    /// The name of service manifest that specified this code package.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: ::models::ServiceManifestName,
    /// The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId is always an empty string.
    #[serde(rename = "ServicePackageActivationId")]
    service_package_activation_id: Option<::models::ServicePackageActivationId>,
    /// The name of the code package defined in the service manifest.
    #[serde(rename = "CodePackageName")]
    code_package_name: ::models::CodePackageName,
    /// The instance ID for currently running entry point. For a code package setup entry point (if specified) runs first and after it finishes main entry point is started. Each time entry point executable is run, its instance ID will change. If 0 is passed in as the code package instance ID, the API will restart the code package with whatever instance ID it is currently running. If an instance ID other than 0 is passed in, the API will restart the code package only if the current Instance ID matches the passed in instance ID. Note, passing in the exact instance ID (not 0) in the API is safer, because if ensures at most one restart of the code package.
    #[serde(rename = "CodePackageInstanceId")]
    code_package_instance_id: ::models::CodePackageInstanceId,
}

impl RestartDeployedCodePackageDescription {
    /// Defines description for restarting a deployed code package on Service Fabric node.
    pub fn new(
        service_manifest_name: ::models::ServiceManifestName,
        code_package_name: ::models::CodePackageName,
        code_package_instance_id: ::models::CodePackageInstanceId,
    ) -> RestartDeployedCodePackageDescription {
        RestartDeployedCodePackageDescription {
            service_manifest_name,
            service_package_activation_id: None,
            code_package_name,
            code_package_instance_id,
        }
    }

    pub fn set_service_manifest_name(
        &mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) {
        self.service_manifest_name = service_manifest_name;
    }

    pub fn with_service_manifest_name(
        mut self,
        service_manifest_name: ::models::ServiceManifestName,
    ) -> RestartDeployedCodePackageDescription {
        self.service_manifest_name = service_manifest_name;
        self
    }

    pub fn service_manifest_name(&self) -> &::models::ServiceManifestName {
        &self.service_manifest_name
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
    ) -> RestartDeployedCodePackageDescription {
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

    pub fn set_code_package_name(
        &mut self,
        code_package_name: ::models::CodePackageName,
    ) {
        self.code_package_name = code_package_name;
    }

    pub fn with_code_package_name(
        mut self,
        code_package_name: ::models::CodePackageName,
    ) -> RestartDeployedCodePackageDescription {
        self.code_package_name = code_package_name;
        self
    }

    pub fn code_package_name(&self) -> &::models::CodePackageName {
        &self.code_package_name
    }

    pub fn set_code_package_instance_id(
        &mut self,
        code_package_instance_id: ::models::CodePackageInstanceId,
    ) {
        self.code_package_instance_id = code_package_instance_id;
    }

    pub fn with_code_package_instance_id(
        mut self,
        code_package_instance_id: ::models::CodePackageInstanceId,
    ) -> RestartDeployedCodePackageDescription {
        self.code_package_instance_id = code_package_instance_id;
        self
    }

    pub fn code_package_instance_id(&self) -> &::models::CodePackageInstanceId {
        &self.code_package_instance_id
    }
}
