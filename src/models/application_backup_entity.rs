/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ApplicationBackupEntity : Identifies the Service Fabric application which is being backed up.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationBackupEntity {
    /// The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled.
    #[serde(rename = "EntityKind")]
    entity_kind: ::models::BackupEntityKind,
    /// The name of the application, including the 'fabric:' URI scheme.
    #[serde(rename = "ApplicationName")]
    application_name: Option<::models::ApplicationName>,
}

impl ApplicationBackupEntity {
    /// Identifies the Service Fabric application which is being backed up.
    pub fn new(
        entity_kind: ::models::BackupEntityKind,
    ) -> ApplicationBackupEntity {
        ApplicationBackupEntity {
            entity_kind,
            application_name: None,
        }
    }

    pub fn set_entity_kind(&mut self, entity_kind: ::models::BackupEntityKind) {
        self.entity_kind = entity_kind;
    }

    pub fn with_entity_kind(
        mut self,
        entity_kind: ::models::BackupEntityKind,
    ) -> ApplicationBackupEntity {
        self.entity_kind = entity_kind;
        self
    }

    pub fn entity_kind(&self) -> &::models::BackupEntityKind {
        &self.entity_kind
    }

    pub fn set_application_name(
        &mut self,
        application_name: ::models::ApplicationName,
    ) {
        self.application_name = Some(application_name);
    }

    pub fn with_application_name(
        mut self,
        application_name: ::models::ApplicationName,
    ) -> ApplicationBackupEntity {
        self.application_name = Some(application_name);
        self
    }

    pub fn application_name(&self) -> Option<&::models::ApplicationName> {
        self.application_name.as_ref()
    }

    pub fn reset_application_name(&mut self) {
        self.application_name = None;
    }
}
