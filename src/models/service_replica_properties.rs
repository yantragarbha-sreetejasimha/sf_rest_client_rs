/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceReplicaProperties : Describes the properties of a service replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceReplicaProperties {
    /// The operation system required by the code in service.
    #[serde(rename = "osType")]
    os_type: ::models::OperatingSystemType,
    /// Describes the set of code packages that forms the service. A code package describes the container and the properties for running it. All the code packages are started together on the same host and share the same context (network, process etc.).
    #[serde(rename = "codePackages")]
    code_packages: Vec<::models::ContainerCodePackageProperties>,
    /// The names of the private networks that this service needs to be part of.
    #[serde(rename = "networkRefs")]
    network_refs: Option<Vec<::models::NetworkRef>>,
    /// Reference to sinks in DiagnosticsDescription.
    #[serde(rename = "diagnostics")]
    diagnostics: Option<::models::DiagnosticsRef>,
}

impl ServiceReplicaProperties {
    /// Describes the properties of a service replica.
    pub fn new(
        os_type: ::models::OperatingSystemType,
        code_packages: Vec<::models::ContainerCodePackageProperties>,
    ) -> ServiceReplicaProperties {
        ServiceReplicaProperties {
            os_type,
            code_packages,
            network_refs: None,
            diagnostics: None,
        }
    }

    pub fn set_os_type(&mut self, os_type: ::models::OperatingSystemType) {
        self.os_type = os_type;
    }

    pub fn with_os_type(
        mut self,
        os_type: ::models::OperatingSystemType,
    ) -> ServiceReplicaProperties {
        self.os_type = os_type;
        self
    }

    pub fn os_type(&self) -> &::models::OperatingSystemType {
        &self.os_type
    }

    pub fn set_code_packages(
        &mut self,
        code_packages: Vec<::models::ContainerCodePackageProperties>,
    ) {
        self.code_packages = code_packages;
    }

    pub fn with_code_packages(
        mut self,
        code_packages: Vec<::models::ContainerCodePackageProperties>,
    ) -> ServiceReplicaProperties {
        self.code_packages = code_packages;
        self
    }

    pub fn code_packages(
        &self,
    ) -> &Vec<::models::ContainerCodePackageProperties> {
        &self.code_packages
    }

    pub fn set_network_refs(
        &mut self,
        network_refs: Vec<::models::NetworkRef>,
    ) {
        self.network_refs = Some(network_refs);
    }

    pub fn with_network_refs(
        mut self,
        network_refs: Vec<::models::NetworkRef>,
    ) -> ServiceReplicaProperties {
        self.network_refs = Some(network_refs);
        self
    }

    pub fn network_refs(&self) -> Option<&Vec<::models::NetworkRef>> {
        self.network_refs.as_ref()
    }

    pub fn reset_network_refs(&mut self) {
        self.network_refs = None;
    }

    pub fn set_diagnostics(&mut self, diagnostics: ::models::DiagnosticsRef) {
        self.diagnostics = Some(diagnostics);
    }

    pub fn with_diagnostics(
        mut self,
        diagnostics: ::models::DiagnosticsRef,
    ) -> ServiceReplicaProperties {
        self.diagnostics = Some(diagnostics);
        self
    }

    pub fn diagnostics(&self) -> Option<&::models::DiagnosticsRef> {
        self.diagnostics.as_ref()
    }

    pub fn reset_diagnostics(&mut self) {
        self.diagnostics = None;
    }
}
