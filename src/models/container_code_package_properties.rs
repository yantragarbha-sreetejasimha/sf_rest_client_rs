/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerCodePackageProperties : Describes a container and its runtime properties.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerCodePackageProperties {
    /// The name of the code package.
    #[serde(rename = "name")]
    name: String,
    /// The Container image to use.
    #[serde(rename = "image")]
    image: String,
    /// Image registry credential.
    #[serde(rename = "imageRegistryCredential")]
    image_registry_credential: Option<::models::ImageRegistryCredential>,
    /// Override for the default entry point in the container.
    #[serde(rename = "entryPoint")]
    entry_point: Option<String>,
    /// Command array to execute within the container in exec form.
    #[serde(rename = "commands")]
    commands: Option<Vec<String>>,
    /// The environment variables to set in this container
    #[serde(rename = "environmentVariables")]
    environment_variables: Option<Vec<::models::EnvironmentVariable>>,
    /// The settings to set in this container. The setting file path can be fetched from environment variable \"Fabric_SettingPath\". The path for Windows container is \"C:\\\\secrets\". The path for Linux container is \"/var/secrets\".
    #[serde(rename = "settings")]
    settings: Option<Vec<::models::Setting>>,
    /// The labels to set in this container.
    #[serde(rename = "labels")]
    labels: Option<Vec<::models::ContainerLabel>>,
    /// The endpoints exposed by this container.
    #[serde(rename = "endpoints")]
    endpoints: Option<Vec<::models::EndpointProperties>>,
    /// The resources required by this container.
    #[serde(rename = "resources")]
    resources: ::models::ResourceRequirements,
    /// Volumes to be attached to the container. The lifetime of these volumes is independent of the application's lifetime.
    #[serde(rename = "volumeRefs")]
    volume_refs: Option<Vec<::models::VolumeReference>>,
    /// Volumes to be attached to the container. The lifetime of these volumes is scoped to the application's lifetime.
    #[serde(rename = "volumes")]
    volumes: Option<Vec<::models::ApplicationScopedVolume>>,
    /// Reference to sinks in DiagnosticsDescription.
    #[serde(rename = "diagnostics")]
    diagnostics: Option<::models::DiagnosticsRef>,
    /// A list of ReliableCollection resources used by this particular code package. Please refer to ReliableCollectionsRef for more details.
    #[serde(rename = "reliableCollectionsRefs")]
    reliable_collections_refs: Option<Vec<::models::ReliableCollectionsRef>>,
    /// Runtime information of a container instance.
    #[serde(rename = "instanceView")]
    instance_view: Option<::models::ContainerInstanceView>,
    /// An array of liveness probes for a code package. It determines when to restart a code package.
    #[serde(rename = "livenessProbe")]
    liveness_probe: Option<Vec<::models::Probe>>,
    /// An array of readiness probes for a code package. It determines when to unpublish an endpoint.
    #[serde(rename = "readinessProbe")]
    readiness_probe: Option<Vec<::models::Probe>>,
}

