/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ProvisionApplicationTypeDescription : Describes the operation to register or provision an application type using an application package uploaded to the Service Fabric image store.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionApplicationTypeDescription {
    /// The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision.
    #[serde(rename = "Kind")]
    kind: ::models::ProvisionApplicationTypeKind,
    /// Indicates whether or not provisioning should occur _asynchronously. When set to true, the provision operation returns when the request is accepted by the system, and the provision operation continues without any timeout limit. The default value is false. For large application packages, we recommend setting the value to true.
    #[serde(rename = "Async")]
    _async: bool,
    /// The relative path for the application package in the image store specified during the prior upload operation.
    #[serde(rename = "ApplicationTypeBuildPath")]
    application_type_build_path: String,
    /// The kind of action that needs to be taken for cleaning up the application package after successful provision.
    #[serde(rename = "ApplicationPackageCleanupPolicy")]
    application_package_cleanup_policy:
        Option<::models::ApplicationPackageCleanupPolicy>,
}

impl ProvisionApplicationTypeDescription {
    /// Describes the operation to register or provision an application type using an application package uploaded to the Service Fabric image store.
    pub fn new(
        kind: ::models::ProvisionApplicationTypeKind,
        _async: bool,
        application_type_build_path: String,
    ) -> ProvisionApplicationTypeDescription {
        ProvisionApplicationTypeDescription {
            kind,
            _async,
            application_type_build_path,
            application_package_cleanup_policy: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ProvisionApplicationTypeKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ProvisionApplicationTypeKind,
    ) -> ProvisionApplicationTypeDescription {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ProvisionApplicationTypeKind {
        &self.kind
    }

    pub fn set_async(&mut self, _async: bool) {
        self._async = _async;
    }

    pub fn with_async(
        mut self,
        _async: bool,
    ) -> ProvisionApplicationTypeDescription {
        self._async = _async;
        self
    }

    pub fn _async(&self) -> &bool {
        &self._async
    }

    pub fn set_application_type_build_path(
        &mut self,
        application_type_build_path: String,
    ) {
        self.application_type_build_path = application_type_build_path;
    }

    pub fn with_application_type_build_path(
        mut self,
        application_type_build_path: String,
    ) -> ProvisionApplicationTypeDescription {
        self.application_type_build_path = application_type_build_path;
        self
    }

    pub fn application_type_build_path(&self) -> &String {
        &self.application_type_build_path
    }

    pub fn set_application_package_cleanup_policy(
        &mut self,
        application_package_cleanup_policy: ::models::ApplicationPackageCleanupPolicy,
    ) {
        self.application_package_cleanup_policy =
            Some(application_package_cleanup_policy);
    }

    pub fn with_application_package_cleanup_policy(
        mut self,
        application_package_cleanup_policy: ::models::ApplicationPackageCleanupPolicy,
    ) -> ProvisionApplicationTypeDescription {
        self.application_package_cleanup_policy =
            Some(application_package_cleanup_policy);
        self
    }

    pub fn application_package_cleanup_policy(
        &self,
    ) -> Option<&::models::ApplicationPackageCleanupPolicy> {
        self.application_package_cleanup_policy.as_ref()
    }

    pub fn reset_application_package_cleanup_policy(&mut self) {
        self.application_package_cleanup_policy = None;
    }
}