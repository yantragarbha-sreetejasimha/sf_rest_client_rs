/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.1
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Epoch : An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Epoch {
    /// The current configuration number of this Epoch. The configuration number is an increasing value that is updated whenever the configuration of this replica set changes.
    #[serde(rename = "ConfigurationVersion")]
    configuration_version: Option<String>,
    /// The current data loss number of this Epoch. The data loss number property is an increasing value which is updated whenever data loss is suspected, as when loss of a quorum of replicas in the replica set that includes the Primary replica.
    #[serde(rename = "DataLossVersion")]
    data_loss_version: Option<String>,
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new()
    }
}

impl Epoch {
    /// An Epoch is a configuration number for the partition as a whole. When the configuration of the replica set changes, for example when the Primary replica changes, the operations that are replicated from the new Primary replica are said to be a new Epoch from the ones which were sent by the old Primary replica.
    pub fn new() -> Epoch {
        Epoch {
            configuration_version: None,
            data_loss_version: None,
        }
    }

    pub fn set_configuration_version(&mut self, configuration_version: String) {
        self.configuration_version = Some(configuration_version);
    }

    pub fn with_configuration_version(
        mut self,
        configuration_version: String,
    ) -> Epoch {
        self.configuration_version = Some(configuration_version);
        self
    }

    pub fn configuration_version(&self) -> Option<&String> {
        self.configuration_version.as_ref()
    }

    pub fn reset_configuration_version(&mut self) {
        self.configuration_version = None;
    }

    pub fn set_data_loss_version(&mut self, data_loss_version: String) {
        self.data_loss_version = Some(data_loss_version);
    }

    pub fn with_data_loss_version(
        mut self,
        data_loss_version: String,
    ) -> Epoch {
        self.data_loss_version = Some(data_loss_version);
        self
    }

    pub fn data_loss_version(&self) -> Option<&String> {
        self.data_loss_version.as_ref()
    }

    pub fn reset_data_loss_version(&mut self) {
        self.data_loss_version = None;
    }
}
