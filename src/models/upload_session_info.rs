/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UploadSessionInfo : Information about an image store upload session. A session is associated with a relative path in the image store.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadSessionInfo {
    /// The remote location within image store. This path is relative to the image store root.
    #[serde(rename = "StoreRelativePath")]
    store_relative_path: Option<String>,
    /// A unique ID of the upload session. A session ID can be reused only if the session was committed or removed.
    #[serde(rename = "SessionId")]
    session_id: Option<String>,
    /// The date and time when the upload session was last modified.
    #[serde(rename = "ModifiedDate")]
    modified_date: Option<String>,
    /// The size in bytes of the uploading file.
    #[serde(rename = "FileSize")]
    file_size: Option<String>,
    /// List of chunk ranges that image store has not received yet.
    #[serde(rename = "ExpectedRanges")]
    expected_ranges: Option<Vec<::models::UploadChunkRange>>,
}

impl Default for UploadSessionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl UploadSessionInfo {
    /// Information about an image store upload session. A session is associated with a relative path in the image store.
    pub fn new() -> UploadSessionInfo {
        UploadSessionInfo {
            store_relative_path: None,
            session_id: None,
            modified_date: None,
            file_size: None,
            expected_ranges: None,
        }
    }

    pub fn set_store_relative_path(&mut self, store_relative_path: String) {
        self.store_relative_path = Some(store_relative_path);
    }

    pub fn with_store_relative_path(
        mut self,
        store_relative_path: String,
    ) -> UploadSessionInfo {
        self.store_relative_path = Some(store_relative_path);
        self
    }

    pub fn store_relative_path(&self) -> Option<&String> {
        self.store_relative_path.as_ref()
    }

    pub fn reset_store_relative_path(&mut self) {
        self.store_relative_path = None;
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = Some(session_id);
    }

    pub fn with_session_id(mut self, session_id: String) -> UploadSessionInfo {
        self.session_id = Some(session_id);
        self
    }

    pub fn session_id(&self) -> Option<&String> {
        self.session_id.as_ref()
    }

    pub fn reset_session_id(&mut self) {
        self.session_id = None;
    }

    pub fn set_modified_date(&mut self, modified_date: String) {
        self.modified_date = Some(modified_date);
    }

    pub fn with_modified_date(
        mut self,
        modified_date: String,
    ) -> UploadSessionInfo {
        self.modified_date = Some(modified_date);
        self
    }

    pub fn modified_date(&self) -> Option<&String> {
        self.modified_date.as_ref()
    }

    pub fn reset_modified_date(&mut self) {
        self.modified_date = None;
    }

    pub fn set_file_size(&mut self, file_size: String) {
        self.file_size = Some(file_size);
    }

    pub fn with_file_size(mut self, file_size: String) -> UploadSessionInfo {
        self.file_size = Some(file_size);
        self
    }

    pub fn file_size(&self) -> Option<&String> {
        self.file_size.as_ref()
    }

    pub fn reset_file_size(&mut self) {
        self.file_size = None;
    }

    pub fn set_expected_ranges(
        &mut self,
        expected_ranges: Vec<::models::UploadChunkRange>,
    ) {
        self.expected_ranges = Some(expected_ranges);
    }

    pub fn with_expected_ranges(
        mut self,
        expected_ranges: Vec<::models::UploadChunkRange>,
    ) -> UploadSessionInfo {
        self.expected_ranges = Some(expected_ranges);
        self
    }

    pub fn expected_ranges(&self) -> Option<&Vec<::models::UploadChunkRange>> {
        self.expected_ranges.as_ref()
    }

    pub fn reset_expected_ranges(&mut self) {
        self.expected_ranges = None;
    }
}
