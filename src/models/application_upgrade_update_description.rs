/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationUpgradeUpdateDescription : Describes the parameters for updating an ongoing application upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationUpgradeUpdateDescription {
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: ::models::ApplicationName,
    /// The kind of upgrade out of the following possible values.
    #[serde(rename = "UpgradeKind")]
    upgrade_kind: ::models::UpgradeKind,
    /// Defines a health policy used to evaluate the health of an application or one of its children entities.
    #[serde(rename = "ApplicationHealthPolicy")]
    application_health_policy: Option<::models::ApplicationHealthPolicy>,
    /// Describes the parameters for updating a rolling upgrade of application or cluster.
    #[serde(rename = "UpdateDescription")]
    update_description: Option<::models::RollingUpgradeUpdateDescription>,
}

impl ApplicationUpgradeUpdateDescription {
    /// Describes the parameters for updating an ongoing application upgrade.
    pub fn new(
        name: ::models::ApplicationName,
        upgrade_kind: ::models::UpgradeKind,
    ) -> ApplicationUpgradeUpdateDescription {
        ApplicationUpgradeUpdateDescription {
            name,
            upgrade_kind,
            application_health_policy: None,
            update_description: None,
        }
    }

    pub fn set_name(&mut self, name: ::models::ApplicationName) {
        self.name = name;
    }

    pub fn with_name(
        mut self,
        name: ::models::ApplicationName,
    ) -> ApplicationUpgradeUpdateDescription {
        self.name = name;
        self
    }

    pub fn name(&self) -> &::models::ApplicationName {
        &self.name
    }

    pub fn set_upgrade_kind(&mut self, upgrade_kind: ::models::UpgradeKind) {
        self.upgrade_kind = upgrade_kind;
    }

    pub fn with_upgrade_kind(
        mut self,
        upgrade_kind: ::models::UpgradeKind,
    ) -> ApplicationUpgradeUpdateDescription {
        self.upgrade_kind = upgrade_kind;
        self
    }

    pub fn upgrade_kind(&self) -> &::models::UpgradeKind {
        &self.upgrade_kind
    }

    pub fn set_application_health_policy(
        &mut self,
        application_health_policy: ::models::ApplicationHealthPolicy,
    ) {
        self.application_health_policy = Some(application_health_policy);
    }

    pub fn with_application_health_policy(
        mut self,
        application_health_policy: ::models::ApplicationHealthPolicy,
    ) -> ApplicationUpgradeUpdateDescription {
        self.application_health_policy = Some(application_health_policy);
        self
    }

    pub fn application_health_policy(
        &self,
    ) -> Option<&::models::ApplicationHealthPolicy> {
        self.application_health_policy.as_ref()
    }

    pub fn reset_application_health_policy(&mut self) {
        self.application_health_policy = None;
    }

    pub fn set_update_description(
        &mut self,
        update_description: ::models::RollingUpgradeUpdateDescription,
    ) {
        self.update_description = Some(update_description);
    }

    pub fn with_update_description(
        mut self,
        update_description: ::models::RollingUpgradeUpdateDescription,
    ) -> ApplicationUpgradeUpdateDescription {
        self.update_description = Some(update_description);
        self
    }

    pub fn update_description(
        &self,
    ) -> Option<&::models::RollingUpgradeUpdateDescription> {
        self.update_description.as_ref()
    }

    pub fn reset_update_description(&mut self) {
        self.update_description = None;
    }
}
