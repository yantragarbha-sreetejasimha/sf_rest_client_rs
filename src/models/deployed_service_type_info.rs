/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployedServiceTypeInfo : Information about service type deployed on a node, information such as the status of the service type registration on a node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedServiceTypeInfo {
    /// Name of the service type as specified in the service manifest.
    #[serde(rename = "ServiceTypeName")]
    service_type_name: Option<::models::ServiceTypeName>,
    /// The name of the service manifest in which this service type is defined.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: Option<::models::ServiceManifestName>,
    /// The name of the code package that registered the service type.
    #[serde(rename = "CodePackageName")]
    code_package_name: Option<::models::CodePackageName>,
    /// The status of the service type registration on the node.
    #[serde(rename = "Status")]
    status: Option<::models::ServiceTypeRegistrationStatus>,
    /// The ActivationId of a deployed service package. If ServicePackageActivationMode specified at the time of creating the service is 'SharedProcess' (or if it is not specified, in which case it defaults to 'SharedProcess'), then value of ServicePackageActivationId is always an empty string.
    #[serde(rename = "ServicePackageActivationId")]
    service_package_activation_id: Option<::models::ServicePackageActivationId>,
}

impl Default for DeployedServiceTypeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployedServiceTypeInfo {
    /// Information about service type deployed on a node, information such as the status of the service type registration on a node.
    pub fn new() -> DeployedServiceTypeInfo {
        DeployedServiceTypeInfo {
            service_type_name: None,
            service_manifest_name: None,
            code_package_name: None,
            status: None,
            service_package_activation_id: None,
        }
    }

    pub fn set_service_type_name(
        &mut self,
        service_type_name: ::models::ServiceTypeName,
    ) {
        self.service_type_name = Some(service_type_name);
    }

    pub fn with_service_type_name(
        mut self,
        service_type_name: ::models::ServiceTypeName,
    ) -> DeployedServiceTypeInfo {
        self.service_type_name = Some(service_type_name);
        self
    }

    pub fn service_type_name(&self) -> Option<&::models::ServiceTypeName> {
        self.service_type_name.as_ref()
    }

    pub fn reset_service_type_name(&mut self) {
        self.service_type_name = None;
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
    ) -> DeployedServiceTypeInfo {
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

    pub fn set_code_package_name(
        &mut self,
        code_package_name: ::models::CodePackageName,
    ) {
        self.code_package_name = Some(code_package_name);
    }

    pub fn with_code_package_name(
        mut self,
        code_package_name: ::models::CodePackageName,
    ) -> DeployedServiceTypeInfo {
        self.code_package_name = Some(code_package_name);
        self
    }

    pub fn code_package_name(&self) -> Option<&::models::CodePackageName> {
        self.code_package_name.as_ref()
    }

    pub fn reset_code_package_name(&mut self) {
        self.code_package_name = None;
    }

    pub fn set_status(
        &mut self,
        status: ::models::ServiceTypeRegistrationStatus,
    ) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ServiceTypeRegistrationStatus,
    ) -> DeployedServiceTypeInfo {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::ServiceTypeRegistrationStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
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
    ) -> DeployedServiceTypeInfo {
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
}
