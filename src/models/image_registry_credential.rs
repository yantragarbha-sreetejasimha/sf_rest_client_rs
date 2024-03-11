/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ImageRegistryCredential : Image registry credential.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    /// Docker image registry server, without protocol such as `http` and `https`.
    #[serde(rename = "server")]
    server: String,
    /// The username for the private registry.
    #[serde(rename = "username")]
    username: String,
    /// The type of the image registry password being given in password
    #[serde(rename = "passwordType")]
    password_type: Option<::models::ImageRegistryPasswordType>,
    /// The password for the private registry. The password is required for create or update operations, however it is not returned in the get or list operations. Will be processed based on the type provided.
    #[serde(rename = "password")]
    password: Option<String>,
}

impl ImageRegistryCredential {
    /// Image registry credential.
    pub fn new(server: String, username: String) -> ImageRegistryCredential {
        ImageRegistryCredential {
            server,
            username,
            password_type: None,
            password: None,
        }
    }

    pub fn set_server(&mut self, server: String) {
        self.server = server;
    }

    pub fn with_server(mut self, server: String) -> ImageRegistryCredential {
        self.server = server;
        self
    }

    pub fn server(&self) -> &String {
        &self.server
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn with_username(
        mut self,
        username: String,
    ) -> ImageRegistryCredential {
        self.username = username;
        self
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn set_password_type(
        &mut self,
        password_type: ::models::ImageRegistryPasswordType,
    ) {
        self.password_type = Some(password_type);
    }

    pub fn with_password_type(
        mut self,
        password_type: ::models::ImageRegistryPasswordType,
    ) -> ImageRegistryCredential {
        self.password_type = Some(password_type);
        self
    }

    pub fn password_type(
        &self,
    ) -> Option<&::models::ImageRegistryPasswordType> {
        self.password_type.as_ref()
    }

    pub fn reset_password_type(&mut self) {
        self.password_type = None;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = Some(password);
    }

    pub fn with_password(
        mut self,
        password: String,
    ) -> ImageRegistryCredential {
        self.password = Some(password);
        self
    }

    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn reset_password(&mut self) {
        self.password = None;
    }
}
