/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RepairTaskUpdateInfo : Describes the result of an operation that created or updated a repair task.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairTaskUpdateInfo {
    /// The new version of the repair task.
    #[serde(rename = "Version")]
    version: String,
}

impl RepairTaskUpdateInfo {
    /// Describes the result of an operation that created or updated a repair task.  This type supports the Service Fabric platform; it is not meant to be used directly from your code.
    pub fn new(version: String) -> RepairTaskUpdateInfo {
        RepairTaskUpdateInfo { version }
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn with_version(mut self, version: String) -> RepairTaskUpdateInfo {
        self.version = version;
        self
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}