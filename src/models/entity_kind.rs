/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EntityKind : The entity type of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityKind {}

impl Default for EntityKind {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityKind {
    /// The entity type of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    pub fn new() -> EntityKind {
        EntityKind {}
    }
}

// TODO enum
// List of EntityKind
//const (
//
//
//
//)
