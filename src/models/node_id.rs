/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeId : An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeId {
    /// Value of the node Id. This is a 128 bit integer.
    #[serde(rename = "Id")]
    id: Option<String>,
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeId {
    /// An internal ID used by Service Fabric to uniquely identify a node. Node Id is deterministically generated from node name.
    pub fn new() -> NodeId {
        NodeId { id: None }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: String) -> NodeId {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }
}
