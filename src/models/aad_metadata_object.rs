/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.3.0.9
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AadMetadataObject : Azure Active Directory metadata object used for secured connection to cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AadMetadataObject {
    /// The client authentication method.
    #[serde(rename = "type")]
    _type: Option<String>,
    /// Azure Active Directory metadata used for secured connection to cluster.
    #[serde(rename = "metadata")]
    metadata: Option<::models::AadMetadata>,
}

impl Default for AadMetadataObject {
    fn default() -> Self {
        Self::new()
    }
}

impl AadMetadataObject {
    /// Azure Active Directory metadata object used for secured connection to cluster.
    pub fn new() -> AadMetadataObject {
        AadMetadataObject {
            _type: None,
            metadata: None,
        }
    }

    pub fn set_type(&mut self, _type: String) {
        self._type = Some(_type);
    }

    pub fn with_type(mut self, _type: String) -> AadMetadataObject {
        self._type = Some(_type);
        self
    }

    pub fn _type(&self) -> Option<&String> {
        self._type.as_ref()
    }

    pub fn reset_type(&mut self) {
        self._type = None;
    }

    pub fn set_metadata(&mut self, metadata: ::models::AadMetadata) {
        self.metadata = Some(metadata);
    }

    pub fn with_metadata(
        mut self,
        metadata: ::models::AadMetadata,
    ) -> AadMetadataObject {
        self.metadata = Some(metadata);
        self
    }

    pub fn metadata(&self) -> Option<&::models::AadMetadata> {
        self.metadata.as_ref()
    }

    pub fn reset_metadata(&mut self) {
        self.metadata = None;
    }
}
