/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosEventsSegment : Contains the list of Chaos events and the continuation token to get the next segment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosEventsSegment {
    /// The continuation token parameter is used to obtain next set of results. The continuation token is included in the response of the API when the results from the system do not fit in a single response. When this value is passed to the next API call, the API returns next set of results. If there are no further results, then the continuation token is not included in the response.
    #[serde(rename = "ContinuationToken")]
    continuation_token: Option<::models::ContinuationToken>,
    /// List of Chaos events that meet the user-supplied criteria.
    #[serde(rename = "History")]
    history: Option<::models::ChaosEventHistory>,
}

impl Default for ChaosEventsSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosEventsSegment {
    /// Contains the list of Chaos events and the continuation token to get the next segment.
    pub fn new() -> ChaosEventsSegment {
        ChaosEventsSegment {
            continuation_token: None,
            history: None,
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
    ) -> ChaosEventsSegment {
        self.continuation_token = Some(continuation_token);
        self
    }

    pub fn continuation_token(&self) -> Option<&::models::ContinuationToken> {
        self.continuation_token.as_ref()
    }

    pub fn reset_continuation_token(&mut self) {
        self.continuation_token = None;
    }

    pub fn set_history(&mut self, history: ::models::ChaosEventHistory) {
        self.history = Some(history);
    }

    pub fn with_history(
        mut self,
        history: ::models::ChaosEventHistory,
    ) -> ChaosEventsSegment {
        self.history = Some(history);
        self
    }

    pub fn history(&self) -> Option<&::models::ChaosEventHistory> {
        self.history.as_ref()
    }

    pub fn reset_history(&mut self) {
        self.history = None;
    }
}
