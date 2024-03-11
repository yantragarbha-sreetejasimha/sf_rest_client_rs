/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationDescription : Describes a Service Fabric application.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationDescription {
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: ::models::ApplicationName,
    /// The application type name as defined in the application manifest.
    #[serde(rename = "TypeName")]
    type_name: ::models::ApplicationTypeName,
    /// The version of the application type as defined in the application manifest.
    #[serde(rename = "TypeVersion")]
    type_version: ::models::ApplicationTypeVersion,
    /// List of application parameters with overridden values from their default values specified in the application manifest.
    #[serde(rename = "ParameterList")]
    parameter_list: Option<::models::ApplicationParameterList>,
    /// Describes capacity information for services of this application. This description can be used for describing the following. - Reserving the capacity for the services on the nodes - Limiting the total number of nodes that services of this application can run on - Limiting the custom capacity metrics to limit the total consumption of this metric by the services of this application
    #[serde(rename = "ApplicationCapacity")]
    application_capacity: Option<::models::ApplicationCapacityDescription>,
    /// Managed application identity description.
    #[serde(rename = "ManagedApplicationIdentity")]
    managed_application_identity:
        Option<::models::ManagedApplicationIdentityDescription>,
}

impl ApplicationDescription {
    /// Describes a Service Fabric application.
    pub fn new(
        name: ::models::ApplicationName,
        type_name: ::models::ApplicationTypeName,
        type_version: ::models::ApplicationTypeVersion,
    ) -> ApplicationDescription {
        ApplicationDescription {
            name,
            type_name,
            type_version,
            parameter_list: None,
            application_capacity: None,
            managed_application_identity: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::ApplicationName) {
        self.name = name;
    }

    pub fn with_name(
        mut self,
        name: ::models::ApplicationName,
    ) -> ApplicationDescription {
        self.name = name;
        self
    }

    pub fn name(&self) -> &::models::ApplicationName {
        &self.name
    }

    pub fn set_type_name(&mut self, type_name: ::models::ApplicationTypeName) {
        self.type_name = type_name;
    }

    pub fn with_type_name(
        mut self,
        type_name: ::models::ApplicationTypeName,
    ) -> ApplicationDescription {
        self.type_name = type_name;
        self
    }

    pub fn type_name(&self) -> &::models::ApplicationTypeName {
        &self.type_name
    }

    pub fn set_type_version(
        &mut self,
        type_version: ::models::ApplicationTypeVersion,
    ) {
        self.type_version = type_version;
    }

    pub fn with_type_version(
        mut self,
        type_version: ::models::ApplicationTypeVersion,
    ) -> ApplicationDescription {
        self.type_version = type_version;
        self
    }

    pub fn type_version(&self) -> &::models::ApplicationTypeVersion {
        &self.type_version
    }

    pub fn set_parameter_list(
        &mut self,
        parameter_list: ::models::ApplicationParameterList,
    ) {
        self.parameter_list = Some(parameter_list);
    }

    pub fn with_parameter_list(
        mut self,
        parameter_list: ::models::ApplicationParameterList,
    ) -> ApplicationDescription {
        self.parameter_list = Some(parameter_list);
        self
    }

    pub fn parameter_list(
        &self,
    ) -> Option<&::models::ApplicationParameterList> {
        self.parameter_list.as_ref()
    }

    pub fn reset_parameter_list(&mut self) {
        self.parameter_list = None;
    }

    pub fn set_application_capacity(
        &mut self,
        application_capacity: ::models::ApplicationCapacityDescription,
    ) {
        self.application_capacity = Some(application_capacity);
    }

    pub fn with_application_capacity(
        mut self,
        application_capacity: ::models::ApplicationCapacityDescription,
    ) -> ApplicationDescription {
        self.application_capacity = Some(application_capacity);
        self
    }

    pub fn application_capacity(
        &self,
    ) -> Option<&::models::ApplicationCapacityDescription> {
        self.application_capacity.as_ref()
    }

    pub fn reset_application_capacity(&mut self) {
        self.application_capacity = None;
    }

    pub fn set_managed_application_identity(
        &mut self,
        managed_application_identity: ::models::ManagedApplicationIdentityDescription,
    ) {
        self.managed_application_identity = Some(managed_application_identity);
    }

    pub fn with_managed_application_identity(
        mut self,
        managed_application_identity: ::models::ManagedApplicationIdentityDescription,
    ) -> ApplicationDescription {
        self.managed_application_identity = Some(managed_application_identity);
        self
    }

    pub fn managed_application_identity(
        &self,
    ) -> Option<&::models::ManagedApplicationIdentityDescription> {
        self.managed_application_identity.as_ref()
    }

    pub fn reset_managed_application_identity(&mut self) {
        self.managed_application_identity = None;
    }
}
