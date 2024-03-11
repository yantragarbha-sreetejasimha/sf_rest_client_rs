/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoScalingPolicy : Describes the auto scaling policy

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoScalingPolicy {
    /// The name of the auto scaling policy.
    #[serde(rename = "name")]
    name: String,
    /// Determines when auto scaling operation will be invoked.
    #[serde(rename = "trigger")]
    trigger: ::models::AutoScalingTrigger,
    /// The mechanism that is used to scale when auto scaling operation is invoked.
    #[serde(rename = "mechanism")]
    mechanism: ::models::AutoScalingMechanism,
}

impl AutoScalingPolicy {
    /// Describes the auto scaling policy
    pub fn new(
        name: String,
        trigger: ::models::AutoScalingTrigger,
        mechanism: ::models::AutoScalingMechanism,
    ) -> AutoScalingPolicy {
        AutoScalingPolicy {
            name,
            trigger,
            mechanism,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> AutoScalingPolicy {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_trigger(&mut self, trigger: ::models::AutoScalingTrigger) {
        self.trigger = trigger;
    }

    pub fn with_trigger(
        mut self,
        trigger: ::models::AutoScalingTrigger,
    ) -> AutoScalingPolicy {
        self.trigger = trigger;
        self
    }

    pub fn trigger(&self) -> &::models::AutoScalingTrigger {
        &self.trigger
    }

    pub fn set_mechanism(&mut self, mechanism: ::models::AutoScalingMechanism) {
        self.mechanism = mechanism;
    }

    pub fn with_mechanism(
        mut self,
        mechanism: ::models::AutoScalingMechanism,
    ) -> AutoScalingPolicy {
        self.mechanism = mechanism;
        self
    }

    pub fn mechanism(&self) -> &::models::AutoScalingMechanism {
        &self.mechanism
    }
}