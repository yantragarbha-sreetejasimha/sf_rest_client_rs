/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ImageStoreInfo : Information about the ImageStore's resource usage

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageStoreInfo {
    /// disk capacity and available disk space on the node where the ImageStore primary is placed.
    #[serde(rename = "DiskInfo")]
    disk_info: Option<::models::DiskInfo>,
    /// the ImageStore's file system usage for metadata.
    #[serde(rename = "UsedByMetadata")]
    used_by_metadata: Option<::models::UsageInfo>,
    /// The ImageStore's file system usage for staging files that are being uploaded.
    #[serde(rename = "UsedByStaging")]
    used_by_staging: Option<::models::UsageInfo>,
    /// the ImageStore's file system usage for copied application and cluster packages. [Removing application and cluster packages](https://docs.microsoft.com/rest/api/servicefabric/sfclient-api-deleteimagestorecontent) will free up this space.
    #[serde(rename = "UsedByCopy")]
    used_by_copy: Option<::models::UsageInfo>,
    /// the ImageStore's file system usage for registered and cluster packages. [Unregistering application](https://docs.microsoft.com/rest/api/servicefabric/sfclient-api-unprovisionapplicationtype) and [cluster packages](https://docs.microsoft.com/rest/api/servicefabric/sfclient-api-unprovisionapplicationtype) will free up this space.
    #[serde(rename = "UsedByRegister")]
    used_by_register: Option<::models::UsageInfo>,
}

impl Default for ImageStoreInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageStoreInfo {
    /// Information about the ImageStore's resource usage
    pub fn new() -> ImageStoreInfo {
        ImageStoreInfo {
            disk_info: None,
            used_by_metadata: None,
            used_by_staging: None,
            used_by_copy: None,
            used_by_register: None,
        }
    }

    pub fn set_disk_info(&mut self, disk_info: ::models::DiskInfo) {
        self.disk_info = Some(disk_info);
    }

    pub fn with_disk_info(
        mut self,
        disk_info: ::models::DiskInfo,
    ) -> ImageStoreInfo {
        self.disk_info = Some(disk_info);
        self
    }

    pub fn disk_info(&self) -> Option<&::models::DiskInfo> {
        self.disk_info.as_ref()
    }

    pub fn reset_disk_info(&mut self) {
        self.disk_info = None;
    }

    pub fn set_used_by_metadata(
        &mut self,
        used_by_metadata: ::models::UsageInfo,
    ) {
        self.used_by_metadata = Some(used_by_metadata);
    }

    pub fn with_used_by_metadata(
        mut self,
        used_by_metadata: ::models::UsageInfo,
    ) -> ImageStoreInfo {
        self.used_by_metadata = Some(used_by_metadata);
        self
    }

    pub fn used_by_metadata(&self) -> Option<&::models::UsageInfo> {
        self.used_by_metadata.as_ref()
    }

    pub fn reset_used_by_metadata(&mut self) {
        self.used_by_metadata = None;
    }

    pub fn set_used_by_staging(
        &mut self,
        used_by_staging: ::models::UsageInfo,
    ) {
        self.used_by_staging = Some(used_by_staging);
    }

    pub fn with_used_by_staging(
        mut self,
        used_by_staging: ::models::UsageInfo,
    ) -> ImageStoreInfo {
        self.used_by_staging = Some(used_by_staging);
        self
    }

    pub fn used_by_staging(&self) -> Option<&::models::UsageInfo> {
        self.used_by_staging.as_ref()
    }

    pub fn reset_used_by_staging(&mut self) {
        self.used_by_staging = None;
    }

    pub fn set_used_by_copy(&mut self, used_by_copy: ::models::UsageInfo) {
        self.used_by_copy = Some(used_by_copy);
    }

    pub fn with_used_by_copy(
        mut self,
        used_by_copy: ::models::UsageInfo,
    ) -> ImageStoreInfo {
        self.used_by_copy = Some(used_by_copy);
        self
    }

    pub fn used_by_copy(&self) -> Option<&::models::UsageInfo> {
        self.used_by_copy.as_ref()
    }

    pub fn reset_used_by_copy(&mut self) {
        self.used_by_copy = None;
    }

    pub fn set_used_by_register(
        &mut self,
        used_by_register: ::models::UsageInfo,
    ) {
        self.used_by_register = Some(used_by_register);
    }

    pub fn with_used_by_register(
        mut self,
        used_by_register: ::models::UsageInfo,
    ) -> ImageStoreInfo {
        self.used_by_register = Some(used_by_register);
        self
    }

    pub fn used_by_register(&self) -> Option<&::models::UsageInfo> {
        self.used_by_register.as_ref()
    }

    pub fn reset_used_by_register(&mut self) {
        self.used_by_register = None;
    }
}