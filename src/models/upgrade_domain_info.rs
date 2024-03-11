/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpgradeDomainInfo : Information about an upgrade domain.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeDomainInfo {
    /// The name of the upgrade domain
    #[serde(rename = "Name")]
    name: Option<::models::UpgradeDomainName>,
    /// The state of the upgrade domain.
    #[serde(rename = "State")]
    state: Option<::models::UpgradeDomainState>,
}

impl Default for UpgradeDomainInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeDomainInfo {
    /// Information about an upgrade domain.
    pub fn new() -> UpgradeDomainInfo {
        UpgradeDomainInfo {
            name: None,
            state: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::UpgradeDomainName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::UpgradeDomainName,
    ) -> UpgradeDomainInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::UpgradeDomainName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_state(&mut self, state: ::models::UpgradeDomainState) {
        self.state = Some(state);
    }

    pub fn with_state(
        mut self,
        state: ::models::UpgradeDomainState,
    ) -> UpgradeDomainInfo {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&::models::UpgradeDomainState> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }
}
