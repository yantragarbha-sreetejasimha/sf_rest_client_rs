/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PackageSharingPolicyInfo : Represents a policy for the package sharing.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSharingPolicyInfo {
    /// The name of code, configuration or data package that should be shared.
    #[serde(rename = "SharedPackageName")]
    shared_package_name: Option<String>,
    /// Represents the scope for PackageSharingPolicy. This is specified during DeployServicePackageToNode operation.
    #[serde(rename = "PackageSharingScope")]
    package_sharing_scope: Option<::models::PackageSharingPolicyScope>,
}

impl Default for PackageSharingPolicyInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageSharingPolicyInfo {
    /// Represents a policy for the package sharing.
    pub fn new() -> PackageSharingPolicyInfo {
        PackageSharingPolicyInfo {
            shared_package_name: None,
            package_sharing_scope: None,
        }
    }

    pub fn set_shared_package_name(&mut self, shared_package_name: String) {
        self.shared_package_name = Some(shared_package_name);
    }

    pub fn with_shared_package_name(
        mut self,
        shared_package_name: String,
    ) -> PackageSharingPolicyInfo {
        self.shared_package_name = Some(shared_package_name);
        self
    }

    pub fn shared_package_name(&self) -> Option<&String> {
        self.shared_package_name.as_ref()
    }

    pub fn reset_shared_package_name(&mut self) {
        self.shared_package_name = None;
    }

    pub fn set_package_sharing_scope(
        &mut self,
        package_sharing_scope: ::models::PackageSharingPolicyScope,
    ) {
        self.package_sharing_scope = Some(package_sharing_scope);
    }

    pub fn with_package_sharing_scope(
        mut self,
        package_sharing_scope: ::models::PackageSharingPolicyScope,
    ) -> PackageSharingPolicyInfo {
        self.package_sharing_scope = Some(package_sharing_scope);
        self
    }

    pub fn package_sharing_scope(
        &self,
    ) -> Option<&::models::PackageSharingPolicyScope> {
        self.package_sharing_scope.as_ref()
    }

    pub fn reset_package_sharing_scope(&mut self) {
        self.package_sharing_scope = None;
    }
}
