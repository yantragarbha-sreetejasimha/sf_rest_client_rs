/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FolderInfo : Information about a image store folder. It includes how many files this folder contains and its image store relative path.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderInfo {
    /// The remote location within image store. This path is relative to the image store root.
    #[serde(rename = "StoreRelativePath")]
    store_relative_path: Option<::models::ImageStoreRelativePath>,
    /// The number of files from within the image store folder.
    #[serde(rename = "FileCount")]
    file_count: Option<String>,
}

impl Default for FolderInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl FolderInfo {
    /// Information about a image store folder. It includes how many files this folder contains and its image store relative path.
    pub fn new() -> FolderInfo {
        FolderInfo {
            store_relative_path: None,
            file_count: None,
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
    ) -> FolderInfo {
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

    pub fn set_file_count(&mut self, file_count: String) {
        self.file_count = Some(file_count);
    }

    pub fn with_file_count(mut self, file_count: String) -> FolderInfo {
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
