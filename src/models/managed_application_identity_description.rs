/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ManagedApplicationIdentityDescription : Managed application identity description.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedApplicationIdentityDescription {
    /// Token service endpoint.
    #[serde(rename = "TokenServiceEndpoint")]
    token_service_endpoint: Option<String>,
    /// A list of managed application identity objects.
    #[serde(rename = "ManagedIdentities")]
    managed_identities: Option<::models::ManagedApplicationIdentityList>,
}

impl Default for ManagedApplicationIdentityDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ManagedApplicationIdentityDescription {
    /// Managed application identity description.
    pub fn new() -> ManagedApplicationIdentityDescription {
        ManagedApplicationIdentityDescription {
            token_service_endpoint: None,
            managed_identities: None,
        }
    }

    pub fn set_token_service_endpoint(
        &mut self,
        token_service_endpoint: String,
    ) {
        self.token_service_endpoint = Some(token_service_endpoint);
    }

    pub fn with_token_service_endpoint(
        mut self,
        token_service_endpoint: String,
    ) -> ManagedApplicationIdentityDescription {
        self.token_service_endpoint = Some(token_service_endpoint);
        self
    }

    pub fn token_service_endpoint(&self) -> Option<&String> {
        self.token_service_endpoint.as_ref()
    }

    pub fn reset_token_service_endpoint(&mut self) {
        self.token_service_endpoint = None;
    }

    pub fn set_managed_identities(
        &mut self,
        managed_identities: ::models::ManagedApplicationIdentityList,
    ) {
        self.managed_identities = Some(managed_identities);
    }

    pub fn with_managed_identities(
        mut self,
        managed_identities: ::models::ManagedApplicationIdentityList,
    ) -> ManagedApplicationIdentityDescription {
        self.managed_identities = Some(managed_identities);
        self
    }

    pub fn managed_identities(
        &self,
    ) -> Option<&::models::ManagedApplicationIdentityList> {
        self.managed_identities.as_ref()
    }

    pub fn reset_managed_identities(&mut self) {
        self.managed_identities = None;
    }
}
