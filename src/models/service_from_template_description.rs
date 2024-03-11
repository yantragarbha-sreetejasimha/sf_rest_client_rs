/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceFromTemplateDescription : Defines description for creating a Service Fabric service from a template defined in the application manifest.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceFromTemplateDescription {
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "ApplicationName")]
    application_name: ::models::ApplicationName,
    /// The full name of the service with 'fabric:' URI scheme.
    #[serde(rename = "ServiceName")]
    service_name: ::models::ServiceName,
    /// Name of the service type as specified in the service manifest.
    #[serde(rename = "ServiceTypeName")]
    service_type_name: ::models::ServiceTypeName,
    /// The initialization data for the newly created service instance.
    #[serde(rename = "InitializationData")]
    initialization_data: Option<String>,
    /// The activation mode of service package to be used for a service.
    #[serde(rename = "ServicePackageActivationMode")]
    service_package_activation_mode:
        Option<::models::ServicePackageActivationMode>,
    /// The DNS name of the service. It requires the DNS system service to be enabled in Service Fabric cluster.
    #[serde(rename = "ServiceDnsName")]
    service_dns_name: Option<String>,
}

impl ServiceFromTemplateDescription {
    /// Defines description for creating a Service Fabric service from a template defined in the application manifest.
    pub fn new(
        application_name: ::models::ApplicationName,
        service_name: ::models::ServiceName,
        service_type_name: ::models::ServiceTypeName,
    ) -> ServiceFromTemplateDescription {
        ServiceFromTemplateDescription {
            application_name,
            service_name,
            service_type_name,
            initialization_data: None,
            service_package_activation_mode: None,
            service_dns_name: None,
        }
    }

    pub fn set_application_name(
        &mut self,
        application_name: ::models::ApplicationName,
    ) {
        self.application_name = application_name;
    }

    pub fn with_application_name(
        mut self,
        application_name: ::models::ApplicationName,
    ) -> ServiceFromTemplateDescription {
        self.application_name = application_name;
        self
    }

    pub fn application_name(&self) -> &::models::ApplicationName {
        &self.application_name
    }

    pub fn set_service_name(&mut self, service_name: ::models::ServiceName) {
        self.service_name = service_name;
    }

    pub fn with_service_name(
        mut self,
        service_name: ::models::ServiceName,
    ) -> ServiceFromTemplateDescription {
        self.service_name = service_name;
        self
    }

    pub fn service_name(&self) -> &::models::ServiceName {
        &self.service_name
    }

    pub fn set_service_type_name(
        &mut self,
        service_type_name: ::models::ServiceTypeName,
    ) {
        self.service_type_name = service_type_name;
    }

    pub fn with_service_type_name(
        mut self,
        service_type_name: ::models::ServiceTypeName,
    ) -> ServiceFromTemplateDescription {
        self.service_type_name = service_type_name;
        self
    }

    pub fn service_type_name(&self) -> &::models::ServiceTypeName {
        &self.service_type_name
    }

    pub fn set_initialization_data(&mut self, initialization_data: String) {
        self.initialization_data = Some(initialization_data);
    }

    pub fn with_initialization_data(
        mut self,
        initialization_data: String,
    ) -> ServiceFromTemplateDescription {
        self.initialization_data = Some(initialization_data);
        self
    }

    pub fn initialization_data(&self) -> Option<&String> {
        self.initialization_data.as_ref()
    }

    pub fn reset_initialization_data(&mut self) {
        self.initialization_data = None;
    }

    pub fn set_service_package_activation_mode(
        &mut self,
        service_package_activation_mode: ::models::ServicePackageActivationMode,
    ) {
        self.service_package_activation_mode =
            Some(service_package_activation_mode);
    }

    pub fn with_service_package_activation_mode(
        mut self,
        service_package_activation_mode: ::models::ServicePackageActivationMode,
    ) -> ServiceFromTemplateDescription {
        self.service_package_activation_mode =
            Some(service_package_activation_mode);
        self
    }

    pub fn service_package_activation_mode(
        &self,
    ) -> Option<&::models::ServicePackageActivationMode> {
        self.service_package_activation_mode.as_ref()
    }

    pub fn reset_service_package_activation_mode(&mut self) {
        self.service_package_activation_mode = None;
    }

    pub fn set_service_dns_name(&mut self, service_dns_name: String) {
        self.service_dns_name = Some(service_dns_name);
    }

    pub fn with_service_dns_name(
        mut self,
        service_dns_name: String,
    ) -> ServiceFromTemplateDescription {
        self.service_dns_name = Some(service_dns_name);
        self
    }

    pub fn service_dns_name(&self) -> Option<&String> {
        self.service_dns_name.as_ref()
    }

    pub fn reset_service_dns_name(&mut self) {
        self.service_dns_name = None;
    }
}
