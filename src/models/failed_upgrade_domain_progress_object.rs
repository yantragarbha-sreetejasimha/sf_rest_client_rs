/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FailedUpgradeDomainProgressObject : The detailed upgrade progress for nodes in the current upgrade domain at the point of failure. Not applicable to node-by-node upgrades.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedUpgradeDomainProgressObject {
    /// The name of the upgrade domain
    #[serde(rename = "DomainName")]
    domain_name: Option<::models::UpgradeDomainName>,
    /// List of upgrading nodes and their statuses
    #[serde(rename = "NodeUpgradeProgressList")]
    node_upgrade_progress_list: Option<::models::NodeUpgradeProgressInfoList>,
}

impl Default for FailedUpgradeDomainProgressObject {
    fn default() -> Self {
        Self::new()
    }
}

impl FailedUpgradeDomainProgressObject {
    /// The detailed upgrade progress for nodes in the current upgrade domain at the point of failure. Not applicable to node-by-node upgrades.
    pub fn new() -> FailedUpgradeDomainProgressObject {
        FailedUpgradeDomainProgressObject {
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
    ) -> FailedUpgradeDomainProgressObject {
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
    ) -> FailedUpgradeDomainProgressObject {
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
