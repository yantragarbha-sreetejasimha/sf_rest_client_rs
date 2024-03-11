/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BackupEntity : Describes the Service Fabric entity that is configured for backup.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupEntity {
    /// The entity type of a Service Fabric entity such as Application, Service or a Partition where periodic backups can be enabled.
    #[serde(rename = "EntityKind")]
    entity_kind: ::models::BackupEntityKind,
}

impl BackupEntity {
    /// Describes the Service Fabric entity that is configured for backup.
    pub fn new(entity_kind: ::models::BackupEntityKind) -> BackupEntity {
        BackupEntity { entity_kind }
    }

    pub fn set_entity_kind(&mut self, entity_kind: ::models::BackupEntityKind) {
        self.entity_kind = entity_kind;
    }

    pub fn with_entity_kind(
        mut self,
        entity_kind: ::models::BackupEntityKind,
    ) -> BackupEntity {
        self.entity_kind = entity_kind;
        self
    }

    pub fn entity_kind(&self) -> &::models::BackupEntityKind {
        &self.entity_kind
    }
}
