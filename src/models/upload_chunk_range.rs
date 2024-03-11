/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UploadChunkRange : Information about which portion of the file to upload.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadChunkRange {
    /// The start position of the portion of the file. It's represented by the number of bytes.
    #[serde(rename = "StartPosition")]
    start_position: Option<String>,
    /// The end position of the portion of the file. It's represented by the number of bytes.
    #[serde(rename = "EndPosition")]
    end_position: Option<String>,
}

impl Default for UploadChunkRange {
    fn default() -> Self {
        Self::new()
    }
}

impl UploadChunkRange {
    /// Information about which portion of the file to upload.
    pub fn new() -> UploadChunkRange {
        UploadChunkRange {
            start_position: None,
            end_position: None,
        }
    }

    pub fn set_start_position(&mut self, start_position: String) {
        self.start_position = Some(start_position);
    }

    pub fn with_start_position(
        mut self,
        start_position: String,
    ) -> UploadChunkRange {
        self.start_position = Some(start_position);
        self
    }

    pub fn start_position(&self) -> Option<&String> {
        self.start_position.as_ref()
    }

    pub fn reset_start_position(&mut self) {
        self.start_position = None;
    }

    pub fn set_end_position(&mut self, end_position: String) {
        self.end_position = Some(end_position);
    }

    pub fn with_end_position(
        mut self,
        end_position: String,
    ) -> UploadChunkRange {
        self.end_position = Some(end_position);
        self
    }

    pub fn end_position(&self) -> Option<&String> {
        self.end_position.as_ref()
    }

    pub fn reset_end_position(&mut self) {
        self.end_position = None;
    }
}