impl ContainerCodePackageProperties {
    /// Describes a container and its runtime properties.
    pub fn new(
        name: String,
        image: String,
        resources: ::models::ResourceRequirements,
    ) -> ContainerCodePackageProperties {
        ContainerCodePackageProperties {
            name,
            image,
            image_registry_credential: None,
            entry_point: None,
            commands: None,
            environment_variables: None,
            settings: None,
            labels: None,
            endpoints: None,
            resources,
            volume_refs: None,
            volumes: None,
            diagnostics: None,
            reliable_collections_refs: None,
            instance_view: None,
            liveness_probe: None,
            readiness_probe: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> ContainerCodePackageProperties {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_image(&mut self, image: String) {
        self.image = image;
    }

    pub fn with_image(
        mut self,
        image: String,
    ) -> ContainerCodePackageProperties {
        self.image = image;
        self
    }

    pub fn image(&self) -> &String {
        &self.image
    }

    pub fn set_image_registry_credential(
        &mut self,
        image_registry_credential: ::models::ImageRegistryCredential,
    ) {
        self.image_registry_credential = Some(image_registry_credential);
    }

    pub fn with_image_registry_credential(
        mut self,
        image_registry_credential: ::models::ImageRegistryCredential,
    ) -> ContainerCodePackageProperties {
        self.image_registry_credential = Some(image_registry_credential);
        self
    }

    pub fn image_registry_credential(
        &self,
    ) -> Option<&::models::ImageRegistryCredential> {
        self.image_registry_credential.as_ref()
    }

    pub fn reset_image_registry_credential(&mut self) {
        self.image_registry_credential = None;
    }

    pub fn set_entry_point(&mut self, entry_point: String) {
        self.entry_point = Some(entry_point);
    }

    pub fn with_entry_point(
        mut self,
        entry_point: String,
    ) -> ContainerCodePackageProperties {
        self.entry_point = Some(entry_point);
        self
    }

    pub fn entry_point(&self) -> Option<&String> {
        self.entry_point.as_ref()
    }

    pub fn reset_entry_point(&mut self) {
        self.entry_point = None;
    }

    pub fn set_commands(&mut self, commands: Vec<String>) {
        self.commands = Some(commands);
    }

    pub fn with_commands(
        mut self,
        commands: Vec<String>,
    ) -> ContainerCodePackageProperties {
        self.commands = Some(commands);
        self
    }

    pub fn commands(&self) -> Option<&Vec<String>> {
        self.commands.as_ref()
    }

    pub fn reset_commands(&mut self) {
        self.commands = None;
    }

    pub fn set_environment_variables(
        &mut self,
        environment_variables: Vec<::models::EnvironmentVariable>,
    ) {
        self.environment_variables = Some(environment_variables);
    }

    pub fn with_environment_variables(
        mut self,
        environment_variables: Vec<::models::EnvironmentVariable>,
    ) -> ContainerCodePackageProperties {
        self.environment_variables = Some(environment_variables);
        self
    }

    pub fn environment_variables(
        &self,
    ) -> Option<&Vec<::models::EnvironmentVariable>> {
        self.environment_variables.as_ref()
    }

    pub fn reset_environment_variables(&mut self) {
        self.environment_variables = None;
    }

    pub fn set_settings(&mut self, settings: Vec<::models::Setting>) {
        self.settings = Some(settings);
    }

    pub fn with_settings(
        mut self,
        settings: Vec<::models::Setting>,
    ) -> ContainerCodePackageProperties {
        self.settings = Some(settings);
        self
    }

    pub fn settings(&self) -> Option<&Vec<::models::Setting>> {
        self.settings.as_ref()
    }

    pub fn reset_settings(&mut self) {
        self.settings = None;
    }

    pub fn set_labels(&mut self, labels: Vec<::models::ContainerLabel>) {
        self.labels = Some(labels);
    }

    pub fn with_labels(
        mut self,
        labels: Vec<::models::ContainerLabel>,
    ) -> ContainerCodePackageProperties {
        self.labels = Some(labels);
        self
    }

    pub fn labels(&self) -> Option<&Vec<::models::ContainerLabel>> {
        self.labels.as_ref()
    }

    pub fn reset_labels(&mut self) {
        self.labels = None;
    }

    pub fn set_endpoints(
        &mut self,
        endpoints: Vec<::models::EndpointProperties>,
    ) {
        self.endpoints = Some(endpoints);
    }

    pub fn with_endpoints(
        mut self,
        endpoints: Vec<::models::EndpointProperties>,
    ) -> ContainerCodePackageProperties {
        self.endpoints = Some(endpoints);
        self
    }

    pub fn endpoints(&self) -> Option<&Vec<::models::EndpointProperties>> {
        self.endpoints.as_ref()
    }

    pub fn reset_endpoints(&mut self) {
        self.endpoints = None;
    }

    pub fn set_resources(&mut self, resources: ::models::ResourceRequirements) {
        self.resources = resources;
    }

    pub fn with_resources(
        mut self,
        resources: ::models::ResourceRequirements,
    ) -> ContainerCodePackageProperties {
        self.resources = resources;
        self
    }

    pub fn resources(&self) -> &::models::ResourceRequirements {
        &self.resources
    }

    pub fn set_volume_refs(
        &mut self,
        volume_refs: Vec<::models::VolumeReference>,
    ) {
        self.volume_refs = Some(volume_refs);
    }

    pub fn with_volume_refs(
        mut self,
        volume_refs: Vec<::models::VolumeReference>,
    ) -> ContainerCodePackageProperties {
        self.volume_refs = Some(volume_refs);
        self
    }

    pub fn volume_refs(&self) -> Option<&Vec<::models::VolumeReference>> {
        self.volume_refs.as_ref()
    }

    pub fn reset_volume_refs(&mut self) {
        self.volume_refs = None;
    }

    pub fn set_volumes(
        &mut self,
        volumes: Vec<::models::ApplicationScopedVolume>,
    ) {
        self.volumes = Some(volumes);
    }

    pub fn with_volumes(
        mut self,
        volumes: Vec<::models::ApplicationScopedVolume>,
    ) -> ContainerCodePackageProperties {
        self.volumes = Some(volumes);
        self
    }

    pub fn volumes(&self) -> Option<&Vec<::models::ApplicationScopedVolume>> {
        self.volumes.as_ref()
    }

    pub fn reset_volumes(&mut self) {
        self.volumes = None;
    }

    pub fn set_diagnostics(&mut self, diagnostics: ::models::DiagnosticsRef) {
        self.diagnostics = Some(diagnostics);
    }

    pub fn with_diagnostics(
        mut self,
        diagnostics: ::models::DiagnosticsRef,
    ) -> ContainerCodePackageProperties {
        self.diagnostics = Some(diagnostics);
        self
    }

    pub fn diagnostics(&self) -> Option<&::models::DiagnosticsRef> {
        self.diagnostics.as_ref()
    }

    pub fn reset_diagnostics(&mut self) {
        self.diagnostics = None;
    }

    pub fn set_reliable_collections_refs(
        &mut self,
        reliable_collections_refs: Vec<::models::ReliableCollectionsRef>,
    ) {
        self.reliable_collections_refs = Some(reliable_collections_refs);
    }

    pub fn with_reliable_collections_refs(
        mut self,
        reliable_collections_refs: Vec<::models::ReliableCollectionsRef>,
    ) -> ContainerCodePackageProperties {
        self.reliable_collections_refs = Some(reliable_collections_refs);
        self
    }

    pub fn reliable_collections_refs(
        &self,
    ) -> Option<&Vec<::models::ReliableCollectionsRef>> {
        self.reliable_collections_refs.as_ref()
    }

    pub fn reset_reliable_collections_refs(&mut self) {
        self.reliable_collections_refs = None;
    }

    pub fn set_instance_view(
        &mut self,
        instance_view: ::models::ContainerInstanceView,
    ) {
        self.instance_view = Some(instance_view);
    }

    pub fn with_instance_view(
        mut self,
        instance_view: ::models::ContainerInstanceView,
    ) -> ContainerCodePackageProperties {
        self.instance_view = Some(instance_view);
        self
    }

    pub fn instance_view(&self) -> Option<&::models::ContainerInstanceView> {
        self.instance_view.as_ref()
    }

    pub fn reset_instance_view(&mut self) {
        self.instance_view = None;
    }

    pub fn set_liveness_probe(&mut self, liveness_probe: Vec<::models::Probe>) {
        self.liveness_probe = Some(liveness_probe);
    }

    pub fn with_liveness_probe(
        mut self,
        liveness_probe: Vec<::models::Probe>,
    ) -> ContainerCodePackageProperties {
        self.liveness_probe = Some(liveness_probe);
        self
    }

    pub fn liveness_probe(&self) -> Option<&Vec<::models::Probe>> {
        self.liveness_probe.as_ref()
    }

    pub fn reset_liveness_probe(&mut self) {
        self.liveness_probe = None;
    }

    pub fn set_readiness_probe(
        &mut self,
        readiness_probe: Vec<::models::Probe>,
    ) {
        self.readiness_probe = Some(readiness_probe);
    }

    pub fn with_readiness_probe(
        mut self,
        readiness_probe: Vec<::models::Probe>,
    ) -> ContainerCodePackageProperties {
        self.readiness_probe = Some(readiness_probe);
        self
    }

    pub fn readiness_probe(&self) -> Option<&Vec<::models::Probe>> {
        self.readiness_probe.as_ref()
    }

    pub fn reset_readiness_probe(&mut self) {
        self.readiness_probe = None;
    }
}
