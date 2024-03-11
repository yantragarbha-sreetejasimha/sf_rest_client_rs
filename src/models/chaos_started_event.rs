/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.0.0.42
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ChaosStartedEvent : Chaos Started event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChaosStartedEvent {
    /// The kind of FabricEvent.
    #[serde(rename = "Kind")]
    kind: ::models::FabricEventKind,
    /// The identifier for the FabricEvent instance.
    #[serde(rename = "EventInstanceId")]
    event_instance_id: String,
    /// The category of event.
    #[serde(rename = "Category")]
    category: Option<String>,
    /// The time event was logged.
    #[serde(rename = "TimeStamp")]
    time_stamp: String,
    /// Shows there is existing related events available.
    #[serde(rename = "HasCorrelatedEvents")]
    has_correlated_events: Option<bool>,
    /// Maximum number of concurrent faults.
    #[serde(rename = "MaxConcurrentFaults")]
    max_concurrent_faults: i64,
    /// Time to run in seconds.
    #[serde(rename = "TimeToRunInSeconds")]
    time_to_run_in_seconds: f64,
    /// Maximum timeout for cluster stabilization in seconds.
    #[serde(rename = "MaxClusterStabilizationTimeoutInSeconds")]
    max_cluster_stabilization_timeout_in_seconds: f64,
    /// Wait time between iterations in seconds.
    #[serde(rename = "WaitTimeBetweenIterationsInSeconds")]
    wait_time_between_iterations_in_seconds: f64,
    /// Wait time between faults in seconds.
    #[serde(rename = "WaitTimeBetweenFaultsInSeconds")]
    wait_time_between_faults_in_seconds: f64,
    /// Indicates MoveReplica fault is enabled.
    #[serde(rename = "MoveReplicaFaultEnabled")]
    move_replica_fault_enabled: bool,
    /// List of included Node types.
    #[serde(rename = "IncludedNodeTypeList")]
    included_node_type_list: String,
    /// List of included Applications.
    #[serde(rename = "IncludedApplicationList")]
    included_application_list: String,
    /// Health policy.
    #[serde(rename = "ClusterHealthPolicy")]
    cluster_health_policy: String,
    /// Chaos Context.
    #[serde(rename = "ChaosContext")]
    chaos_context: String,
}

