/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExternalStoreProvisionApplicationTypeDescription : Describes the operation to register or provision an application type using an application package from an external store instead of a package uploaded to the Service Fabric image store.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalStoreProvisionApplicationTypeDescription {
    /// The kind of application type registration or provision requested. The application package can be registered or provisioned either from the image store or from an external store. Following are the kinds of the application type provision.
    #[serde(rename = "Kind")]
    kind: ::models::ProvisionApplicationTypeKind,
    /// Indicates whether or not provisioning should occur _asynchronously. When set to true, the provision operation returns when the request is accepted by the system, and the provision operation continues without any timeout limit. The default value is false. For large application packages, we recommend setting the value to true.
    #[serde(rename = "Async")]
    _async: bool,
    /// The path to the '.sfpkg' application package from where the application package can be downloaded using HTTP or HTTPS protocols. The application package can be stored in an external store that provides GET operation to download the file. Supported protocols are HTTP and HTTPS, and the path must allow READ access.
    #[serde(rename = "ApplicationPackageDownloadUri")]
    application_package_download_uri: String,
    /// The application type name represents the name of the application type found in the application manifest.
    #[serde(rename = "ApplicationTypeName")]
    application_type_name: String,
    /// The application type version represents the version of the application type found in the application manifest.
    #[serde(rename = "ApplicationTypeVersion")]
    application_type_version: String,
}

impl ExternalStoreProvisionApplicationTypeDescription {
    /// Describes the operation to register or provision an application type using an application package from an external store instead of a package uploaded to the Service Fabric image store.
    pub fn new(
        kind: ::models::ProvisionApplicationTypeKind,
        _async: bool,
        application_package_download_uri: String,
        application_type_name: String,
        application_type_version: String,
    ) -> ExternalStoreProvisionApplicationTypeDescription {
        ExternalStoreProvisionApplicationTypeDescription {
            kind,
            _async,
            application_package_download_uri,
            application_type_name,
            application_type_version,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ProvisionApplicationTypeKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ProvisionApplicationTypeKind,
    ) -> ExternalStoreProvisionApplicationTypeDescription {
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
    ) -> ExternalStoreProvisionApplicationTypeDescription {
        self._async = _async;
        self
    }

    pub fn _async(&self) -> &bool {
        &self._async
    }

    pub fn set_application_package_download_uri(
        &mut self,
        application_package_download_uri: String,
    ) {
        self.application_package_download_uri =
            application_package_download_uri;
    }

    pub fn with_application_package_download_uri(
        mut self,
        application_package_download_uri: String,
    ) -> ExternalStoreProvisionApplicationTypeDescription {
        self.application_package_download_uri =
            application_package_download_uri;
        self
    }

    pub fn application_package_download_uri(&self) -> &String {
        &self.application_package_download_uri
    }

    pub fn set_application_type_name(&mut self, application_type_name: String) {
        self.application_type_name = application_type_name;
    }

    pub fn with_application_type_name(
        mut self,
        application_type_name: String,
    ) -> ExternalStoreProvisionApplicationTypeDescription {
        self.application_type_name = application_type_name;
        self
    }

    pub fn application_type_name(&self) -> &String {
        &self.application_type_name
    }

    pub fn set_application_type_version(
        &mut self,
        application_type_version: String,
    ) {
        self.application_type_version = application_type_version;
    }

    pub fn with_application_type_version(
        mut self,
        application_type_version: String,
    ) -> ExternalStoreProvisionApplicationTypeDescription {
        self.application_type_version = application_type_version;
        self
    }

    pub fn application_type_version(&self) -> &String {
        &self.application_type_version
    }
}
