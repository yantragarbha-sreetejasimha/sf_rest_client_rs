/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PagedComposeDeploymentStatusInfoList : The list of compose deployments in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedComposeDeploymentStatusInfoList {
    /// The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response.
    #[serde(rename = "ContinuationToken")]
    continuation_token: Option<::models::ContinuationToken>,
    /// List of compose deployment status information.
    #[serde(rename = "Items")]
    items: Option<Vec<::models::ComposeDeploymentStatusInfo>>,
}

impl Default for PagedComposeDeploymentStatusInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl PagedComposeDeploymentStatusInfoList {
    /// The list of compose deployments in the cluster. The list is paged when all of the results cannot fit in a single message. The next set of results can be obtained by executing the same query with the continuation token provided in this list.
    pub fn new() -> PagedComposeDeploymentStatusInfoList {
        PagedComposeDeploymentStatusInfoList {
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
    ) -> PagedComposeDeploymentStatusInfoList {
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
        items: Vec<::models::ComposeDeploymentStatusInfo>,
    ) {
        self.items = Some(items);
    }

    pub fn with_items(
        mut self,
        items: Vec<::models::ComposeDeploymentStatusInfo>,
    ) -> PagedComposeDeploymentStatusInfoList {
        self.items = Some(items);
        self
    }

    pub fn items(&self) -> Option<&Vec<::models::ComposeDeploymentStatusInfo>> {
        self.items.as_ref()
    }

    pub fn reset_items(&mut self) {
        self.items = None;
    }
}
