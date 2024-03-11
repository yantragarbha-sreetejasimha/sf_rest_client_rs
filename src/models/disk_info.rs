/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DiskInfo : Information about the disk

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    /// the disk size in bytes
    #[serde(rename = "Capacity")]
    capacity: Option<String>,
    /// the available disk space in bytes
    #[serde(rename = "AvailableSpace")]
    available_space: Option<String>,
}

impl Default for DiskInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl DiskInfo {
    /// Information about the disk
    pub fn new() -> DiskInfo {
        DiskInfo {
            capacity: None,
            available_space: None,
        }
    }

    pub fn set_capacity(&mut self, capacity: String) {
        self.capacity = Some(capacity);
    }

    pub fn with_capacity(mut self, capacity: String) -> DiskInfo {
        self.capacity = Some(capacity);
        self
    }

    pub fn capacity(&self) -> Option<&String> {
        self.capacity.as_ref()
    }

    pub fn reset_capacity(&mut self) {
        self.capacity = None;
    }

    pub fn set_available_space(&mut self, available_space: String) {
        self.available_space = Some(available_space);
    }

    pub fn with_available_space(mut self, available_space: String) -> DiskInfo {
        self.available_space = Some(available_space);
        self
    }

    pub fn available_space(&self) -> Option<&String> {
        self.available_space.as_ref()
    }

    pub fn reset_available_space(&mut self) {
        self.available_space = None;
    }
}
