/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ResolvedServicePartition : Information about a service partition and its associated endpoints.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolvedServicePartition {
    /// The full name of the service with 'fabric:' URI scheme.
    #[serde(rename = "Name")]
    name: ::models::ServiceName,
    /// A representation of the resolved partition.
    #[serde(rename = "PartitionInformation")]
    partition_information: ::models::PartitionInformation,
    /// List of resolved service endpoints of a service partition.
    #[serde(rename = "Endpoints")]
    endpoints: ::models::ResolvedServiceEndpointList,
    /// The version of this resolved service partition result. This version should be passed in the next time the ResolveService call is made via the PreviousRspVersion query parameter.
    #[serde(rename = "Version")]
    version: String,
}

impl ResolvedServicePartition {
    /// Information about a service partition and its associated endpoints.
    pub fn new(
        name: ::models::ServiceName,
        partition_information: ::models::PartitionInformation,
        endpoints: ::models::ResolvedServiceEndpointList,
        version: String,
    ) -> ResolvedServicePartition {
        ResolvedServicePartition {
            name,
            partition_information,
            endpoints,
            version,
        }
    }

    pub fn set_name(&mut self, name: ::models::ServiceName) {
        self.name = name;
    }

    pub fn with_name(
        mut self,
        name: ::models::ServiceName,
    ) -> ResolvedServicePartition {
        self.name = name;
        self
    }

    pub fn name(&self) -> &::models::ServiceName {
        &self.name
    }

    pub fn set_partition_information(
        &mut self,
        partition_information: ::models::PartitionInformation,
    ) {
        self.partition_information = partition_information;
    }

    pub fn with_partition_information(
        mut self,
        partition_information: ::models::PartitionInformation,
    ) -> ResolvedServicePartition {
        self.partition_information = partition_information;
        self
    }

    pub fn partition_information(&self) -> &::models::PartitionInformation {
        &self.partition_information
    }

    pub fn set_endpoints(
        &mut self,
        endpoints: ::models::ResolvedServiceEndpointList,
    ) {
        self.endpoints = endpoints;
    }

    pub fn with_endpoints(
        mut self,
        endpoints: ::models::ResolvedServiceEndpointList,
    ) -> ResolvedServicePartition {
        self.endpoints = endpoints;
        self
    }

    pub fn endpoints(&self) -> &::models::ResolvedServiceEndpointList {
        &self.endpoints
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn with_version(mut self, version: String) -> ResolvedServicePartition {
        self.version = version;
        self
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}
