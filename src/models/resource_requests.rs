/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResourceRequests : This type describes the requested resources for a given container. It describes the least amount of resources required for the container. A container can consume more than requested resources up to the specified limits before being restarted. Currently, the requested resources are treated as limits.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceRequests {
    /// The memory request in GB for this container.
    #[serde(rename = "memoryInGB")]
    memory_in_gb: f64,
    /// Requested number of CPU cores. At present, only full cores are supported.
    #[serde(rename = "cpu")]
    cpu: f64,
}

impl ResourceRequests {
    /// This type describes the requested resources for a given container. It describes the least amount of resources required for the container. A container can consume more than requested resources up to the specified limits before being restarted. Currently, the requested resources are treated as limits.
    pub fn new(memory_in_gb: f64, cpu: f64) -> ResourceRequests {
        ResourceRequests { memory_in_gb, cpu }
    }

    pub fn set_memory_in_gb(&mut self, memory_in_gb: f64) {
        self.memory_in_gb = memory_in_gb;
    }

    pub fn with_memory_in_gb(mut self, memory_in_gb: f64) -> ResourceRequests {
        self.memory_in_gb = memory_in_gb;
        self
    }

    pub fn memory_in_gb(&self) -> &f64 {
        &self.memory_in_gb
    }

    pub fn set_cpu(&mut self, cpu: f64) {
        self.cpu = cpu;
    }

    pub fn with_cpu(mut self, cpu: f64) -> ResourceRequests {
        self.cpu = cpu;
        self
    }

    pub fn cpu(&self) -> &f64 {
        &self.cpu
    }
}
