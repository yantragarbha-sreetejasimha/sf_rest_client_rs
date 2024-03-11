/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CurrentUpgradeDomainProgressInfo : Information about the current in-progress upgrade domain.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUpgradeDomainProgressInfo {
    /// The name of the upgrade domain
    #[serde(rename = "DomainName")]
    domain_name: Option<::models::UpgradeDomainName>,
    /// List of upgrading nodes and their statuses
    #[serde(rename = "NodeUpgradeProgressList")]
    node_upgrade_progress_list: Option<::models::NodeUpgradeProgressInfoList>,
}

impl Default for CurrentUpgradeDomainProgressInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentUpgradeDomainProgressInfo {
    /// Information about the current in-progress upgrade domain.
    pub fn new() -> CurrentUpgradeDomainProgressInfo {
        CurrentUpgradeDomainProgressInfo {
            domain_name: None,
            node_upgrade_progress_list: None,
        }
    }

    pub fn set_domain_name(
        &mut self,
        domain_name: ::models::UpgradeDomainName,
    ) {
        self.domain_name = Some(domain_name);
    }

    pub fn with_domain_name(
        mut self,
        domain_name: ::models::UpgradeDomainName,
    ) -> CurrentUpgradeDomainProgressInfo {
        self.domain_name = Some(domain_name);
        self
    }

    pub fn domain_name(&self) -> Option<&::models::UpgradeDomainName> {
        self.domain_name.as_ref()
    }

    pub fn reset_domain_name(&mut self) {
        self.domain_name = None;
    }

    pub fn set_node_upgrade_progress_list(
        &mut self,
        node_upgrade_progress_list: ::models::NodeUpgradeProgressInfoList,
    ) {
        self.node_upgrade_progress_list = Some(node_upgrade_progress_list);
    }

    pub fn with_node_upgrade_progress_list(
        mut self,
        node_upgrade_progress_list: ::models::NodeUpgradeProgressInfoList,
    ) -> CurrentUpgradeDomainProgressInfo {
        self.node_upgrade_progress_list = Some(node_upgrade_progress_list);
        self
    }

    pub fn node_upgrade_progress_list(
        &self,
    ) -> Option<&::models::NodeUpgradeProgressInfoList> {
        self.node_upgrade_progress_list.as_ref()
    }

    pub fn reset_node_upgrade_progress_list(&mut self) {
        self.node_upgrade_progress_list = None;
    }
}
