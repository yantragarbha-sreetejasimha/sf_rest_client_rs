/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UsageInfo : Information about how much space and how many files in the file system the ImageStore is using in this category

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageInfo {
    /// the size of all files in this category
    #[serde(rename = "UsedSpace")]
    used_space: Option<String>,
    /// the number of all files in this category
    #[serde(rename = "FileCount")]
    file_count: Option<String>,
}

impl Default for UsageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl UsageInfo {
    /// Information about how much space and how many files in the file system the ImageStore is using in this category
    pub fn new() -> UsageInfo {
        UsageInfo {
            used_space: None,
            file_count: None,
        }
    }

    pub fn set_used_space(&mut self, used_space: String) {
        self.used_space = Some(used_space);
    }

    pub fn with_used_space(mut self, used_space: String) -> UsageInfo {
        self.used_space = Some(used_space);
        self
    }

    pub fn used_space(&self) -> Option<&String> {
        self.used_space.as_ref()
    }

    pub fn reset_used_space(&mut self) {
        self.used_space = None;
    }

    pub fn set_file_count(&mut self, file_count: String) {
        self.file_count = Some(file_count);
    }

    pub fn with_file_count(mut self, file_count: String) -> UsageInfo {
        self.file_count = Some(file_count);
        self
    }

    pub fn file_count(&self) -> Option<&String> {
        self.file_count.as_ref()
    }

    pub fn reset_file_count(&mut self) {
        self.file_count = None;
    }
}
