use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T>
where
    T: serde::Deserialize<'de>,
{
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.is_empty() {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => Error::from(e),
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

mod application_api;
pub use self::application_api::{ApplicationApi, ApplicationApiClient};
mod application_resource_api;
pub use self::application_resource_api::{
    ApplicationResourceApi, ApplicationResourceApiClient,
};
mod application_type_api;
pub use self::application_type_api::{
    ApplicationTypeApi, ApplicationTypeApiClient,
};
mod backup_restore_api;
pub use self::backup_restore_api::{BackupRestoreApi, BackupRestoreApiClient};
mod chaos_api;
pub use self::chaos_api::{ChaosApi, ChaosApiClient};
mod cluster_api;
pub use self::cluster_api::{ClusterApi, ClusterApiClient};
mod code_package_api;
pub use self::code_package_api::{CodePackageApi, CodePackageApiClient};
mod compose_deployment_api;
pub use self::compose_deployment_api::{
    ComposeDeploymentApi, ComposeDeploymentApiClient,
};
mod events_store_api;
pub use self::events_store_api::{EventsStoreApi, EventsStoreApiClient};
mod faults_api;
pub use self::faults_api::{FaultsApi, FaultsApiClient};
mod image_store_api;
pub use self::image_store_api::{ImageStoreApi, ImageStoreApiClient};
mod infrastructure_api;
pub use self::infrastructure_api::{
    InfrastructureApi, InfrastructureApiClient,
};
mod node_api;
pub use self::node_api::{NodeApi, NodeApiClient};
mod partition_api;
pub use self::partition_api::{PartitionApi, PartitionApiClient};
mod property_management_api;
pub use self::property_management_api::{
    PropertyManagementApi, PropertyManagementApiClient,
};
mod repair_management_api;
pub use self::repair_management_api::{
    RepairManagementApi, RepairManagementApiClient,
};
mod replica_api;
pub use self::replica_api::{ReplicaApi, ReplicaApiClient};
mod service_api;
pub use self::service_api::{ServiceApi, ServiceApiClient};
mod service_package_api;
pub use self::service_package_api::{
    ServicePackageApi, ServicePackageApiClient,
};
mod service_type_api;
pub use self::service_type_api::{ServiceTypeApi, ServiceTypeApiClient};
mod volume_resource_api;
pub use self::volume_resource_api::{
    VolumeResourceApi, VolumeResourceApiClient,
};

pub mod client;
pub mod configuration;
