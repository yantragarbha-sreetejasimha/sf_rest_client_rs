/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResumeApplicationUpgradeDescription : Describes the parameters for resuming an unmonitored manual Service Fabric application upgrade

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResumeApplicationUpgradeDescription {
    /// The name of the upgrade domain in which to resume the upgrade.
    #[serde(rename = "UpgradeDomainName")]
    upgrade_domain_name: String,
}

impl ResumeApplicationUpgradeDescription {
    /// Describes the parameters for resuming an unmonitored manual Service Fabric application upgrade
    pub fn new(
        upgrade_domain_name: String,
    ) -> ResumeApplicationUpgradeDescription {
        ResumeApplicationUpgradeDescription {
            upgrade_domain_name,
        }
    }

    pub fn set_upgrade_domain_name(&mut self, upgrade_domain_name: String) {
        self.upgrade_domain_name = upgrade_domain_name;
    }

    pub fn with_upgrade_domain_name(
        mut self,
        upgrade_domain_name: String,
    ) -> ResumeApplicationUpgradeDescription {
        self.upgrade_domain_name = upgrade_domain_name;
        self
    }

    pub fn upgrade_domain_name(&self) -> &String {
        &self.upgrade_domain_name
    }
}
