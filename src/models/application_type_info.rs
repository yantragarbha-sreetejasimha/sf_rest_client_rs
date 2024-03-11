/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeInfo : Information about an application type.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeInfo {
    /// The application type name as defined in the application manifest.
    #[serde(rename = "Name")]
    name: Option<::models::ApplicationTypeName>,
    /// The version of the application type as defined in the application manifest.
    #[serde(rename = "Version")]
    version: Option<::models::ApplicationTypeVersion>,
    /// List of application type parameters that can be overridden when creating or updating the application.
    #[serde(rename = "DefaultParameterList")]
    default_parameter_list: Option<::models::ApplicationTypeParameterList>,
    /// The status of the application type.
    #[serde(rename = "Status")]
    status: Option<::models::ApplicationTypeStatus>,
    /// Additional detailed information about the status of the application type.
    #[serde(rename = "StatusDetails")]
    status_details: Option<String>,
    /// The mechanism used to define a Service Fabric application type.
    #[serde(rename = "ApplicationTypeDefinitionKind")]
    application_type_definition_kind:
        Option<::models::ApplicationTypeDefinitionKind>,
}

impl Default for ApplicationTypeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationTypeInfo {
    /// Information about an application type.
    pub fn new() -> ApplicationTypeInfo {
        ApplicationTypeInfo {
            name: None,
            version: None,
            default_parameter_list: None,
            status: None,
            status_details: None,
            application_type_definition_kind: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::ApplicationTypeName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::ApplicationTypeName,
    ) -> ApplicationTypeInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::ApplicationTypeName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_version(&mut self, version: ::models::ApplicationTypeVersion) {
        self.version = Some(version);
    }

    pub fn with_version(
        mut self,
        version: ::models::ApplicationTypeVersion,
    ) -> ApplicationTypeInfo {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<&::models::ApplicationTypeVersion> {
        self.version.as_ref()
    }

    pub fn reset_version(&mut self) {
        self.version = None;
    }

    pub fn set_default_parameter_list(
        &mut self,
        default_parameter_list: ::models::ApplicationTypeParameterList,
    ) {
        self.default_parameter_list = Some(default_parameter_list);
    }

    pub fn with_default_parameter_list(
        mut self,
        default_parameter_list: ::models::ApplicationTypeParameterList,
    ) -> ApplicationTypeInfo {
        self.default_parameter_list = Some(default_parameter_list);
        self
    }

    pub fn default_parameter_list(
        &self,
    ) -> Option<&::models::ApplicationTypeParameterList> {
        self.default_parameter_list.as_ref()
    }

    pub fn reset_default_parameter_list(&mut self) {
        self.default_parameter_list = None;
    }

    pub fn set_status(&mut self, status: ::models::ApplicationTypeStatus) {
        self.status = Some(status);
    }

    pub fn with_status(
        mut self,
        status: ::models::ApplicationTypeStatus,
    ) -> ApplicationTypeInfo {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&::models::ApplicationTypeStatus> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_status_details(&mut self, status_details: String) {
        self.status_details = Some(status_details);
    }

    pub fn with_status_details(
        mut self,
        status_details: String,
    ) -> ApplicationTypeInfo {
        self.status_details = Some(status_details);
        self
    }

    pub fn status_details(&self) -> Option<&String> {
        self.status_details.as_ref()
    }

    pub fn reset_status_details(&mut self) {
        self.status_details = None;
    }

    pub fn set_application_type_definition_kind(
        &mut self,
        application_type_definition_kind: ::models::ApplicationTypeDefinitionKind,
    ) {
        self.application_type_definition_kind =
            Some(application_type_definition_kind);
    }

    pub fn with_application_type_definition_kind(
        mut self,
        application_type_definition_kind: ::models::ApplicationTypeDefinitionKind,
    ) -> ApplicationTypeInfo {
        self.application_type_definition_kind =
            Some(application_type_definition_kind);
        self
    }

    pub fn application_type_definition_kind(
        &self,
    ) -> Option<&::models::ApplicationTypeDefinitionKind> {
        self.application_type_definition_kind.as_ref()
    }

    pub fn reset_application_type_definition_kind(&mut self) {
        self.application_type_definition_kind = None;
    }
}
