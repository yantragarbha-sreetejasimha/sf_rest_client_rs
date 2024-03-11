use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient<C: hyper::client::Connect> {
    configuration: Rc<Configuration<C>>,
    application_api: Box<dyn (::apis::ApplicationApi)>,
    application_type_api: Box<dyn (::apis::ApplicationTypeApi)>,
    backup_restore_api: Box<dyn (::apis::BackupRestoreApi)>,
    chaos_api: Box<dyn (::apis::ChaosApi)>,
    cluster_api: Box<dyn (::apis::ClusterApi)>,
    code_package_api: Box<dyn (::apis::CodePackageApi)>,
    compose_deployment_api: Box<dyn (::apis::ComposeDeploymentApi)>,
    events_store_api: Box<dyn (::apis::EventsStoreApi)>,
    faults_api: Box<dyn (::apis::FaultsApi)>,
    image_store_api: Box<dyn (::apis::ImageStoreApi)>,
    infrastructure_api: Box<dyn (::apis::InfrastructureApi)>,
    mesh_applications_api: Box<dyn (::apis::MeshApplicationsApi)>,
    mesh_code_packages_api: Box<dyn (::apis::MeshCodePackagesApi)>,
    mesh_gateways_api: Box<dyn (::apis::MeshGatewaysApi)>,
    mesh_networks_api: Box<dyn (::apis::MeshNetworksApi)>,
    mesh_secret_values_api: Box<dyn (::apis::MeshSecretValuesApi)>,
    mesh_secrets_api: Box<dyn (::apis::MeshSecretsApi)>,
    mesh_service_replicas_api: Box<dyn (::apis::MeshServiceReplicasApi)>,
    mesh_services_api: Box<dyn (::apis::MeshServicesApi)>,
    mesh_volumes_api: Box<dyn (::apis::MeshVolumesApi)>,
    node_api: Box<dyn (::apis::NodeApi)>,
    partition_api: Box<dyn (::apis::PartitionApi)>,
    property_management_api: Box<dyn (::apis::PropertyManagementApi)>,
    repair_management_api: Box<dyn (::apis::RepairManagementApi)>,
    replica_api: Box<dyn (::apis::ReplicaApi)>,
    service_api: Box<dyn (::apis::ServiceApi)>,
    service_package_api: Box<dyn (::apis::ServicePackageApi)>,
    service_type_api: Box<dyn (::apis::ServiceTypeApi)>,
}

