/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeployServicePackageToNodeDescription : Defines description for downloading packages associated with a service manifest to image cache on a Service Fabric node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployServicePackageToNodeDescription {
    /// The name of service manifest whose packages need to be downloaded.
    #[serde(rename = "ServiceManifestName")]
    service_manifest_name: ::models::ServiceManifestName,
    /// The application type name as defined in the application manifest.
    #[serde(rename = "ApplicationTypeName")]
    application_type_name: ::models::ApplicationTypeName,
    /// The version of the application type as defined in the application manifest.
    #[serde(rename = "ApplicationTypeVersion")]
    application_type_version: ::models::ApplicationTypeVersion,
    /// The name of a Service Fabric node.
    #[serde(rename = "NodeName")]
    node_name: ::models::NodeName,
    /// List of package sharing policy information.
    #[serde(rename = "PackageSharingPolicy")]
    package_sharing_policy: Option<::models::PackageSharingPolicyInfoList>,
}

impl DeployServicePackageToNodeDescription {
    /// Defines description for downloading packages associated with a service manifest to image cache on a Service Fabric node.
    pub fn new(
        service_manifest_name: ::models::ServiceManifestName,
        application_type_name: ::models::ApplicationTypeName,
        application_type_version: ::models::ApplicationTypeVersion,
        node_name: ::models::NodeName,
    ) -> DeployServicePackageToNodeDescription {
        DeployServicePackageToNodeDescription {
            service_manifest_name,
            application_type_name,
            application_type_version,
            node_name,
            package_sharing_policy: None,
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
    ) -> DeployServicePackageToNodeDescription {
        self.service_manifest_name = service_manifest_name;
        self
    }

    pub fn service_manifest_name(&self) -> &::models::ServiceManifestName {
        &self.service_manifest_name
    }

    pub fn set_application_type_name(
        &mut self,
        application_type_name: ::models::ApplicationTypeName,
    ) {
        self.application_type_name = application_type_name;
    }

    pub fn with_application_type_name(
        mut self,
        application_type_name: ::models::ApplicationTypeName,
    ) -> DeployServicePackageToNodeDescription {
        self.application_type_name = application_type_name;
        self
    }

    pub fn application_type_name(&self) -> &::models::ApplicationTypeName {
        &self.application_type_name
    }

    pub fn set_application_type_version(
        &mut self,
        application_type_version: ::models::ApplicationTypeVersion,
    ) {
        self.application_type_version = application_type_version;
    }

    pub fn with_application_type_version(
        mut self,
        application_type_version: ::models::ApplicationTypeVersion,
    ) -> DeployServicePackageToNodeDescription {
        self.application_type_version = application_type_version;
        self
    }

    pub fn application_type_version(
        &self,
    ) -> &::models::ApplicationTypeVersion {
        &self.application_type_version
    }

    pub fn set_node_name(&mut self, node_name: ::models::NodeName) {
        self.node_name = node_name;
    }

    pub fn with_node_name(
        mut self,
        node_name: ::models::NodeName,
    ) -> DeployServicePackageToNodeDescription {
        self.node_name = node_name;
        self
    }

    pub fn node_name(&self) -> &::models::NodeName {
        &self.node_name
    }

    pub fn set_package_sharing_policy(
        &mut self,
        package_sharing_policy: ::models::PackageSharingPolicyInfoList,
    ) {
        self.package_sharing_policy = Some(package_sharing_policy);
    }

    pub fn with_package_sharing_policy(
        mut self,
        package_sharing_policy: ::models::PackageSharingPolicyInfoList,
    ) -> DeployServicePackageToNodeDescription {
        self.package_sharing_policy = Some(package_sharing_policy);
        self
    }

    pub fn package_sharing_policy(
        &self,
    ) -> Option<&::models::PackageSharingPolicyInfoList> {
        self.package_sharing_policy.as_ref()
    }

    pub fn reset_package_sharing_policy(&mut self) {
        self.package_sharing_policy = None;
    }
}
