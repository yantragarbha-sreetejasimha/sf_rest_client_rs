/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceUpgradeProgress : Information about how many replicas are completed or pending for a specific service during upgrade.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceUpgradeProgress {
    /// Name of the Service resource.
    #[serde(rename = "ServiceName")]
    service_name: Option<String>,
    /// The number of replicas that completes the upgrade in the service.
    #[serde(rename = "CompletedReplicaCount")]
    completed_replica_count: Option<String>,
    /// The number of replicas that are waiting to be upgraded in the service.
    #[serde(rename = "PendingReplicaCount")]
    pending_replica_count: Option<String>,
}

impl Default for ServiceUpgradeProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceUpgradeProgress {
    /// Information about how many replicas are completed or pending for a specific service during upgrade.
    pub fn new() -> ServiceUpgradeProgress {
        ServiceUpgradeProgress {
            service_name: None,
            completed_replica_count: None,
            pending_replica_count: None,
        }
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = Some(service_name);
    }

    pub fn with_service_name(
        mut self,
        service_name: String,
    ) -> ServiceUpgradeProgress {
        self.service_name = Some(service_name);
        self
    }

    pub fn service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    pub fn reset_service_name(&mut self) {
        self.service_name = None;
    }

    pub fn set_completed_replica_count(
        &mut self,
        completed_replica_count: String,
    ) {
        self.completed_replica_count = Some(completed_replica_count);
    }

    pub fn with_completed_replica_count(
        mut self,
        completed_replica_count: String,
    ) -> ServiceUpgradeProgress {
        self.completed_replica_count = Some(completed_replica_count);
        self
    }

    pub fn completed_replica_count(&self) -> Option<&String> {
        self.completed_replica_count.as_ref()
    }

    pub fn reset_completed_replica_count(&mut self) {
        self.completed_replica_count = None;
    }

    pub fn set_pending_replica_count(&mut self, pending_replica_count: String) {
        self.pending_replica_count = Some(pending_replica_count);
    }

    pub fn with_pending_replica_count(
        mut self,
        pending_replica_count: String,
    ) -> ServiceUpgradeProgress {
        self.pending_replica_count = Some(pending_replica_count);
        self
    }

    pub fn pending_replica_count(&self) -> Option<&String> {
        self.pending_replica_count.as_ref()
    }

    pub fn reset_pending_replica_count(&mut self) {
        self.pending_replica_count = None;
    }
}
