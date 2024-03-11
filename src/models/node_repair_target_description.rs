/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeRepairTargetDescription : Describes the list of nodes targeted by a repair action.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRepairTargetDescription {
    /// The kind of repair target described by the current object.
    #[serde(rename = "Kind")]
    kind: ::models::RepairTargetKind,
    /// The list of nodes targeted by a repair action.
    #[serde(rename = "NodeNames")]
    node_names: Option<Vec<String>>,
}

impl NodeRepairTargetDescription {
    /// Describes the list of nodes targeted by a repair action.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.
    pub fn new(
        kind: ::models::RepairTargetKind,
    ) -> NodeRepairTargetDescription {
        NodeRepairTargetDescription {
            kind,
            node_names: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::RepairTargetKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::RepairTargetKind,
    ) -> NodeRepairTargetDescription {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::RepairTargetKind {
        &self.kind
    }

    pub fn set_node_names(&mut self, node_names: Vec<String>) {
        self.node_names = Some(node_names);
    }

    pub fn with_node_names(
        mut self,
        node_names: Vec<String>,
    ) -> NodeRepairTargetDescription {
        self.node_names = Some(node_names);
        self
    }

    pub fn node_names(&self) -> Option<&Vec<String>> {
        self.node_names.as_ref()
    }

    pub fn reset_node_names(&mut self) {
        self.node_names = None;
    }
}
