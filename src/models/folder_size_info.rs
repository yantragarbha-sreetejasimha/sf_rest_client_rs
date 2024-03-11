/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FolderSizeInfo : Information of a image store folder size

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderSizeInfo {
    /// The remote location within image store. This path is relative to the image store root.
    #[serde(rename = "StoreRelativePath")]
    store_relative_path: Option<::models::ImageStoreRelativePath>,
    /// The size of folder in bytes.
    #[serde(rename = "FolderSize")]
    folder_size: Option<String>,
}

impl Default for FolderSizeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl FolderSizeInfo {
    /// Information of a image store folder size
    pub fn new() -> FolderSizeInfo {
        FolderSizeInfo {
            store_relative_path: None,
            folder_size: None,
        }
    }

    pub fn set_store_relative_path(
        &mut self,
        store_relative_path: ::models::ImageStoreRelativePath,
    ) {
        self.store_relative_path = Some(store_relative_path);
    }

    pub fn with_store_relative_path(
        mut self,
        store_relative_path: ::models::ImageStoreRelativePath,
    ) -> FolderSizeInfo {
        self.store_relative_path = Some(store_relative_path);
        self
    }

    pub fn store_relative_path(
        &self,
    ) -> Option<&::models::ImageStoreRelativePath> {
        self.store_relative_path.as_ref()
    }

    pub fn reset_store_relative_path(&mut self) {
        self.store_relative_path = None;
    }

    pub fn set_folder_size(&mut self, folder_size: String) {
        self.folder_size = Some(folder_size);
    }

    pub fn with_folder_size(mut self, folder_size: String) -> FolderSizeInfo {
        self.folder_size = Some(folder_size);
        self
    }

    pub fn folder_size(&self) -> Option<&String> {
        self.folder_size.as_ref()
    }

    pub fn reset_folder_size(&mut self) {
        self.folder_size = None;
    }
}
