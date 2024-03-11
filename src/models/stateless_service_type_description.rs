/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatelessServiceTypeDescription : Describes a stateless service type defined in the service manifest of a provisioned application type.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatelessServiceTypeDescription {
    /// The kind of service (Stateless or Stateful).
    #[serde(rename = "Kind")]
    kind: ::models::ServiceKind,
    /// Indicates whether the service type is a stateful service type or a stateless service type. This property is true if the service type is a stateful service type, false otherwise.
    #[serde(rename = "IsStateful")]
    is_stateful: Option<bool>,
    /// Name of the service type as specified in the service manifest.
    #[serde(rename = "ServiceTypeName")]
    service_type_name: Option<::models::ServiceTypeName>,
    /// The placement constraint to be used when instantiating this service in a Service Fabric cluster.
    #[serde(rename = "PlacementConstraints")]
    placement_constraints: Option<String>,
    /// The service load metrics is given as an array of ServiceLoadMetricDescription objects.
    #[serde(rename = "LoadMetrics")]
    load_metrics: Option<::models::ServiceLoadMetricsList>,
    /// List of service placement policy descriptions.
    #[serde(rename = "ServicePlacementPolicies")]
    service_placement_policies:
        Option<::models::ServicePlacementPolicyDescriptionList>,
    /// List of service type extensions.
    #[serde(rename = "Extensions")]
    extensions: Option<::models::ServiceTypeExtensionDescriptionList>,
    /// A flag indicating if this type is not implemented and hosted by a user service process, but is implicitly hosted by a system created process. This value is true for services using the guest executable services, false otherwise.
    #[serde(rename = "UseImplicitHost")]
    use_implicit_host: Option<bool>,
}

impl StatelessServiceTypeDescription {
    /// Describes a stateless service type defined in the service manifest of a provisioned application type.
    pub fn new(kind: ::models::ServiceKind) -> StatelessServiceTypeDescription {
        StatelessServiceTypeDescription {
            kind,
            is_stateful: None,
            service_type_name: None,
            placement_constraints: None,
            load_metrics: None,
            service_placement_policies: None,
            extensions: None,
            use_implicit_host: None,
        }
    }

    pub fn set_kind(&mut self, kind: ::models::ServiceKind) {
        self.kind = kind;
    }

    pub fn with_kind(
        mut self,
        kind: ::models::ServiceKind,
    ) -> StatelessServiceTypeDescription {
        self.kind = kind;
        self
    }

    pub fn kind(&self) -> &::models::ServiceKind {
        &self.kind
    }

    pub fn set_is_stateful(&mut self, is_stateful: bool) {
        self.is_stateful = Some(is_stateful);
    }

    pub fn with_is_stateful(
        mut self,
        is_stateful: bool,
    ) -> StatelessServiceTypeDescription {
        self.is_stateful = Some(is_stateful);
        self
    }

    pub fn is_stateful(&self) -> Option<&bool> {
        self.is_stateful.as_ref()
    }

    pub fn reset_is_stateful(&mut self) {
        self.is_stateful = None;
    }

    pub fn set_service_type_name(
        &mut self,
        service_type_name: ::models::ServiceTypeName,
    ) {
        self.service_type_name = Some(service_type_name);
    }

    pub fn with_service_type_name(
        mut self,
        service_type_name: ::models::ServiceTypeName,
    ) -> StatelessServiceTypeDescription {
        self.service_type_name = Some(service_type_name);
        self
    }

    pub fn service_type_name(&self) -> Option<&::models::ServiceTypeName> {
        self.service_type_name.as_ref()
    }

    pub fn reset_service_type_name(&mut self) {
        self.service_type_name = None;
    }

    pub fn set_placement_constraints(&mut self, placement_constraints: String) {
        self.placement_constraints = Some(placement_constraints);
    }

    pub fn with_placement_constraints(
        mut self,
        placement_constraints: String,
    ) -> StatelessServiceTypeDescription {
        self.placement_constraints = Some(placement_constraints);
        self
    }

    pub fn placement_constraints(&self) -> Option<&String> {
        self.placement_constraints.as_ref()
    }

    pub fn reset_placement_constraints(&mut self) {
        self.placement_constraints = None;
    }

    pub fn set_load_metrics(
        &mut self,
        load_metrics: ::models::ServiceLoadMetricsList,
    ) {
        self.load_metrics = Some(load_metrics);
    }

    pub fn with_load_metrics(
        mut self,
        load_metrics: ::models::ServiceLoadMetricsList,
    ) -> StatelessServiceTypeDescription {
        self.load_metrics = Some(load_metrics);
        self
    }

    pub fn load_metrics(&self) -> Option<&::models::ServiceLoadMetricsList> {
        self.load_metrics.as_ref()
    }

    pub fn reset_load_metrics(&mut self) {
        self.load_metrics = None;
    }

    pub fn set_service_placement_policies(
        &mut self,
        service_placement_policies: ::models::ServicePlacementPolicyDescriptionList,
    ) {
        self.service_placement_policies = Some(service_placement_policies);
    }

    pub fn with_service_placement_policies(
        mut self,
        service_placement_policies: ::models::ServicePlacementPolicyDescriptionList,
    ) -> StatelessServiceTypeDescription {
        self.service_placement_policies = Some(service_placement_policies);
        self
    }

    pub fn service_placement_policies(
        &self,
    ) -> Option<&::models::ServicePlacementPolicyDescriptionList> {
        self.service_placement_policies.as_ref()
    }

    pub fn reset_service_placement_policies(&mut self) {
        self.service_placement_policies = None;
    }

    pub fn set_extensions(
        &mut self,
        extensions: ::models::ServiceTypeExtensionDescriptionList,
    ) {
        self.extensions = Some(extensions);
    }

    pub fn with_extensions(
        mut self,
        extensions: ::models::ServiceTypeExtensionDescriptionList,
    ) -> StatelessServiceTypeDescription {
        self.extensions = Some(extensions);
        self
    }

    pub fn extensions(
        &self,
    ) -> Option<&::models::ServiceTypeExtensionDescriptionList> {
        self.extensions.as_ref()
    }

    pub fn reset_extensions(&mut self) {
        self.extensions = None;
    }

    pub fn set_use_implicit_host(&mut self, use_implicit_host: bool) {
        self.use_implicit_host = Some(use_implicit_host);
    }

    pub fn with_use_implicit_host(
        mut self,
        use_implicit_host: bool,
    ) -> StatelessServiceTypeDescription {
        self.use_implicit_host = Some(use_implicit_host);
        self
    }

    pub fn use_implicit_host(&self) -> Option<&bool> {
        self.use_implicit_host.as_ref()
    }

    pub fn reset_use_implicit_host(&mut self) {
        self.use_implicit_host = None;
    }
}
