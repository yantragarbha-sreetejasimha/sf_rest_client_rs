/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PagedPropertyInfoList : The paged list of Service Fabric properties under a given name. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedPropertyInfoList {
    /// The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response.
    #[serde(rename = "ContinuationToken")]
    continuation_token: Option<::models::ContinuationToken>,
    /// Indicates whether any property under the given name has been modified during the enumeration. If there was a modification, this property value is false.
    #[serde(rename = "IsConsistent")]
    is_consistent: Option<bool>,
    /// List of property information.
    #[serde(rename = "Properties")]
    properties: Option<Vec<::models::PropertyInfo>>,
}

impl Default for PagedPropertyInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl PagedPropertyInfoList {
    /// The paged list of Service Fabric properties under a given name. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list.
    pub fn new() -> PagedPropertyInfoList {
        PagedPropertyInfoList {
            continuation_token: None,
            is_consistent: None,
            properties: None,
        }
    }

    pub fn set_continuation_token(
        &mut self,
        continuation_token: ::models::ContinuationToken,
    ) {
        self.continuation_token = Some(continuation_token);
    }

    pub fn with_continuation_token(
        mut self,
        continuation_token: ::models::ContinuationToken,
    ) -> PagedPropertyInfoList {
        self.continuation_token = Some(continuation_token);
        self
    }

    pub fn continuation_token(&self) -> Option<&::models::ContinuationToken> {
        self.continuation_token.as_ref()
    }

    pub fn reset_continuation_token(&mut self) {
        self.continuation_token = None;
    }

    pub fn set_is_consistent(&mut self, is_consistent: bool) {
        self.is_consistent = Some(is_consistent);
    }

    pub fn with_is_consistent(
        mut self,
        is_consistent: bool,
    ) -> PagedPropertyInfoList {
        self.is_consistent = Some(is_consistent);
        self
    }

    pub fn is_consistent(&self) -> Option<&bool> {
        self.is_consistent.as_ref()
    }

    pub fn reset_is_consistent(&mut self) {
        self.is_consistent = None;
    }

    pub fn set_properties(&mut self, properties: Vec<::models::PropertyInfo>) {
        self.properties = Some(properties);
    }

    pub fn with_properties(
        mut self,
        properties: Vec<::models::PropertyInfo>,
    ) -> PagedPropertyInfoList {
        self.properties = Some(properties);
        self
    }

    pub fn properties(&self) -> Option<&Vec<::models::PropertyInfo>> {
        self.properties.as_ref()
    }

    pub fn reset_properties(&mut self) {
        self.properties = None;
    }
}
