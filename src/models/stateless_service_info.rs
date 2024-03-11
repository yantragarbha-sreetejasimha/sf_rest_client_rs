/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatelessServiceInfo : Information about a stateless Service Fabric service.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatelessServiceInfo {
    /// The identity of the service. This ID is an encoded representation of the service name. This is used in the REST APIs to identify the service resource. Starting in version 6.0, hierarchical names are delimited with the \"\\~\" character. For example, if the service name is \"fabric:/myapp/app1/svc1\", the service identity would be \"myapp~app1\\~svc1\" in 6.0+ and \"myapp/app1/svc1\" in previous versions.
    #[serde(rename = "Id")]
    id: Option<::models::ServiceId>,
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "ServiceKind")]
    service_kind: ::models::ServiceKind,
    /// The full name of the service with 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: Option<::models::ServiceName>,
    /// Name of the service type as specified in the service manifest.
    #[serde(rename = "TypeName")]
    type_name: Option<::models::ServiceTypeName>,
    /// The version of the service manifest.
    #[serde(rename = "ManifestVersion")]
    manifest_version: Option<String>,
    /// The health state of a Service Fabric entity such as Cluster, Node, Application, Service, Partition, Replica etc.
    #[serde(rename = "HealthState")]
    health_state: Option<::models::HealthState>,
    /// The status of the application.
    #[serde(rename = "ServiceStatus")]
    service_status: Option<::models::ServiceStatus>,
    /// Whether the service is in a service group.
    #[serde(rename = "IsServiceGroup")]
    is_service_group: Option<bool>,
}

impl StatelessServiceInfo {
    /// Information about a stateless Service Fabric service.
    pub fn new(service_kind: ::models::ServiceKind) -> StatelessServiceInfo {
        StatelessServiceInfo {
            id: None,
            service_kind,
            name: None,
            type_name: None,
            manifest_version: None,
            health_state: None,
            service_status: None,
            is_service_group: None,
        }
    }

    pub fn set_id(&mut self, id: ::models::ServiceId) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: ::models::ServiceId) -> StatelessServiceInfo {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&::models::ServiceId> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_service_kind(&mut self, service_kind: ::models::ServiceKind) {
        self.service_kind = service_kind;
    }

    pub fn with_service_kind(
        mut self,
        service_kind: ::models::ServiceKind,
    ) -> StatelessServiceInfo {
        self.service_kind = service_kind;
        self
    }

    pub fn service_kind(&self) -> &::models::ServiceKind {
        &self.service_kind
    }

    pub fn set_name(&mut self, name: ::models::ServiceName) {
        self.name = Some(name);
    }

    pub fn with_name(
        mut self,
        name: ::models::ServiceName,
    ) -> StatelessServiceInfo {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&::models::ServiceName> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_type_name(&mut self, type_name: ::models::ServiceTypeName) {
        self.type_name = Some(type_name);
    }

    pub fn with_type_name(
        mut self,
        type_name: ::models::ServiceTypeName,
    ) -> StatelessServiceInfo {
        self.type_name = Some(type_name);
        self
    }

    pub fn type_name(&self) -> Option<&::models::ServiceTypeName> {
        self.type_name.as_ref()
    }

    pub fn reset_type_name(&mut self) {
        self.type_name = None;
    }

    pub fn set_manifest_version(&mut self, manifest_version: String) {
        self.manifest_version = Some(manifest_version);
    }

    pub fn with_manifest_version(
        mut self,
        manifest_version: String,
    ) -> StatelessServiceInfo {
        self.manifest_version = Some(manifest_version);
        self
    }

    pub fn manifest_version(&self) -> Option<&String> {
        self.manifest_version.as_ref()
    }

    pub fn reset_manifest_version(&mut self) {
        self.manifest_version = None;
    }

    pub fn set_health_state(&mut self, health_state: ::models::HealthState) {
        self.health_state = Some(health_state);
    }

    pub fn with_health_state(
        mut self,
        health_state: ::models::HealthState,
    ) -> StatelessServiceInfo {
        self.health_state = Some(health_state);
        self
    }

    pub fn health_state(&self) -> Option<&::models::HealthState> {
        self.health_state.as_ref()
    }

    pub fn reset_health_state(&mut self) {
        self.health_state = None;
    }

    pub fn set_service_status(
        &mut self,
        service_status: ::models::ServiceStatus,
    ) {
        self.service_status = Some(service_status);
    }

    pub fn with_service_status(
        mut self,
        service_status: ::models::ServiceStatus,
    ) -> StatelessServiceInfo {
        self.service_status = Some(service_status);
        self
    }

    pub fn service_status(&self) -> Option<&::models::ServiceStatus> {
        self.service_status.as_ref()
    }

    pub fn reset_service_status(&mut self) {
        self.service_status = None;
    }

    pub fn set_is_service_group(&mut self, is_service_group: bool) {
        self.is_service_group = Some(is_service_group);
    }

    pub fn with_is_service_group(
        mut self,
        is_service_group: bool,
    ) -> StatelessServiceInfo {
        self.is_service_group = Some(is_service_group);
        self
    }

    pub fn is_service_group(&self) -> Option<&bool> {
        self.is_service_group.as_ref()
    }

    pub fn reset_is_service_group(&mut self) {
        self.is_service_group = None;
    }
}