impl<C: hyper::client::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            application_api: Box::new(::apis::ApplicationApiClient::new(
                rc.clone(),
            )),
            application_type_api: Box::new(
                ::apis::ApplicationTypeApiClient::new(rc.clone()),
            ),
            backup_restore_api: Box::new(::apis::BackupRestoreApiClient::new(
                rc.clone(),
            )),
            chaos_api: Box::new(::apis::ChaosApiClient::new(rc.clone())),
            cluster_api: Box::new(::apis::ClusterApiClient::new(rc.clone())),
            code_package_api: Box::new(::apis::CodePackageApiClient::new(
                rc.clone(),
            )),
            compose_deployment_api: Box::new(
                ::apis::ComposeDeploymentApiClient::new(rc.clone()),
            ),
            events_store_api: Box::new(::apis::EventsStoreApiClient::new(
                rc.clone(),
            )),
            faults_api: Box::new(::apis::FaultsApiClient::new(rc.clone())),
            image_store_api: Box::new(::apis::ImageStoreApiClient::new(
                rc.clone(),
            )),
            infrastructure_api: Box::new(::apis::InfrastructureApiClient::new(
                rc.clone(),
            )),
            mesh_applications_api: Box::new(
                ::apis::MeshApplicationsApiClient::new(rc.clone()),
            ),
            mesh_code_packages_api: Box::new(
                ::apis::MeshCodePackagesApiClient::new(rc.clone()),
            ),
            mesh_gateways_api: Box::new(::apis::MeshGatewaysApiClient::new(
                rc.clone(),
            )),
            mesh_networks_api: Box::new(::apis::MeshNetworksApiClient::new(
                rc.clone(),
            )),
            mesh_secret_values_api: Box::new(
                ::apis::MeshSecretValuesApiClient::new(rc.clone()),
            ),
            mesh_secrets_api: Box::new(::apis::MeshSecretsApiClient::new(
                rc.clone(),
            )),
            mesh_service_replicas_api: Box::new(
                ::apis::MeshServiceReplicasApiClient::new(rc.clone()),
            ),
            mesh_services_api: Box::new(::apis::MeshServicesApiClient::new(
                rc.clone(),
            )),
            mesh_volumes_api: Box::new(::apis::MeshVolumesApiClient::new(
                rc.clone(),
            )),
            node_api: Box::new(::apis::NodeApiClient::new(rc.clone())),
            partition_api: Box::new(::apis::PartitionApiClient::new(
                rc.clone(),
            )),
            property_management_api: Box::new(
                ::apis::PropertyManagementApiClient::new(rc.clone()),
            ),
            repair_management_api: Box::new(
                ::apis::RepairManagementApiClient::new(rc.clone()),
            ),
            replica_api: Box::new(::apis::ReplicaApiClient::new(rc.clone())),
            service_api: Box::new(::apis::ServiceApiClient::new(rc.clone())),
            service_package_api: Box::new(
                ::apis::ServicePackageApiClient::new(rc.clone()),
            ),
            service_type_api: Box::new(::apis::ServiceTypeApiClient::new(
                rc.clone(),
            )),
        }
    }

    pub fn application_api(&self) -> &dyn (::apis::ApplicationApi) {
        self.application_api.as_ref()
    }

    pub fn application_type_api(&self) -> &dyn (::apis::ApplicationTypeApi) {
        self.application_type_api.as_ref()
    }

    pub fn backup_restore_api(&self) -> &dyn (::apis::BackupRestoreApi) {
        self.backup_restore_api.as_ref()
    }

    pub fn chaos_api(&self) -> &dyn (::apis::ChaosApi) {
        self.chaos_api.as_ref()
    }

    pub fn cluster_api(&self) -> &dyn (::apis::ClusterApi) {
        self.cluster_api.as_ref()
    }

    pub fn code_package_api(&self) -> &dyn (::apis::CodePackageApi) {
        self.code_package_api.as_ref()
    }

    pub fn compose_deployment_api(
        &self,
    ) -> &dyn (::apis::ComposeDeploymentApi) {
        self.compose_deployment_api.as_ref()
    }

    pub fn events_store_api(&self) -> &dyn (::apis::EventsStoreApi) {
        self.events_store_api.as_ref()
    }

    pub fn faults_api(&self) -> &dyn (::apis::FaultsApi) {
        self.faults_api.as_ref()
    }

    pub fn image_store_api(&self) -> &dyn (::apis::ImageStoreApi) {
        self.image_store_api.as_ref()
    }

    pub fn infrastructure_api(&self) -> &dyn (::apis::InfrastructureApi) {
        self.infrastructure_api.as_ref()
    }

    pub fn mesh_applications_api(&self) -> &dyn (::apis::MeshApplicationsApi) {
        self.mesh_applications_api.as_ref()
    }

    pub fn mesh_code_packages_api(&self) -> &dyn (::apis::MeshCodePackagesApi) {
        self.mesh_code_packages_api.as_ref()
    }

    pub fn mesh_gateways_api(&self) -> &dyn (::apis::MeshGatewaysApi) {
        self.mesh_gateways_api.as_ref()
    }

    pub fn mesh_networks_api(&self) -> &dyn (::apis::MeshNetworksApi) {
        self.mesh_networks_api.as_ref()
    }

    pub fn mesh_secret_values_api(&self) -> &dyn (::apis::MeshSecretValuesApi) {
        self.mesh_secret_values_api.as_ref()
    }

    pub fn mesh_secrets_api(&self) -> &dyn (::apis::MeshSecretsApi) {
        self.mesh_secrets_api.as_ref()
    }

    pub fn mesh_service_replicas_api(
        &self,
    ) -> &dyn (::apis::MeshServiceReplicasApi) {
        self.mesh_service_replicas_api.as_ref()
    }

    pub fn mesh_services_api(&self) -> &dyn (::apis::MeshServicesApi) {
        self.mesh_services_api.as_ref()
    }

    pub fn mesh_volumes_api(&self) -> &dyn (::apis::MeshVolumesApi) {
        self.mesh_volumes_api.as_ref()
    }

    pub fn node_api(&self) -> &dyn (::apis::NodeApi) {
        self.node_api.as_ref()
    }

    pub fn partition_api(&self) -> &dyn (::apis::PartitionApi) {
        self.partition_api.as_ref()
    }

    pub fn property_management_api(
        &self,
    ) -> &dyn (::apis::PropertyManagementApi) {
        self.property_management_api.as_ref()
    }

    pub fn repair_management_api(&self) -> &dyn (::apis::RepairManagementApi) {
        self.repair_management_api.as_ref()
    }

    pub fn replica_api(&self) -> &dyn (::apis::ReplicaApi) {
        self.replica_api.as_ref()
    }

    pub fn service_api(&self) -> &dyn (::apis::ServiceApi) {
        self.service_api.as_ref()
    }

    pub fn service_package_api(&self) -> &dyn (::apis::ServicePackageApi) {
        self.service_package_api.as_ref()
    }

    pub fn service_type_api(&self) -> &dyn (::apis::ServiceTypeApi) {
        self.service_type_api.as_ref()
    }
}