impl ChaosStartedEvent {
    /// Chaos Started event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        max_concurrent_faults: i64,
        time_to_run_in_seconds: f64,
        max_cluster_stabilization_timeout_in_seconds: f64,
        wait_time_between_iterations_in_seconds: f64,
        wait_time_between_faults_in_seconds: f64,
        move_replica_fault_enabled: bool,
        included_node_type_list: String,
        included_application_list: String,
        cluster_health_policy: String,
        chaos_context: String,
    ) -> ChaosStartedEvent {
        ChaosStartedEvent {
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            max_concurrent_faults,
            time_to_run_in_seconds,
            max_cluster_stabilization_timeout_in_seconds,
            wait_time_between_iterations_in_seconds,
            wait_time_between_faults_in_seconds,
            move_replica_fault_enabled,
            included_node_type_list,
            included_application_list,
            cluster_health_policy,
            chaos_context,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ChaosStartedEvent {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::FabricEventKind {
        &self.kind
    }

    pub fn set_event_instance_id(&mut self, event_instance_id: String) {
        self.event_instance_id = event_instance_id;
    }

    pub fn with_event_instance_id(
        mut self,
        event_instance_id: String,
    ) -> ChaosStartedEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn with_category(mut self, category: String) -> ChaosStartedEvent {
        self.category = Some(category);
        self
    }

    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    pub fn reset_category(&mut self) {
        self.category = None;
    }

    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    pub fn with_time_stamp(mut self, time_stamp: String) -> ChaosStartedEvent {
        self.time_stamp = time_stamp;
        self
    }

    pub fn time_stamp(&self) -> &String {
        &self.time_stamp
    }

    pub fn set_has_correlated_events(&mut self, has_correlated_events: bool) {
        self.has_correlated_events = Some(has_correlated_events);
    }

    pub fn with_has_correlated_events(
        mut self,
        has_correlated_events: bool,
    ) -> ChaosStartedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_max_concurrent_faults(&mut self, max_concurrent_faults: i64) {
        self.max_concurrent_faults = max_concurrent_faults;
    }

    pub fn with_max_concurrent_faults(
        mut self,
        max_concurrent_faults: i64,
    ) -> ChaosStartedEvent {
        self.max_concurrent_faults = max_concurrent_faults;
        self
    }

    pub fn max_concurrent_faults(&self) -> &i64 {
        &self.max_concurrent_faults
    }

    pub fn set_time_to_run_in_seconds(&mut self, time_to_run_in_seconds: f64) {
        self.time_to_run_in_seconds = time_to_run_in_seconds;
    }

    pub fn with_time_to_run_in_seconds(
        mut self,
        time_to_run_in_seconds: f64,
    ) -> ChaosStartedEvent {
        self.time_to_run_in_seconds = time_to_run_in_seconds;
        self
    }

    pub fn time_to_run_in_seconds(&self) -> &f64 {
        &self.time_to_run_in_seconds
    }

    pub fn set_max_cluster_stabilization_timeout_in_seconds(
        &mut self,
        max_cluster_stabilization_timeout_in_seconds: f64,
    ) {
        self.max_cluster_stabilization_timeout_in_seconds =
            max_cluster_stabilization_timeout_in_seconds;
    }

    pub fn with_max_cluster_stabilization_timeout_in_seconds(
        mut self,
        max_cluster_stabilization_timeout_in_seconds: f64,
    ) -> ChaosStartedEvent {
        self.max_cluster_stabilization_timeout_in_seconds =
            max_cluster_stabilization_timeout_in_seconds;
        self
    }

    pub fn max_cluster_stabilization_timeout_in_seconds(&self) -> &f64 {
        &self.max_cluster_stabilization_timeout_in_seconds
    }

    pub fn set_wait_time_between_iterations_in_seconds(
        &mut self,
        wait_time_between_iterations_in_seconds: f64,
    ) {
        self.wait_time_between_iterations_in_seconds =
            wait_time_between_iterations_in_seconds;
    }

    pub fn with_wait_time_between_iterations_in_seconds(
        mut self,
        wait_time_between_iterations_in_seconds: f64,
    ) -> ChaosStartedEvent {
        self.wait_time_between_iterations_in_seconds =
            wait_time_between_iterations_in_seconds;
        self
    }

    pub fn wait_time_between_iterations_in_seconds(&self) -> &f64 {
        &self.wait_time_between_iterations_in_seconds
    }

    pub fn set_wait_time_between_faults_in_seconds(
        &mut self,
        wait_time_between_faults_in_seconds: f64,
    ) {
        self.wait_time_between_faults_in_seconds =
            wait_time_between_faults_in_seconds;
    }

    pub fn with_wait_time_between_faults_in_seconds(
        mut self,
        wait_time_between_faults_in_seconds: f64,
    ) -> ChaosStartedEvent {
        self.wait_time_between_faults_in_seconds =
            wait_time_between_faults_in_seconds;
        self
    }

    pub fn wait_time_between_faults_in_seconds(&self) -> &f64 {
        &self.wait_time_between_faults_in_seconds
    }

    pub fn set_move_replica_fault_enabled(
        &mut self,
        move_replica_fault_enabled: bool,
    ) {
        self.move_replica_fault_enabled = move_replica_fault_enabled;
    }

    pub fn with_move_replica_fault_enabled(
        mut self,
        move_replica_fault_enabled: bool,
    ) -> ChaosStartedEvent {
        self.move_replica_fault_enabled = move_replica_fault_enabled;
        self
    }

    pub fn move_replica_fault_enabled(&self) -> &bool {
        &self.move_replica_fault_enabled
    }

    pub fn set_included_node_type_list(
        &mut self,
        included_node_type_list: String,
    ) {
        self.included_node_type_list = included_node_type_list;
    }

    pub fn with_included_node_type_list(
        mut self,
        included_node_type_list: String,
    ) -> ChaosStartedEvent {
        self.included_node_type_list = included_node_type_list;
        self
    }

    pub fn included_node_type_list(&self) -> &String {
        &self.included_node_type_list
    }

    pub fn set_included_application_list(
        &mut self,
        included_application_list: String,
    ) {
        self.included_application_list = included_application_list;
    }

    pub fn with_included_application_list(
        mut self,
        included_application_list: String,
    ) -> ChaosStartedEvent {
        self.included_application_list = included_application_list;
        self
    }

    pub fn included_application_list(&self) -> &String {
        &self.included_application_list
    }

    pub fn set_cluster_health_policy(&mut self, cluster_health_policy: String) {
        self.cluster_health_policy = cluster_health_policy;
    }

    pub fn with_cluster_health_policy(
        mut self,
        cluster_health_policy: String,
    ) -> ChaosStartedEvent {
        self.cluster_health_policy = cluster_health_policy;
        self
    }

    pub fn cluster_health_policy(&self) -> &String {
        &self.cluster_health_policy
    }

    pub fn set_chaos_context(&mut self, chaos_context: String) {
        self.chaos_context = chaos_context;
    }

    pub fn with_chaos_context(
        mut self,
        chaos_context: String,
    ) -> ChaosStartedEvent {
        self.chaos_context = chaos_context;
        self
    }

    pub fn chaos_context(&self) -> &String {
        &self.chaos_context
    }
}
