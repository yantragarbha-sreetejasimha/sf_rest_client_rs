/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResourceLimits : This type describes the resource limits for a given container. It describes the most amount of resources a container is allowed to use before being restarted.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// The memory limit in GB.
    #[serde(rename = "memoryInGB")]
    memory_in_gb: Option<f64>,
    /// CPU limits in cores. At present, only full cores are supported.
    #[serde(rename = "cpu")]
    cpu: Option<f64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceLimits {
    /// This type describes the resource limits for a given container. It describes the most amount of resources a container is allowed to use before being restarted.
    pub fn new() -> ResourceLimits {
        ResourceLimits {
            memory_in_gb: None,
            cpu: None,
        }
    }

    pub fn set_memory_in_gb(&mut self, memory_in_gb: f64) {
        self.memory_in_gb = Some(memory_in_gb);
    }

    pub fn with_memory_in_gb(mut self, memory_in_gb: f64) -> ResourceLimits {
        self.memory_in_gb = Some(memory_in_gb);
        self
    }

    pub fn memory_in_gb(&self) -> Option<&f64> {
        self.memory_in_gb.as_ref()
    }

    pub fn reset_memory_in_gb(&mut self) {
        self.memory_in_gb = None;
    }

    pub fn set_cpu(&mut self, cpu: f64) {
        self.cpu = Some(cpu);
    }

    pub fn with_cpu(mut self, cpu: f64) -> ResourceLimits {
        self.cpu = Some(cpu);
        self
    }

    pub fn cpu(&self) -> Option<&f64> {
        self.cpu.as_ref()
    }

    pub fn reset_cpu(&mut self) {
        self.cpu = None;
    }
}
