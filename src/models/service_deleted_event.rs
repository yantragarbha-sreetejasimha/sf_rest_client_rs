/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceDeletedEvent : Service Deleted event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDeletedEvent {
    /// The identity of the service. This ID is an encoded representation of the service name. This is used in the REST APIs to identify the service resource. Starting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the service name is \"fabric:/myapp/app1/svc1\", the service identity would be \"myapp~app1\\~svc1\" in 6.0+ and \"myapp/app1/svc1\" in previous versions.
    #[serde(rename = "ServiceId")]
    service_id: Option<::models::ServiceId>,
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
    /// Service type name.
    #[serde(rename = "ServiceTypeName")]
    service_type_name: String,
    /// Application name.
    #[serde(rename = "ApplicationName")]
    application_name: String,
    /// Application type name.
    #[serde(rename = "ApplicationTypeName")]
    application_type_name: String,
    /// Id of Service instance.
    #[serde(rename = "ServiceInstance")]
    service_instance: i64,
    /// Indicates if Service is stateful.
    #[serde(rename = "IsStateful")]
    is_stateful: bool,
    /// Number of partitions.
    #[serde(rename = "PartitionCount")]
    partition_count: i32,
    /// Size of target replicas set.
    #[serde(rename = "TargetReplicaSetSize")]
    target_replica_set_size: i32,
    /// Minimum size of replicas set.
    #[serde(rename = "MinReplicaSetSize")]
    min_replica_set_size: i32,
    /// Version of Service package.
    #[serde(rename = "ServicePackageVersion")]
    service_package_version: String,
}

impl ServiceDeletedEvent {
    /// Service Deleted event.
    pub fn new(
        kind: ::models::FabricEventKind,
        event_instance_id: String,
        time_stamp: String,
        service_type_name: String,
        application_name: String,
        application_type_name: String,
        service_instance: i64,
        is_stateful: bool,
        partition_count: i32,
        target_replica_set_size: i32,
        min_replica_set_size: i32,
        service_package_version: String,
    ) -> ServiceDeletedEvent {
        ServiceDeletedEvent {
            service_id: None,
            kind,
            event_instance_id,
            category: None,
            time_stamp,
            has_correlated_events: None,
            service_type_name,
            application_name,
            application_type_name,
            service_instance,
            is_stateful,
            partition_count,
            target_replica_set_size,
            min_replica_set_size,
            service_package_version,
        }
    }

    pub fn set_service_id(&mut self, service_id: ::models::ServiceId) {
        self.service_id = Some(service_id);
    }

    pub fn with_service_id(
        mut self,
        service_id: ::models::ServiceId,
    ) -> ServiceDeletedEvent {
        self.service_id = Some(service_id);
        self
    }

    pub fn service_id(&self) -> Option<&::models::ServiceId> {
        self.service_id.as_ref()
    }

    pub fn reset_service_id(&mut self) {
        self.service_id = None;
    }

    pub fn set_kind(&mut self, kind: ::models::FabricEventKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::FabricEventKind,
    ) -> ServiceDeletedEvent {
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
    ) -> ServiceDeletedEvent {
        self.event_instance_id = event_instance_id;
        self
    }

    pub fn event_instance_id(&self) -> &String {
        &self.event_instance_id
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn with_category(mut self, category: String) -> ServiceDeletedEvent {
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

    pub fn with_time_stamp(
        mut self,
        time_stamp: String,
    ) -> ServiceDeletedEvent {
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
    ) -> ServiceDeletedEvent {
        self.has_correlated_events = Some(has_correlated_events);
        self
    }

    pub fn has_correlated_events(&self) -> Option<&bool> {
        self.has_correlated_events.as_ref()
    }

    pub fn reset_has_correlated_events(&mut self) {
        self.has_correlated_events = None;
    }

    pub fn set_service_type_name(&mut self, service_type_name: String) {
        self.service_type_name = service_type_name;
    }

    pub fn with_service_type_name(
        mut self,
        service_type_name: String,
    ) -> ServiceDeletedEvent {
        self.service_type_name = service_type_name;
        self
    }

    pub fn service_type_name(&self) -> &String {
        &self.service_type_name
    }

    pub fn set_application_name(&mut self, application_name: String) {
        self.application_name = application_name;
    }

    pub fn with_application_name(
        mut self,
        application_name: String,
    ) -> ServiceDeletedEvent {
        self.application_name = application_name;
        self
    }

    pub fn application_name(&self) -> &String {
        &self.application_name
    }

    pub fn set_application_type_name(&mut self, application_type_name: String) {
        self.application_type_name = application_type_name;
    }

    pub fn with_application_type_name(
        mut self,
        application_type_name: String,
    ) -> ServiceDeletedEvent {
        self.application_type_name = application_type_name;
        self
    }

    pub fn application_type_name(&self) -> &String {
        &self.application_type_name
    }

    pub fn set_service_instance(&mut self, service_instance: i64) {
        self.service_instance = service_instance;
    }

    pub fn with_service_instance(
        mut self,
        service_instance: i64,
    ) -> ServiceDeletedEvent {
        self.service_instance = service_instance;
        self
    }

    pub fn service_instance(&self) -> &i64 {
        &self.service_instance
    }

    pub fn set_is_stateful(&mut self, is_stateful: bool) {
        self.is_stateful = is_stateful;
    }

    pub fn with_is_stateful(
        mut self,
        is_stateful: bool,
    ) -> ServiceDeletedEvent {
        self.is_stateful = is_stateful;
        self
    }

    pub fn is_stateful(&self) -> &bool {
        &self.is_stateful
    }

    pub fn set_partition_count(&mut self, partition_count: i32) {
        self.partition_count = partition_count;
    }

    pub fn with_partition_count(
        mut self,
        partition_count: i32,
    ) -> ServiceDeletedEvent {
        self.partition_count = partition_count;
        self
    }

    pub fn partition_count(&self) -> &i32 {
        &self.partition_count
    }

    pub fn set_target_replica_set_size(
        &mut self,
        target_replica_set_size: i32,
    ) {
        self.target_replica_set_size = target_replica_set_size;
    }

    pub fn with_target_replica_set_size(
        mut self,
        target_replica_set_size: i32,
    ) -> ServiceDeletedEvent {
        self.target_replica_set_size = target_replica_set_size;
        self
    }

    pub fn target_replica_set_size(&self) -> &i32 {
        &self.target_replica_set_size
    }

    pub fn set_min_replica_set_size(&mut self, min_replica_set_size: i32) {
        self.min_replica_set_size = min_replica_set_size;
    }

    pub fn with_min_replica_set_size(
        mut self,
        min_replica_set_size: i32,
    ) -> ServiceDeletedEvent {
        self.min_replica_set_size = min_replica_set_size;
        self
    }

    pub fn min_replica_set_size(&self) -> &i32 {
        &self.min_replica_set_size
    }

    pub fn set_service_package_version(
        &mut self,
        service_package_version: String,
    ) {
        self.service_package_version = service_package_version;
    }

    pub fn with_service_package_version(
        mut self,
        service_package_version: String,
    ) -> ServiceDeletedEvent {
        self.service_package_version = service_package_version;
        self
    }

    pub fn service_package_version(&self) -> &String {
        &self.service_package_version
    }
}
