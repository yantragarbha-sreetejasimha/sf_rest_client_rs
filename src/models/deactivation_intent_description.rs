/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeactivationIntentDescription : Describes the intent or reason for deactivating the node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeactivationIntentDescription {
    /// Describes the intent or reason for deactivating the node. The possible values are following.
    #[serde(rename = "DeactivationIntent")]
    deactivation_intent: Option<String>,
}

impl Default for DeactivationIntentDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl DeactivationIntentDescription {
    /// Describes the intent or reason for deactivating the node.
    pub fn new() -> DeactivationIntentDescription {
        DeactivationIntentDescription {
            deactivation_intent: None,
        }
    }

    pub fn set_deactivation_intent(&mut self, deactivation_intent: String) {
        self.deactivation_intent = Some(deactivation_intent);
    }

    pub fn with_deactivation_intent(
        mut self,
        deactivation_intent: String,
    ) -> DeactivationIntentDescription {
        self.deactivation_intent = Some(deactivation_intent);
        self
    }

    pub fn deactivation_intent(&self) -> Option<&String> {
        self.deactivation_intent.as_ref()
    }

    pub fn reset_deactivation_intent(&mut self) {
        self.deactivation_intent = None;
    }
}
