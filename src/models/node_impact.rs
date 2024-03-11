/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodeImpact : Describes the expected impact of a repair to a particular node.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeImpact {
    /// The name of the impacted node.
    #[serde(rename = "NodeName")]
    node_name: String,
    /// The level of impact expected.
    #[serde(rename = "ImpactLevel")]
    impact_level: Option<String>,
}

impl NodeImpact {
    /// Describes the expected impact of a repair to a particular node.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.
    pub fn new(node_name: String) -> NodeImpact {
        NodeImpact {
            node_name,
            impact_level: None,
        }
    }

    pub fn set_node_name(&mut self, node_name: String) {
        self.node_name = node_name;
    }

    pub fn with_node_name(mut self, node_name: String) -> NodeImpact {
        self.node_name = node_name;
        self
    }

    pub fn node_name(&self) -> &String {
        &self.node_name
    }

    pub fn set_impact_level(&mut self, impact_level: String) {
        self.impact_level = Some(impact_level);
    }

    pub fn with_impact_level(mut self, impact_level: String) -> NodeImpact {
        self.impact_level = Some(impact_level);
        self
    }

    pub fn impact_level(&self) -> Option<&String> {
        self.impact_level.as_ref()
    }

    pub fn reset_impact_level(&mut self) {
        self.impact_level = None;
    }
}
