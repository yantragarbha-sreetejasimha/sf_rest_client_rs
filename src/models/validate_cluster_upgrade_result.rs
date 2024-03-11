/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ValidateClusterUpgradeResult : Specifies result of validating a cluster upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateClusterUpgradeResult {
    /// The expected impact of the upgrade.
    #[serde(rename = "ServiceHostUpgradeImpact")]
    service_host_upgrade_impact: Option<::models::ServiceHostUpgradeImpact>,
    /// A string containing additional details for the Fabric upgrade validation result.
    #[serde(rename = "ValidationDetails")]
    validation_details: Option<String>,
}

impl Default for ValidateClusterUpgradeResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidateClusterUpgradeResult {
    /// Specifies result of validating a cluster upgrade.
    pub fn new() -> ValidateClusterUpgradeResult {
        ValidateClusterUpgradeResult {
            service_host_upgrade_impact: None,
            validation_details: None,
        }
    }

    pub fn set_service_host_upgrade_impact(
        &mut self,
        service_host_upgrade_impact: ::models::ServiceHostUpgradeImpact,
    ) {
        self.service_host_upgrade_impact = Some(service_host_upgrade_impact);
    }

    pub fn with_service_host_upgrade_impact(
        mut self,
        service_host_upgrade_impact: ::models::ServiceHostUpgradeImpact,
    ) -> ValidateClusterUpgradeResult {
        self.service_host_upgrade_impact = Some(service_host_upgrade_impact);
        self
    }

    pub fn service_host_upgrade_impact(
        &self,
    ) -> Option<&::models::ServiceHostUpgradeImpact> {
        self.service_host_upgrade_impact.as_ref()
    }

    pub fn reset_service_host_upgrade_impact(&mut self) {
        self.service_host_upgrade_impact = None;
    }

    pub fn set_validation_details(&mut self, validation_details: String) {
        self.validation_details = Some(validation_details);
    }

    pub fn with_validation_details(
        mut self,
        validation_details: String,
    ) -> ValidateClusterUpgradeResult {
        self.validation_details = Some(validation_details);
        self
    }

    pub fn validation_details(&self) -> Option<&String> {
        self.validation_details.as_ref()
    }

    pub fn reset_validation_details(&mut self) {
        self.validation_details = None;
    }
}