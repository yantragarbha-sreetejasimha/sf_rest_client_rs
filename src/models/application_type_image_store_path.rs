/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationTypeImageStorePath : Path description for the application package in the image store specified during the prior copy operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTypeImageStorePath {
    /// The relative image store path to the application package.
    #[serde(rename = "ApplicationTypeBuildPath")]
    application_type_build_path: String,
}

impl ApplicationTypeImageStorePath {
    /// Path description for the application package in the image store specified during the prior copy operation.
    pub fn new(
        application_type_build_path: String,
    ) -> ApplicationTypeImageStorePath {
        ApplicationTypeImageStorePath {
            application_type_build_path,
        }
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
    ) -> ApplicationTypeImageStorePath {
        self.application_type_build_path = application_type_build_path;
        self
    }

    pub fn application_type_build_path(&self) -> &String {
        &self.application_type_build_path
    }
}
