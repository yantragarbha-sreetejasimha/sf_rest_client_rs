/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScalingPolicyDescription : Describes how the scaling should be performed

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalingPolicyDescription {
    /// Specifies the trigger associated with this scaling policy
    #[serde(rename = "ScalingTrigger")]
    scaling_trigger: ::models::ScalingTriggerDescription,
    /// Specifies the mechanism associated with this scaling policy
    #[serde(rename = "ScalingMechanism")]
    scaling_mechanism: ::models::ScalingMechanismDescription,
}

impl ScalingPolicyDescription {
    /// Describes how the scaling should be performed
    pub fn new(
        scaling_trigger: ::models::ScalingTriggerDescription,
        scaling_mechanism: ::models::ScalingMechanismDescription,
    ) -> ScalingPolicyDescription {
        ScalingPolicyDescription {
            scaling_trigger,
            scaling_mechanism,
        }
    }

    pub fn set_scaling_trigger(
        &mut self,
        scaling_trigger: ::models::ScalingTriggerDescription,
    ) {
        self.scaling_trigger = scaling_trigger;
    }

    pub fn with_scaling_trigger(
        mut self,
        scaling_trigger: ::models::ScalingTriggerDescription,
    ) -> ScalingPolicyDescription {
        self.scaling_trigger = scaling_trigger;
        self
    }

    pub fn scaling_trigger(&self) -> &::models::ScalingTriggerDescription {
        &self.scaling_trigger
    }

    pub fn set_scaling_mechanism(
        &mut self,
        scaling_mechanism: ::models::ScalingMechanismDescription,
    ) {
        self.scaling_mechanism = scaling_mechanism;
    }

    pub fn with_scaling_mechanism(
        mut self,
        scaling_mechanism: ::models::ScalingMechanismDescription,
    ) -> ScalingPolicyDescription {
        self.scaling_mechanism = scaling_mechanism;
        self
    }

    pub fn scaling_mechanism(&self) -> &::models::ScalingMechanismDescription {
        &self.scaling_mechanism
    }
}
