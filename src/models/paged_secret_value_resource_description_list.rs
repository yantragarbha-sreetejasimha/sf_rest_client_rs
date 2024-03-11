/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PagedSecretValueResourceDescriptionList : The list of values of a secret resource, paged if the number of results exceeds the limits of a single message. The next set of results can be obtained by executing the same query with the continuation token provided in the previous page.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedSecretValueResourceDescriptionList {
    /// The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response.
    #[serde(rename = "ContinuationToken")]
    continuation_token: Option<::models::ContinuationToken>,
    /// One page of the list.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::SecretValueResourceDescription>>,
}

impl Default for PagedSecretValueResourceDescriptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl PagedSecretValueResourceDescriptionList {
    /// The list of values of a secret resource, paged if the number of results exceeds the limits of a single message. The next set of results can be obtained by executing the same query with the continuation token provided in the previous page.
    pub fn new() -> PagedSecretValueResourceDescriptionList {
        PagedSecretValueResourceDescriptionList {
            continuation_token: None,
            items: None,
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
    ) -> PagedSecretValueResourceDescriptionList {
        self.continuation_token = Some(continuation_token);
        self
    }

    pub fn continuation_token(&self) -> Option<&::models::ContinuationToken> {
        self.continuation_token.as_ref()
    }

    pub fn reset_continuation_token(&mut self) {
        self.continuation_token = None;
    }

    pub fn set_items(
        &mut self,
        items: Vec<::models::SecretValueResourceDescription>,
    ) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::SecretValueResourceDescription>,
    ) -> PagedSecretValueResourceDescriptionList {
        self.items = Some(items);
        self
    }

    pub fn items(
        &self,
    ) -> Option<&Vec<::models::SecretValueResourceDescription>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}
