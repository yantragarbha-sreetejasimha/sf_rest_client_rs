/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ManagedApplicationIdentity : Describes a managed application identity.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedApplicationIdentity {
    /// The name of the identity.
    #[serde(rename = "Name")]
    name: String,
    /// The identity's PrincipalId.
    #[serde(rename = "PrincipalId")]
    principal_id: Option<String>,
}

impl ManagedApplicationIdentity {
    /// Describes a managed application identity.
    pub fn new(name: String) -> ManagedApplicationIdentity {
        ManagedApplicationIdentity {
            name,
            principal_id: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> ManagedApplicationIdentity {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_principal_id(&mut self, principal_id: String) {
        self.principal_id = Some(principal_id);
    }

    pub fn with_principal_id(
        mut self,
        principal_id: String,
    ) -> ManagedApplicationIdentity {
        self.principal_id = Some(principal_id);
        self
    }

    pub fn principal_id(&self) -> Option<&String> {
        self.principal_id.as_ref()
    }

    pub fn reset_principal_id(&mut self) {
        self.principal_id = None;
    }
}
